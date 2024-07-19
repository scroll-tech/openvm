use std::{sync::Arc, time::Instant};

use afs_chips::{
    common::page::{merge_pages, Page},
    multitier_page_rw_checker::page_controller::MyLessThanTupleParams,
    page_air::PageAir,
    page_btree::cmp,
    range_gate::RangeCheckerGateChip,
};
use afs_stark_backend::{
    config::{Com, PcsProof, PcsProverData},
    keygen::{
        types::{MultiStarkPartialProvingKey, MultiStarkPartialVerifyingKey},
        MultiStarkKeygenBuilder,
    },
    prover::{
        trace::{ProverTraceData, TraceCommitmentBuilder, TraceCommitter},
        types::Proof,
    },
    rap::AnyRap,
    verifier::VerificationError,
};
use afs_test_utils::engine::StarkEngine;
use itertools::Itertools;
use p3_field::{AbstractField, Field, PrimeField, PrimeField64};
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{Domain, StarkGenericConfig, Val};
use tracing::info_span;

use crate::merge_sort::deterministic_page_sort;

use super::is_full_page_air::IsFullIndexedOutputPageAir;

struct SplitRemainingTraces<F> {
    remaining_trace: RowMajorMatrix<F>,
    input_page_trace: RowMajorMatrix<F>,
    output_cached_trace: Vec<RowMajorMatrix<F>>,
    output_main_trace: Vec<RowMajorMatrix<F>>,
    range_checker_trace: RowMajorMatrix<F>,
}

struct SplitRemainingCommitments<SC: StarkGenericConfig> {
    remaining_commitment: Com<SC>,
    input_page_commitment: Com<SC>,
    output_commitment: Vec<Com<SC>>,
}

pub struct SplitRemainingProverData<SC: StarkGenericConfig> {
    pub remaining_pdata: Option<Arc<ProverTraceData<SC>>>,
    pub input_page_pdata: Option<Arc<ProverTraceData<SC>>>,
    pub output_page_pdata: Vec<Option<Arc<ProverTraceData<SC>>>>,
}

pub struct PageController<SC: StarkGenericConfig>
where
    Val<SC>: AbstractField,
{
    remaining_chip: PageAir,
    input_page_chip: PageAir,
    output_chips: Vec<IsFullIndexedOutputPageAir>,
    // public for testing purposes
    pub output_pis: Option<Vec<(u32, Vec<u32>, Vec<u32>)>>,
    range_checker: Arc<RangeCheckerGateChip>,

    traces: Option<SplitRemainingTraces<Val<SC>>>,
    commitments: Option<SplitRemainingCommitments<SC>>,
}

impl<SC: StarkGenericConfig> PageController<SC> {
    pub fn new(
        page_bus_index: usize,
        idx_len: usize,
        data_len: usize,
        is_less_than_tuple_param: MyLessThanTupleParams,
        range_checker: Arc<RangeCheckerGateChip>,
        k: usize,
    ) -> Self
    where
        Val<SC>: Field,
    {
        Self {
            remaining_chip: PageAir::new(page_bus_index, true, idx_len, data_len),
            input_page_chip: PageAir::new(page_bus_index, true, idx_len, data_len),
            output_chips: vec![
                IsFullIndexedOutputPageAir::new(
                    page_bus_index,
                    is_less_than_tuple_param.clone(),
                    range_checker.air.bus_index,
                    idx_len,
                    data_len,
                );
                k
            ],
            output_pis: None,
            range_checker,
            traces: None,
            commitments: None,
        }
    }

    pub fn load_pages(
        &mut self,
        remaining: &Page,
        input_page: &Page,
        output_pages: &[Page],
        pdata: SplitRemainingProverData<SC>,
        trace_committer: &mut TraceCommitter<SC>,
    ) -> SplitRemainingProverData<SC>
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        let trace_span = info_span!("Trace generation").entered();
        self.range_checker.clear();
        let remaining_trace = remaining.gen_trace();
        let input_page_trace = input_page.gen_trace();
        let output_page_cached_trace = output_pages.iter().map(|p| p.gen_trace()).collect_vec();
        let max_val = (1 << self.output_chips[0].page_chip().idx_limb_bits) - 1;
        let ranges = output_pages
            .iter()
            .map(|p| {
                let mut range_min = vec![max_val; self.remaining_chip.idx_len()];
                let mut range_max = vec![0; self.remaining_chip.idx_len()];
                for row in p.iter() {
                    if row.is_alloc == 1 {
                        if cmp(&range_min, &row.idx) > 0 {
                            range_min.clone_from(&row.idx);
                        }
                        if cmp(&range_max, &row.idx) < 0 {
                            range_max.clone_from(&row.idx);
                        }
                    }
                }
                (range_min, range_max)
            })
            .collect_vec();
        let output_page_main_trace_and_is_full = self
            .output_chips
            .iter()
            .zip(output_pages.iter())
            .zip(ranges.iter())
            .map(|((c, p), r)| {
                c.generate_main_trace::<SC>(p, r.clone(), self.range_checker.clone())
            })
            .collect_vec();
        let output_page_is_full = output_page_main_trace_and_is_full
            .iter()
            .map(|(_, x)| *x)
            .collect_vec();
        let output_page_main_trace = output_page_main_trace_and_is_full
            .into_iter()
            .map(|(x, _)| x)
            .collect_vec();
        let range_checker_trace = self.range_checker.generate_trace();
        let pis = output_page_is_full
            .iter()
            .zip(ranges)
            .map(|(f, r)| (*f as u32, r.0, r.1))
            .collect_vec();
        self.output_pis = Some(pis);
        trace_span.exit();

        let trace_commit_span = info_span!("Trace commitment").entered();
        let remaining_pdata = match pdata.remaining_pdata {
            Some(prover_data) => prover_data,
            None => Arc::new(trace_committer.commit(vec![remaining_trace.clone()])),
        };

        let input_page_pdata = match pdata.input_page_pdata {
            Some(prover_data) => prover_data,
            None => Arc::new(trace_committer.commit(vec![input_page_trace.clone()])),
        };

        let output_page_pdata = pdata
            .output_page_pdata
            .into_iter()
            .zip(output_page_cached_trace.iter())
            .map(|(p, t)| match p {
                Some(_) => p,
                None => Some(Arc::new(trace_committer.commit(vec![t.clone()]))),
            })
            .collect_vec();
        trace_commit_span.exit();

        self.traces = Some(SplitRemainingTraces {
            remaining_trace,
            input_page_trace,
            output_cached_trace: output_page_cached_trace,
            output_main_trace: output_page_main_trace,
            range_checker_trace,
        });

        self.commitments = Some(SplitRemainingCommitments {
            remaining_commitment: remaining_pdata.commit.clone(),
            input_page_commitment: input_page_pdata.commit.clone(),
            output_commitment: output_page_pdata
                .iter()
                .map(|d| d.clone().unwrap().commit.clone())
                .collect_vec(),
        });
        SplitRemainingProverData {
            remaining_pdata: Some(remaining_pdata),
            input_page_pdata: Some(input_page_pdata),
            output_page_pdata,
        }
    }

    pub fn airs(&self) -> Vec<&dyn AnyRap<SC>>
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        let mut airs: Vec<&dyn AnyRap<SC>> = vec![&self.remaining_chip, &self.input_page_chip];
        for c in &self.output_chips {
            airs.push(c)
        }
        airs.push(&self.range_checker.air);
        airs
    }

    /// Sets up keygen with the different trace partitions for the chips
    /// init_chip, final_chip, offline_checker, range_checker, and the
    /// ops_sender, which is passed in
    pub fn set_up_keygen_builder(&self, keygen_builder: &mut MultiStarkKeygenBuilder<SC>)
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        let idx_len = self.input_page_chip.idx_len();
        let remaining_ptr = keygen_builder.add_cached_main_matrix(self.remaining_chip.air_width());
        let input_page_ptr =
            keygen_builder.add_cached_main_matrix(self.input_page_chip.air_width());
        let output_page_cached_ptrs = self
            .output_chips
            .iter()
            .map(|_| keygen_builder.add_cached_main_matrix(self.input_page_chip.air_width()))
            .collect_vec();
        let output_page_main_ptrs = self
            .output_chips
            .iter()
            .map(|c| {
                keygen_builder.add_main_matrix(c.air_width() - self.input_page_chip.air_width())
            })
            .collect_vec();
        keygen_builder.add_partitioned_air(&self.remaining_chip, 0, vec![remaining_ptr]);
        keygen_builder.add_partitioned_air(&self.input_page_chip, 0, vec![input_page_ptr]);
        for ((c, c_ptr), m_ptr) in self
            .output_chips
            .iter()
            .zip(output_page_cached_ptrs.into_iter())
            .zip(output_page_main_ptrs.into_iter())
        {
            keygen_builder.add_partitioned_air(c, 1 + 2 * idx_len, vec![c_ptr, m_ptr]);
        }
        keygen_builder.add_air(&self.range_checker.air, 0)
    }

    /// This function clears the trace_builder, loads in the traces for all involved chips
    /// (including the range_checker and the ops_sender, which is passed in along with its trace),
    /// commits them, and then generates the proof.
    /// cached_traces_prover_data is a vector of ProverTraceData object for the cached pages
    /// (init_page, final_page), which is returned by load_page_and_ops
    #[allow(clippy::too_many_arguments)]
    pub fn prove(
        &self,
        engine: &impl StarkEngine<SC>,
        partial_pk: &MultiStarkPartialProvingKey<SC>,
        trace_builder: &mut TraceCommitmentBuilder<SC>,
        pdata: SplitRemainingProverData<SC>,
    ) -> (Proof<SC>, Vec<Vec<Val<SC>>>)
    where
        Val<SC>: PrimeField + PrimeField64,
        Domain<SC>: Send + Sync,
        SC::Pcs: Sync,
        Domain<SC>: Send + Sync,
        PcsProverData<SC>: Send + Sync,
        Com<SC>: Send + Sync,
        SC::Challenge: Send + Sync,
        PcsProof<SC>: Send + Sync,
    {
        trace_builder.clear();
        let traces = self.traces.as_ref().unwrap();
        trace_builder.load_cached_trace(
            traces.remaining_trace.clone(),
            match Arc::try_unwrap(pdata.remaining_pdata.unwrap()) {
                Ok(data) => data,
                Err(_) => panic!("Prover data should have only one owner"),
            },
        );
        trace_builder.load_cached_trace(
            traces.input_page_trace.clone(),
            match Arc::try_unwrap(pdata.input_page_pdata.unwrap()) {
                Ok(data) => data,
                Err(_) => panic!("Prover data should have only one owner"),
            },
        );
        for (t, p) in traces
            .output_cached_trace
            .iter()
            .zip(pdata.output_page_pdata)
        {
            trace_builder.load_cached_trace(
                t.clone(),
                match Arc::try_unwrap(p.unwrap()) {
                    Ok(data) => data,
                    Err(_) => panic!("Prover data should have only one owner"),
                },
            );
        }
        for t in &traces.output_main_trace {
            trace_builder.load_trace(t.clone());
        }
        trace_builder.load_trace(traces.range_checker_trace.clone());
        tracing::info_span!("Trace commitment").in_scope(|| trace_builder.commit_current());

        let partial_vk = partial_pk.partial_vk();
        let main_trace_data = trace_builder.view(&partial_vk, self.airs());

        let mut pis = vec![vec![]; partial_vk.per_air.len()];
        let output_pis = self.output_pis.as_ref().unwrap();
        for (i, pi) in output_pis.iter().enumerate() {
            pis[2 + i].push(Val::<SC>::from_canonical_u32(pi.0));
            for f in &pi.1 {
                pis[2 + i].push(Val::<SC>::from_canonical_u32(*f));
            }
            for f in &pi.2 {
                pis[2 + i].push(Val::<SC>::from_canonical_u32(*f));
            }
        }
        let prover = engine.prover();
        let mut challenger = engine.new_challenger();
        (
            prover.prove(&mut challenger, partial_pk, main_trace_data, &pis),
            pis,
        )
    }

    /// This function takes a proof (returned by the prove function) and verifies it
    pub fn verify(
        &self,
        engine: &impl StarkEngine<SC>,
        partial_vk: MultiStarkPartialVerifyingKey<SC>,
        proof: Proof<SC>,
        pis: Vec<Vec<Val<SC>>>,
    ) -> Result<(), VerificationError>
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        let verifier = engine.verifier();

        let mut challenger = engine.new_challenger();
        verifier.verify(&mut challenger, &partial_vk, self.airs(), &proof, &pis)
    }

    pub fn generate_output_pages(
        &self,
        init_remaining: &Page,
        input_page: &Page,
        input_page_is_sorted: bool,
    ) -> Vec<Page> {
        let mut input_page = input_page.clone();
        if !input_page_is_sorted {
            input_page.rows.sort()
        };
        let new_page = deterministic_page_sort(&[init_remaining.clone(), input_page.clone()]);
        let output_pages = new_page
            .rows
            .chunks(input_page.height())
            .map(|p| Page { rows: p.to_vec() })
            .collect_vec();
        output_pages
    }
}
