use std::sync::Arc;

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
use p3_field::{AbstractField, Field, PrimeField, PrimeField64};
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{Domain, StarkGenericConfig, Val};
use tracing::info_span;

use super::full_page_air::FullIndexedOutputPageAir;

struct GetTopTraces<F> {
    init_remaining_trace: RowMajorMatrix<F>,
    input_page_trace: RowMajorMatrix<F>,
    final_remaining_trace: RowMajorMatrix<F>,
    top_p_cached_trace: RowMajorMatrix<F>,
    top_p_main_trace: RowMajorMatrix<F>,
    range_checker_trace: RowMajorMatrix<F>,
}

struct GetTopCommitments<SC: StarkGenericConfig> {
    init_remaining_commitment: Com<SC>,
    input_page_commitment: Com<SC>,
    final_remaining_commitment: Com<SC>,
    top_p_commitment: Com<SC>,
}

pub struct GetTopProverData<SC: StarkGenericConfig> {
    pub init_remaining_pdata: Option<Arc<ProverTraceData<SC>>>,
    pub input_page_pdata: Option<Arc<ProverTraceData<SC>>>,
    pub final_remaining_pdata: Option<Arc<ProverTraceData<SC>>>,
    pub top_p_pdata: Option<Arc<ProverTraceData<SC>>>,
}

pub struct PageController<SC: StarkGenericConfig>
where
    Val<SC>: AbstractField,
{
    init_remaining_chip: PageAir,
    input_page_chip: PageAir,
    final_remaining_chip: PageAir,
    top_p_chip: FullIndexedOutputPageAir,
    top_p_pis: Option<(Vec<u32>, Vec<u32>)>,
    range_checker: Arc<RangeCheckerGateChip>,

    traces: Option<GetTopTraces<Val<SC>>>,
    commitments: Option<GetTopCommitments<SC>>,
}

impl<SC: StarkGenericConfig> PageController<SC> {
    pub fn idx_len(&self) -> usize {
        self.init_remaining_chip.idx_len()
    }
    pub fn limb_bits(&self) -> usize {
        self.top_p_chip.limb_bits()
    }
    pub fn new(
        page_bus_index: usize,
        idx_len: usize,
        data_len: usize,
        is_less_than_tuple_param: MyLessThanTupleParams,
        range_checker: Arc<RangeCheckerGateChip>,
    ) -> Self
    where
        Val<SC>: Field,
    {
        Self {
            init_remaining_chip: PageAir::new(page_bus_index, true, idx_len, data_len),
            input_page_chip: PageAir::new(page_bus_index, true, idx_len, data_len),
            final_remaining_chip: PageAir::new(page_bus_index, false, idx_len, data_len),
            top_p_chip: FullIndexedOutputPageAir::new(
                page_bus_index,
                is_less_than_tuple_param.clone(),
                range_checker.air.bus_index,
                idx_len,
                data_len,
            ),
            top_p_pis: None,
            range_checker,
            traces: None,
            commitments: None,
        }
    }

    pub fn load_pages(
        &mut self,
        init_remaining: &Page,
        input_page: &Page,
        final_remaining: &Page,
        top_p: &Page,
        pdata: GetTopProverData<SC>,
        trace_committer: &mut TraceCommitter<SC>,
    ) -> GetTopProverData<SC>
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        let trace_span = info_span!("Trace generation").entered();
        self.range_checker.clear();
        let init_remaining_trace = init_remaining.gen_trace();
        let input_page_trace = input_page.gen_trace();
        let final_remaining_trace = final_remaining.gen_trace();
        let top_p_cached_trace = top_p.gen_trace();
        let mut top_p_min = top_p[0].idx.clone();
        let mut top_p_max = top_p[0].idx.clone();
        for row in top_p.rows.iter().skip(1) {
            if cmp(&top_p_min, &row.idx) > 0 {
                top_p_min.clone_from(&row.idx);
            }
            if cmp(&top_p_max, &row.idx) < 0 {
                top_p_max.clone_from(&row.idx);
            }
        }

        let top_p_main_trace = self.top_p_chip.generate_main_trace::<SC>(
            top_p,
            (top_p_min.clone(), top_p_max.clone()),
            self.range_checker.clone(),
        );
        let range_checker_trace = self.range_checker.generate_trace();

        self.top_p_pis = Some((top_p_min, top_p_max));

        trace_span.exit();

        let trace_commit_span = info_span!("Trace commitment").entered();
        let init_remaining_pdata = match pdata.init_remaining_pdata {
            Some(prover_data) => prover_data,
            None => Arc::new(trace_committer.commit(vec![init_remaining_trace.clone()])),
        };

        let input_page_pdata = match pdata.input_page_pdata {
            Some(prover_data) => prover_data,
            None => Arc::new(trace_committer.commit(vec![input_page_trace.clone()])),
        };

        let final_remaining_pdata = match pdata.final_remaining_pdata {
            Some(prover_data) => prover_data,
            None => Arc::new(trace_committer.commit(vec![final_remaining_trace.clone()])),
        };

        let top_p_pdata = match pdata.top_p_pdata {
            Some(prover_data) => prover_data,
            None => Arc::new(trace_committer.commit(vec![top_p_cached_trace.clone()])),
        };
        trace_commit_span.exit();

        self.traces = Some(GetTopTraces {
            init_remaining_trace,
            input_page_trace,
            final_remaining_trace,
            top_p_cached_trace,
            top_p_main_trace,
            range_checker_trace,
        });

        self.commitments = Some(GetTopCommitments {
            init_remaining_commitment: init_remaining_pdata.commit.clone(),
            input_page_commitment: input_page_pdata.commit.clone(),
            final_remaining_commitment: final_remaining_pdata.commit.clone(),
            top_p_commitment: top_p_pdata.commit.clone(),
        });

        GetTopProverData {
            init_remaining_pdata: Some(init_remaining_pdata),
            input_page_pdata: Some(input_page_pdata),
            final_remaining_pdata: Some(final_remaining_pdata),
            top_p_pdata: Some(top_p_pdata),
        }
    }

    pub fn airs(&self) -> Vec<&dyn AnyRap<SC>>
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        vec![
            &self.init_remaining_chip,
            &self.input_page_chip,
            &self.final_remaining_chip,
            &self.top_p_chip,
            &self.range_checker.air,
        ]
    }

    /// Sets up keygen with the different trace partitions for the chips
    /// init_chip, final_chip, offline_checker, range_checker, and the
    /// ops_sender, which is passed in
    pub fn set_up_keygen_builder(&self, keygen_builder: &mut MultiStarkKeygenBuilder<SC>)
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        let init_remaining_ptr =
            keygen_builder.add_cached_main_matrix(self.init_remaining_chip.air_width());
        let input_page_ptr =
            keygen_builder.add_cached_main_matrix(self.input_page_chip.air_width());
        let final_remaining_ptr =
            keygen_builder.add_cached_main_matrix(self.final_remaining_chip.air_width());
        let top_p_cached_ptr =
            keygen_builder.add_cached_main_matrix(self.input_page_chip.air_width());
        let top_p_main_ptr = keygen_builder
            .add_main_matrix(self.top_p_chip.air_width() - self.input_page_chip.air_width());
        keygen_builder.add_partitioned_air(&self.init_remaining_chip, 0, vec![init_remaining_ptr]);
        keygen_builder.add_partitioned_air(&self.input_page_chip, 0, vec![input_page_ptr]);
        keygen_builder.add_partitioned_air(
            &self.final_remaining_chip,
            0,
            vec![final_remaining_ptr],
        );
        keygen_builder.add_partitioned_air(
            &self.top_p_chip,
            2 * self.idx_len(),
            vec![top_p_cached_ptr, top_p_main_ptr],
        );
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
        pdata: GetTopProverData<SC>,
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
            traces.init_remaining_trace.clone(),
            match Arc::try_unwrap(pdata.init_remaining_pdata.unwrap()) {
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
        trace_builder.load_cached_trace(
            traces.final_remaining_trace.clone(),
            match Arc::try_unwrap(pdata.final_remaining_pdata.unwrap()) {
                Ok(data) => data,
                Err(_) => panic!("Prover data should have only one owner"),
            },
        );
        trace_builder.load_cached_trace(
            traces.top_p_cached_trace.clone(),
            match Arc::try_unwrap(pdata.top_p_pdata.unwrap()) {
                Ok(data) => data,
                Err(_) => panic!("Prover data should have only one owner"),
            },
        );
        trace_builder.load_trace(traces.top_p_main_trace.clone());
        trace_builder.load_trace(traces.range_checker_trace.clone());
        tracing::info_span!("Trace commitment").in_scope(|| trace_builder.commit_current());

        let partial_vk = partial_pk.partial_vk();
        let main_trace_data = trace_builder.view(&partial_vk, self.airs());

        let mut pis = vec![vec![]; partial_vk.per_air.len()];
        let top_p_pis = self.top_p_pis.as_ref().unwrap();
        for f in &top_p_pis.0 {
            pis[3].push(Val::<SC>::from_canonical_u32(*f));
        }
        for f in &top_p_pis.1 {
            pis[3].push(Val::<SC>::from_canonical_u32(*f));
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

    pub fn generate_output_pages(&self, init_remaining: &Page, input_page: &Page) -> (Page, Page) {
        let mut new_page = merge_pages(&[init_remaining.clone(), input_page.clone()]);
        let page_height = input_page.height();
        new_page.rows.sort();
        let final_remaining = Page {
            rows: new_page.rows.split_off(page_height),
        };
        (final_remaining, new_page)
    }
}
