use std::sync::Arc;

use afs_chips::{common::page::Page, page_air::PageAir};
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
use p3_field::{AbstractField, Field, PrimeField};
use p3_matrix::dense::{DenseMatrix, RowMajorMatrix};
use p3_uni_stark::{Domain, StarkGenericConfig, Val};
use tracing::info_span;

struct EqualPageContentTraces<F> {
    init_page_traces: Vec<RowMajorMatrix<F>>,
    final_page_traces: Vec<RowMajorMatrix<F>>,
}

struct EqualPageContentCommitments<SC: StarkGenericConfig> {
    init_page_commitments: Vec<Com<SC>>,
    final_page_commitments: Vec<Com<SC>>,
}

pub struct PageController<SC: StarkGenericConfig>
where
    Val<SC>: AbstractField,
{
    init_page_chips: Vec<PageAir>,
    init_page_heights: Vec<usize>,
    final_page_chips: Vec<PageAir>,
    final_page_heights: Vec<usize>,

    traces: Option<EqualPageContentTraces<Val<SC>>>,
    commitments: Option<EqualPageContentCommitments<SC>>,
}

impl<SC: StarkGenericConfig> PageController<SC> {
    pub fn new(
        page_bus_index: usize,
        idx_len: usize,
        data_len: usize,
        init_page_heights: Vec<usize>,
        final_page_heights: Vec<usize>,
    ) -> Self
    where
        Val<SC>: Field,
    {
        Self {
            init_page_chips: vec![
                PageAir::new(page_bus_index, true, idx_len, data_len);
                init_page_heights.len()
            ],
            init_page_heights,
            final_page_chips: vec![
                PageAir::new(page_bus_index, false, idx_len, data_len);
                final_page_heights.len()
            ],
            final_page_heights,
            traces: None,
            commitments: None,
        }
    }

    pub fn load_pages(
        &mut self,
        init_pages: &[Page],
        final_pages: &[Page],
        init_page_pdata: Option<Vec<Arc<ProverTraceData<SC>>>>,
        final_page_pdata: Option<Vec<Arc<ProverTraceData<SC>>>>,
        trace_committer: &mut TraceCommitter<SC>,
    ) -> (Vec<Arc<ProverTraceData<SC>>>, Vec<Arc<ProverTraceData<SC>>>)
    where
        Val<SC>: PrimeField,
    {
        let trace_span = info_span!("Trace generation").entered();

        let init_page_traces = init_pages
            .iter()
            .map(|p| self.gen_page_trace(&p))
            .collect_vec();

        let final_page_traces = final_pages
            .iter()
            .map(|p| self.gen_page_trace(&p))
            .collect_vec();

        trace_span.exit();

        let trace_commit_span = info_span!("Trace commitment").entered();
        let init_page_pdata = match init_page_pdata {
            Some(prover_data) => prover_data,
            None => init_page_traces
                .iter()
                .map(|t| Arc::new(trace_committer.commit(vec![t.clone()])))
                .collect_vec(),
        };

        let final_page_pdata = match final_page_pdata {
            Some(prover_data) => prover_data,
            None => final_page_traces
                .iter()
                .map(|t| Arc::new(trace_committer.commit(vec![t.clone()])))
                .collect_vec(),
        };
        trace_commit_span.exit();

        self.traces = Some(EqualPageContentTraces {
            init_page_traces,
            final_page_traces,
        });

        let init_page_commitments = init_page_pdata
            .iter()
            .map(|p| p.commit.clone())
            .collect_vec();
        let final_page_commitments = final_page_pdata
            .iter()
            .map(|p| p.commit.clone())
            .collect_vec();

        self.commitments = Some(EqualPageContentCommitments {
            init_page_commitments,
            final_page_commitments,
        });

        (init_page_pdata, final_page_pdata)
    }

    /// Sets up keygen with the different trace partitions for the chips
    /// init_chip, final_chip, offline_checker, range_checker, and the
    /// ops_sender, which is passed in
    pub fn set_up_keygen_builder(&self, keygen_builder: &mut MultiStarkKeygenBuilder<SC>)
    where
        Val<SC>: PrimeField,
    {
        let init_page_ptrs = self
            .init_page_chips
            .iter()
            .map(|c| keygen_builder.add_cached_main_matrix(c.air_width()))
            .collect_vec();
        let final_page_ptrs = self
            .final_page_chips
            .iter()
            .map(|c| keygen_builder.add_cached_main_matrix(c.air_width()))
            .collect_vec();

        for ((p, c), h) in init_page_ptrs
            .iter()
            .zip(self.init_page_chips.iter())
            .zip(self.init_page_heights.iter())
        {
            keygen_builder.add_partitioned_air(c, *h, 0, vec![*p])
        }

        for ((p, c), h) in final_page_ptrs
            .iter()
            .zip(self.final_page_chips.iter())
            .zip(self.final_page_heights.iter())
        {
            keygen_builder.add_partitioned_air(c, *h, 0, vec![*p])
        }
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
        init_page_pdata: Vec<Arc<ProverTraceData<SC>>>,
        final_page_pdata: Vec<Arc<ProverTraceData<SC>>>,
    ) -> Proof<SC>
    where
        Val<SC>: PrimeField,
        Domain<SC>: Send + Sync,
        SC::Pcs: Sync,
        Domain<SC>: Send + Sync,
        PcsProverData<SC>: Send + Sync,
        Com<SC>: Send + Sync,
        SC::Challenge: Send + Sync,
        PcsProof<SC>: Send + Sync,
    {
        let traces = self.traces.as_ref().unwrap();

        trace_builder.clear();
        for (t, p) in traces
            .init_page_traces
            .iter()
            .zip(init_page_pdata.into_iter())
        {
            trace_builder.load_cached_trace(
                t.clone(),
                match Arc::try_unwrap(p) {
                    Ok(data) => data,
                    Err(_) => panic!("Prover data should have only one owner"),
                },
            );
        }
        for (t, p) in traces
            .final_page_traces
            .iter()
            .zip(final_page_pdata.into_iter())
        {
            trace_builder.load_cached_trace(
                t.clone(),
                match Arc::try_unwrap(p) {
                    Ok(data) => data,
                    Err(_) => panic!("Prover data should have only one owner"),
                },
            );
        }
        // tracing::info_span!("Trace commitment").in_scope(|| trace_builder.commit_current());

        let partial_vk = partial_pk.partial_vk();

        let mut airs: Vec<&dyn AnyRap<SC>> = vec![];
        for c in &self.init_page_chips {
            airs.push(c);
        }
        for c in &self.final_page_chips {
            airs.push(c);
        }

        let main_trace_data = trace_builder.view(&partial_vk, airs);

        let pis = vec![vec![]; partial_vk.per_air.len()];
        let prover = engine.prover();
        let mut challenger = engine.new_challenger();
        prover.prove(&mut challenger, partial_pk, main_trace_data, &pis)
    }

    /// This function takes a proof (returned by the prove function) and verifies it
    pub fn verify(
        &self,
        engine: &impl StarkEngine<SC>,
        partial_vk: MultiStarkPartialVerifyingKey<SC>,
        proof: Proof<SC>,
    ) -> Result<(), VerificationError>
    where
        Val<SC>: PrimeField,
    {
        let verifier = engine.verifier();

        let pis = vec![vec![]; partial_vk.per_air.len()];

        let mut challenger = engine.new_challenger();
        let mut airs: Vec<&dyn AnyRap<SC>> = vec![];
        for c in &self.init_page_chips {
            airs.push(c);
        }
        for c in &self.final_page_chips {
            airs.push(c);
        }
        verifier.verify(&mut challenger, partial_vk, airs, proof, &pis)
    }

    fn gen_page_trace(&self, page: &Page) -> DenseMatrix<Val<SC>>
    where
        Val<SC>: PrimeField,
    {
        page.gen_trace()
    }
}
