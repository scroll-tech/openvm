use std::sync::Arc;

use afs_chips::common::{
    page::{merge_pages, Page},
    page_cols::PageCols,
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
use p3_uni_stark::{Domain, StarkGenericConfig, Val};

use crate::equal_page_content::page_controller::PageController as EPCPageController;
pub struct PageController<SC: StarkGenericConfig>
where
    Val<SC>: AbstractField,
{
    epc_page_controller: EPCPageController<SC>,
}

impl<SC: StarkGenericConfig> PageController<SC> {
    pub fn new(page_bus_index: usize, idx_len: usize, data_len: usize, k: usize) -> Self
    where
        Val<SC>: Field,
    {
        Self {
            epc_page_controller: EPCPageController::new(
                page_bus_index,
                idx_len,
                data_len,
                k - 1,
                1,
            ),
        }
    }

    pub fn load_pages(
        &mut self,
        pages: &[Page],
        remaining: Page,
        page_pdata: Vec<Option<Arc<ProverTraceData<SC>>>>,
        remaining_page_pdata: Vec<Option<Arc<ProverTraceData<SC>>>>,
        trace_committer: &mut TraceCommitter<SC>,
    ) -> (
        Vec<Option<Arc<ProverTraceData<SC>>>>,
        Vec<Option<Arc<ProverTraceData<SC>>>>,
    )
    where
        Val<SC>: PrimeField,
    {
        self.epc_page_controller.load_pages(
            pages,
            &[remaining],
            page_pdata,
            remaining_page_pdata,
            trace_committer,
        )
    }

    pub fn airs(&self) -> Vec<&dyn AnyRap<SC>>
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        self.epc_page_controller.airs()
    }

    /// Sets up keygen with the different trace partitions for the chips
    /// init_chip, final_chip, offline_checker, range_checker, and the
    /// ops_sender, which is passed in
    pub fn set_up_keygen_builder(&self, keygen_builder: &mut MultiStarkKeygenBuilder<SC>)
    where
        Val<SC>: PrimeField,
    {
        self.epc_page_controller
            .set_up_keygen_builder(keygen_builder)
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
        final_page_pdata: Arc<ProverTraceData<SC>>,
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
        self.epc_page_controller.prove(
            engine,
            partial_pk,
            trace_builder,
            init_page_pdata,
            vec![final_page_pdata],
        )
    }

    /// This function takes a proof (returned by the prove function) and verifies it
    pub fn verify(
        &self,
        engine: &impl StarkEngine<SC>,
        partial_vk: MultiStarkPartialVerifyingKey<SC>,
        proof: Proof<SC>,
    ) -> Result<(), VerificationError>
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        self.epc_page_controller.verify(engine, partial_vk, proof)
    }

    pub fn gen_remaining(&self, pages: &[Page]) -> Page
    where
        Val<SC>: PrimeField,
    {
        let mut rem_pages = merge_pages(pages);
        let len = rem_pages.rows.len();
        let idx_len = pages[0][0].idx.len();
        let data_len = pages[0][0].data.len();
        rem_pages.resize(
            len.next_power_of_two(),
            PageCols::<u32>::blank(idx_len, data_len),
        );
        rem_pages
    }
}
