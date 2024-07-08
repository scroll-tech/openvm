use std::collections::HashSet;
use std::sync::Arc;

use afs_chips::{
    common::page::{merge, merge_pages, Page},
    page_air::PageAir,
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
use p3_field::{AbstractField, Field, PrimeField};
use p3_matrix::dense::{DenseMatrix, RowMajorMatrix};
use p3_uni_stark::{Domain, StarkGenericConfig, Val};
use tracing::info_span;

struct InitRemainingTraces<F> {
    page_traces: Vec<RowMajorMatrix<F>>,
    remaining_trace: RowMajorMatrix<F>,
}

struct InitRemainingCommitments<SC: StarkGenericConfig> {
    page_commitments: Vec<Com<SC>>,
    remaining_commitment: Com<SC>,
}

pub struct PageController<SC: StarkGenericConfig>
where
    Val<SC>: AbstractField,
{
    page_chips: Vec<PageAir>,
    remaining_chip: PageAir,

    traces: Option<InitRemainingTraces<Val<SC>>>,
    commitments: Option<InitRemainingCommitments<SC>>,
}

impl<SC: StarkGenericConfig> PageController<SC> {
    pub fn new(page_bus_index: usize, idx_len: usize, data_len: usize, k: usize) -> Self
    where
        Val<SC>: Field,
    {
        Self {
            page_chips: vec![PageAir::new(page_bus_index, true, idx_len, data_len); k - 1],
            remaining_chip: PageAir::new(page_bus_index, false, idx_len, data_len),
            traces: None,
            commitments: None,
        }
    }

    pub fn load_page_and_ops(
        &mut self,
        pages: &Vec<Page>,
        page_pdata: Option<Vec<Arc<ProverTraceData<SC>>>>,
        trace_committer: &mut TraceCommitter<SC>,
    ) -> (Arc<ProverTraceData<SC>>, Arc<ProverTraceData<SC>>)
    where
        Val<SC>: PrimeField,
    {
        let trace_span = info_span!("Trace generation").entered();

        let init_page_traces = pages.iter().map(|p| self.gen_page_trace(&p)).collect_vec();

        let final_page_trace = self.gen_page_trace(&page);
        let final_page_aux_trace = self.final_chip.gen_aux_trace::<SC>(
            &page,
            self.range_checker.clone(),
            final_write_indices,
        );
        trace_span.exit();

        let trace_commit_span = info_span!("Trace commitment").entered();
        let init_page_pdata = match init_page_pdata {
            Some(prover_data) => prover_data,
            None => Arc::new(trace_committer.commit(vec![init_page_trace.clone()])),
        };

        let final_page_pdata = match final_page_pdata {
            Some(prover_data) => prover_data,
            None => Arc::new(trace_committer.commit(vec![final_page_trace.clone()])),
        };
        trace_commit_span.exit();

        self.traces = Some(PageRWTraces {
            init_page_trace,
            final_page_trace,
            final_page_aux_trace,
            offline_checker_trace,
        });

        self.page_commitments = Some(PageCommitments {
            init_page_commitment: init_page_pdata.commit.clone(),
            final_page_commitment: final_page_pdata.commit.clone(),
        });

        (init_page_pdata, final_page_pdata)
    }

    /// Sets up keygen with the different trace partitions for the chips
    /// init_chip, final_chip, offline_checker, range_checker, and the
    /// ops_sender, which is passed in
    pub fn set_up_keygen_builder(
        &self,
        keygen_builder: &mut MultiStarkKeygenBuilder<SC>,
        page_height: usize,
        offline_checker_trace_degree: usize,
        ops_sender: &dyn AnyRap<SC>,
        ops_sender_trace_degree: usize,
    ) where
        Val<SC>: PrimeField,
    {
        let init_page_ptr = keygen_builder.add_cached_main_matrix(self.init_chip.air_width());
        let final_page_ptr = keygen_builder.add_cached_main_matrix(self.final_chip.page_width());
        let final_page_aux_ptr = keygen_builder.add_main_matrix(self.final_chip.aux_width());

        keygen_builder.add_partitioned_air(&self.init_chip, page_height, 0, vec![init_page_ptr]);

        keygen_builder.add_partitioned_air(
            &self.final_chip,
            page_height,
            0,
            vec![final_page_ptr, final_page_aux_ptr],
        );

        keygen_builder.add_air(&self.offline_checker, offline_checker_trace_degree, 0);

        keygen_builder.add_air(
            &self.range_checker.air,
            self.range_checker.range_max() as usize,
            0,
        );

        keygen_builder.add_air(ops_sender, ops_sender_trace_degree, 0);
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
        init_page_pdata: Arc<ProverTraceData<SC>>,
        final_page_pdata: Arc<ProverTraceData<SC>>,
        ops_sender: &dyn AnyRap<SC>,
        ops_sender_trace: DenseMatrix<Val<SC>>,
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

        trace_builder.load_cached_trace(
            traces.init_page_trace.clone(),
            match Arc::try_unwrap(init_page_pdata) {
                Ok(data) => data,
                Err(_) => panic!("Prover data should have only one owner"),
            },
        );
        trace_builder.load_cached_trace(
            traces.final_page_trace.clone(),
            match Arc::try_unwrap(final_page_pdata) {
                Ok(data) => data,
                Err(_) => panic!("Prover data should have only one owner"),
            },
        );
        trace_builder.load_trace(traces.final_page_aux_trace.clone());
        trace_builder.load_trace(traces.offline_checker_trace.clone());
        trace_builder.load_trace(self.range_checker.generate_trace());
        trace_builder.load_trace(ops_sender_trace);

        tracing::info_span!("Trace commitment").in_scope(|| trace_builder.commit_current());

        let partial_vk = partial_pk.partial_vk();

        let main_trace_data = trace_builder.view(
            &partial_vk,
            vec![
                &self.init_chip,
                &self.final_chip,
                &self.offline_checker,
                &self.range_checker.air,
                ops_sender,
            ],
        );

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
        ops_sender: &dyn AnyRap<SC>,
    ) -> Result<(), VerificationError>
    where
        Val<SC>: PrimeField,
    {
        let verifier = engine.verifier();

        let pis = vec![vec![]; partial_vk.per_air.len()];

        let mut challenger = engine.new_challenger();
        verifier.verify(
            &mut challenger,
            partial_vk,
            vec![
                &self.init_chip,
                &self.final_chip,
                &self.offline_checker,
                &self.range_checker.air,
                ops_sender,
            ],
            proof,
            &pis,
        )
    }

    fn gen_page_trace(&self, page: &Page) -> DenseMatrix<Val<SC>>
    where
        Val<SC>: PrimeField,
    {
        page.gen_trace()
    }

    fn gen_rem_page_trace(&self, pages: &Vec<Page>) -> DenseMatrix<Val<SC>>
    where
        Val<SC>: PrimeField,
    {
        merge_pages(pages).gen_trace()
    }

    fn gen_ops_trace(
        &self,
        page: &mut Page,
        ops: &[Operation],
        range_checker: Arc<RangeCheckerGateChip>,
        trace_degree: usize,
    ) -> RowMajorMatrix<Val<SC>>
    where
        Val<SC>: PrimeField,
    {
        self.offline_checker
            .generate_trace::<SC>(page, ops.to_owned(), range_checker, trace_degree)
    }
}
