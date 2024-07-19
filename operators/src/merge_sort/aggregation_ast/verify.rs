use std::sync::Arc;

use afs_chips::common::page::Page;
use afs_chips::multitier_page_rw_checker::page_controller::MyLessThanTupleParams;
use afs_chips::page_btree::cmp;
use afs_chips::range_gate::RangeCheckerGateChip;
use afs_stark_backend::config::Com;
use afs_stark_backend::keygen::types::{
    MultiStarkPartialProvingKey, MultiStarkPartialVerifyingKey,
};
use afs_stark_backend::keygen::MultiStarkKeygenBuilder;
use afs_stark_backend::prover::trace::TraceCommitter;
use afs_stark_backend::prover::types::Proof;
use afs_stark_backend::rap::AnyRap;
use afs_test_utils::engine::StarkEngine;
use itertools::Itertools;
use p3_field::{PrimeField, PrimeField64};
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::merge_sort::get_top_p::page_controller::PageController as GTPPageController;
use crate::merge_sort::init_remaining::PageController as IRPageController;
use crate::merge_sort::split_remaining::page_controller::PageController as SRPageController;

use super::prove::{
    deterministic_table_sort, FinishMerge, InitRemainingGTP, MergeSort, MergeSortMain, PreMerge,
};
use super::DataFrameRow;

pub struct PageControllers<SC: StarkGenericConfig, const COMMITMENT_LEN: usize> {
    pub init_remaining: IRPageController<SC>,
    pub get_top_p: GTPPageController<SC>,
    pub split_remaining: SRPageController<SC>,
    pub params: PageControllerParams,
}

pub struct PageControllerParams {
    pub page_height: usize,
    pub idx_len: usize,
    pub data_len: usize,
    pub limb_bits: usize,
    pub k: usize,
    pub blank_commit: Vec<u32>,
}

pub struct VerifyingKeys<SC: StarkGenericConfig> {
    pub init_remaining: MultiStarkPartialVerifyingKey<SC>,
    pub get_top_p: MultiStarkPartialVerifyingKey<SC>,
    pub split_remaining: MultiStarkPartialVerifyingKey<SC>,
}

pub struct Airs<'a, SC: StarkGenericConfig> {
    pub init_remaining: Vec<&'a dyn AnyRap<SC>>,
    pub get_top_p: Vec<&'a dyn AnyRap<SC>>,
    pub split_remaining: Vec<&'a dyn AnyRap<SC>>,
}

pub struct ProvingKeys<SC: StarkGenericConfig> {
    pub init_remaining: MultiStarkPartialProvingKey<SC>,
    pub get_top_p: MultiStarkPartialProvingKey<SC>,
    pub split_remaining: MultiStarkPartialProvingKey<SC>,
}

impl<SC: StarkGenericConfig, const COMMITMENT_LEN: usize> PageControllers<SC, COMMITMENT_LEN> {
    pub fn new(
        k: usize,
        page_height: usize,
        idx_len: usize,
        data_len: usize,
        is_less_than_tuple_param: MyLessThanTupleParams,
        trace_committer: &TraceCommitter<SC>,
    ) -> Self
    where
        Val<SC>: PrimeField + PrimeField64,
        Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
    {
        let page_bus_index = 0;
        let lt_bus_index = 1;
        let ir_page_controller = IRPageController::<SC>::new(page_bus_index, idx_len, data_len, k);
        let gtp_page_controller = {
            let range_checker = Arc::new(RangeCheckerGateChip::new(
                lt_bus_index,
                1 << is_less_than_tuple_param.decomp,
            ));
            GTPPageController::<SC>::new(
                page_bus_index,
                idx_len,
                data_len,
                is_less_than_tuple_param.clone(),
                range_checker,
            )
        };
        let sr_page_controller = {
            let range_checker = Arc::new(RangeCheckerGateChip::new(
                lt_bus_index,
                1 << is_less_than_tuple_param.decomp,
            ));
            SRPageController::<SC>::new(
                page_bus_index,
                idx_len,
                data_len,
                is_less_than_tuple_param.clone(),
                range_checker,
                k,
            )
        };
        let blank_page = Page::blank(idx_len, data_len, page_height);
        let blank_commit: [Val<SC>; COMMITMENT_LEN] = trace_committer
            .commit(vec![blank_page.gen_trace()])
            .commit
            .into();
        let blank_commit = blank_commit
            .into_iter()
            .map(|f| f.as_canonical_u64() as u32)
            .collect_vec();
        let params = PageControllerParams {
            page_height,
            idx_len,
            data_len,
            limb_bits: is_less_than_tuple_param.limb_bits,
            k,
            blank_commit,
        };
        Self {
            init_remaining: ir_page_controller,
            get_top_p: gtp_page_controller,
            split_remaining: sr_page_controller,
            params,
        }
    }
}

impl<SC: StarkGenericConfig> ProvingKeys<SC> {
    pub fn new<const COMMITMENT_LEN: usize>(
        engine: &dyn StarkEngine<SC>,
        page_controllers: &PageControllers<SC, COMMITMENT_LEN>,
    ) -> Self
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        let ir_pk = {
            let mut keygen_builder = MultiStarkKeygenBuilder::new(engine.config());

            page_controllers
                .init_remaining
                .set_up_keygen_builder(&mut keygen_builder);

            keygen_builder.generate_partial_pk()
        };
        let gtp_pk = {
            let mut keygen_builder = MultiStarkKeygenBuilder::new(engine.config());

            page_controllers
                .get_top_p
                .set_up_keygen_builder(&mut keygen_builder);

            keygen_builder.generate_partial_pk()
        };
        let sr_pk = {
            let mut keygen_builder = MultiStarkKeygenBuilder::new(engine.config());

            page_controllers
                .split_remaining
                .set_up_keygen_builder(&mut keygen_builder);

            keygen_builder.generate_partial_pk()
        };
        Self {
            init_remaining: ir_pk,
            get_top_p: gtp_pk,
            split_remaining: sr_pk,
        }
    }
}

impl<SC: StarkGenericConfig> VerifyingKeys<SC> {
    pub fn new<const COMMITMENT_LEN: usize>(
        engine: &dyn StarkEngine<SC>,
        page_controllers: &PageControllers<SC, COMMITMENT_LEN>,
    ) -> Self
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        let pks = ProvingKeys::new(engine, page_controllers);
        Self {
            init_remaining: pks.init_remaining.partial_vk(),
            get_top_p: pks.get_top_p.partial_vk(),
            split_remaining: pks.split_remaining.partial_vk(),
        }
    }
}

impl<'a, SC: StarkGenericConfig> Airs<'a, SC> {
    pub fn new<const COMMITMENT_LEN: usize>(
        page_controllers: &'a PageControllers<SC, COMMITMENT_LEN>,
    ) -> Self
    where
        Val<SC>: PrimeField + PrimeField64,
    {
        let ir_airs = page_controllers.init_remaining.airs();
        let gtp_airs = page_controllers.get_top_p.airs();
        let sr_airs = page_controllers.split_remaining.airs();
        Self {
            init_remaining: ir_airs.clone(),
            get_top_p: gtp_airs.clone(),
            split_remaining: sr_airs.clone(),
        }
    }
}

pub trait VMVerifiable<'a, SC: StarkGenericConfig, const COMMITMENT_LEN: usize>
where
    Val<SC>: PrimeField + PrimeField64,
{
    type Input;
    type Output;
    type Params;
    type VerifyingKeys;
    type PageControllers;
    fn verify(
        &self,
        engine: &dyn StarkEngine<SC>,
        page_controllers: &Self::PageControllers,
        verifying_keys: &Self::VerifyingKeys,
        input: Self::Input,
    ) -> Self::Output;
}

pub struct InitRemainingGTPInput {
    pub buf: Vec<DataFrameRow<u32>>,
    pub m: usize,
}

pub struct InitRemainingGTPOutput {
    pub remaining: Vec<u32>,
    pub indexed_table: Vec<DataFrameRow<u32>>,
}

impl<'a, SC: StarkGenericConfig, const COMMITMENT_LEN: usize> VMVerifiable<'a, SC, COMMITMENT_LEN>
    for InitRemainingGTP<SC>
where
    Val<SC>: PrimeField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    type Input = InitRemainingGTPInput;

    type Output = InitRemainingGTPOutput;

    type VerifyingKeys = VerifyingKeys<SC>;

    type Params = PageControllerParams;

    type PageControllers = PageControllers<SC, COMMITMENT_LEN>;

    fn verify(
        &self,
        engine: &dyn StarkEngine<SC>,
        page_controllers: &Self::PageControllers,
        vks: &Self::VerifyingKeys,
        input: Self::Input,
    ) -> Self::Output {
        let k = page_controllers.params.k;
        let idx_len = page_controllers.params.idx_len;
        match self {
            InitRemainingGTP::Init(verifier_inputs) => {
                let init_remaining_vk = &vks.init_remaining;
                for i in 0..k - 1 {
                    let input_com =
                        get_commitment_from_proof(&verifier_inputs.proof, &vks.init_remaining, i);
                    let buf_com = &input.buf[i].commitment;
                    for (b, c) in buf_com.iter().zip(input_com) {
                        assert_eq!(*b, c);
                    }
                }
                verifier_inputs.verify(
                    engine,
                    page_controllers.init_remaining.airs(),
                    init_remaining_vk,
                );
                let remaining_com =
                    get_commitment_from_proof(&verifier_inputs.proof, &vks.init_remaining, k - 1);
                InitRemainingGTPOutput {
                    remaining: remaining_com,
                    indexed_table: vec![],
                }
            }
            InitRemainingGTP::InitGTP { head, tail } => {
                let input_idx = k - 2 + input.m;
                let buf_com = &input.buf[input_idx].commitment;
                let input_com = get_commitment_from_proof(&tail.proof, &vks.get_top_p, 1);
                for (b, i) in buf_com.iter().zip(input_com) {
                    assert_eq!(*b, i);
                }
                let InitRemainingGTPOutput {
                    remaining,
                    mut indexed_table,
                } = head.verify(
                    engine,
                    page_controllers,
                    vks,
                    Self::Input {
                        buf: input.buf,
                        m: input.m - 1,
                    },
                );
                let init_remaining_com = get_commitment_from_proof(&tail.proof, &vks.get_top_p, 0); // check consistency of init_remaining
                for (r, i) in remaining.iter().zip(init_remaining_com) {
                    assert_eq!(*r, i);
                }
                let top_p_start = tail.pis[3][0..idx_len]
                    .iter()
                    .map(|f| f.as_canonical_u64() as u32)
                    .collect_vec();
                let top_p_end = tail.pis[3][idx_len..2 * idx_len]
                    .iter()
                    .map(|f| f.as_canonical_u64() as u32)
                    .collect_vec();
                let top_p_com = get_commitment_from_proof(&tail.proof, &vks.get_top_p, 3);
                indexed_table.push(DataFrameRow {
                    start: top_p_start,
                    end: top_p_end,
                    commitment: top_p_com,
                });
                tail.verify(engine, page_controllers.get_top_p.airs(), &vks.get_top_p);
                let remaining_com = get_commitment_from_proof(&tail.proof, &vks.get_top_p, 2);
                InitRemainingGTPOutput {
                    remaining: remaining_com,
                    indexed_table,
                }
            }
        }
    }
}

impl<'a, SC: StarkGenericConfig, const COMMITMENT_LEN: usize> VMVerifiable<'a, SC, COMMITMENT_LEN>
    for FinishMerge<SC>
where
    Val<SC>: PrimeField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    type Input = InitRemainingGTPInput;

    type Output = Vec<DataFrameRow<u32>>;

    type VerifyingKeys = VerifyingKeys<SC>;

    type Params = PageControllerParams;

    type PageControllers = PageControllers<SC, COMMITMENT_LEN>;

    fn verify(
        &self,
        engine: &dyn StarkEngine<SC>,
        page_controllers: &Self::PageControllers,
        vks: &Self::VerifyingKeys,
        input: Self::Input,
    ) -> Self::Output {
        let k = page_controllers.params.k;
        let idx_len = page_controllers.params.idx_len;
        let input_com = get_commitment_from_proof(&self.split.proof, &vks.split_remaining, 1);
        let buf_com = &input.buf.last().unwrap().commitment;
        for (b, i) in buf_com.iter().zip(input_com) {
            assert_eq!(*b, i);
        }
        let InitRemainingGTPOutput {
            remaining,
            mut indexed_table,
        } = self.init_gtp.verify(
            engine,
            page_controllers,
            vks,
            Self::Input {
                buf: input.buf,
                m: input.m,
            },
        );
        let remaining_com = get_commitment_from_proof(&self.split.proof, &vks.split_remaining, 0);
        for (r, rc) in remaining.into_iter().zip(remaining_com) {
            assert_eq!(r, rc);
        }
        let mut check_blank = false;
        for i in 0..k {
            let is_full = self.split.pis[2 + i][0].as_canonical_u64();
            let start = self.split.pis[2 + i][1..1 + idx_len]
                .iter()
                .map(|f| f.as_canonical_u64() as u32)
                .collect_vec();
            let end = self.split.pis[2 + i][1 + idx_len..1 + 2 * idx_len]
                .iter()
                .map(|f| f.as_canonical_u64() as u32)
                .collect_vec();
            let com = get_commitment_from_proof(&self.split.proof, &vks.split_remaining, 2 + i);
            if !check_blank {
                indexed_table.push(DataFrameRow {
                    start,
                    end,
                    commitment: com,
                });
                if is_full == 0 {
                    check_blank = true;
                }
            } else {
                let blank_commit = &page_controllers.params.blank_commit;
                for (b, c) in blank_commit.iter().zip(com) {
                    assert_eq!(*b, c);
                }
            }
            for i in 0..indexed_table.len() - 1 {
                assert!(cmp(&indexed_table[i].end, &indexed_table[i + 1].start) <= 0)
            }
        }
        indexed_table
    }
}

impl<'a, SC: StarkGenericConfig, const COMMITMENT_LEN: usize> VMVerifiable<'a, SC, COMMITMENT_LEN>
    for PreMerge<SC>
where
    Val<SC>: PrimeField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    type Input = &'a [DataFrameRow<u32>];

    type Output = Vec<DataFrameRow<u32>>;

    type VerifyingKeys = VerifyingKeys<SC>;

    type Params = PageControllerParams;

    type PageControllers = PageControllers<SC, COMMITMENT_LEN>;

    fn verify(
        &self,
        engine: &dyn StarkEngine<SC>,
        page_controllers: &Self::PageControllers,
        vks: &Self::VerifyingKeys,
        input: Self::Input,
    ) -> Self::Output {
        let k = page_controllers.params.k;
        let idx_len = page_controllers.params.idx_len;
        let blank_commit = page_controllers.params.blank_commit.clone();
        let parts = self.merge_sorts.len();
        let len = input.len();
        let mut tables = vec![];
        for (i, m_sort) in self.merge_sorts.iter().enumerate() {
            tables.push(m_sort.verify(
                engine,
                page_controllers,
                vks,
                &input[i * len / parts..(i + 1) * len / parts],
            ));
        }
        tables.resize(
            k,
            vec![DataFrameRow {
                start: vec![0; idx_len],
                end: vec![0; idx_len],
                commitment: blank_commit.clone(),
            }],
        );
        deterministic_table_sort(tables)
    }
}

impl<'a, SC: StarkGenericConfig, const COMMITMENT_LEN: usize> VMVerifiable<'a, SC, COMMITMENT_LEN>
    for MergeSort<SC>
where
    Val<SC>: PrimeField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    type Input = &'a [DataFrameRow<u32>];

    type Output = Vec<DataFrameRow<u32>>;

    type VerifyingKeys = VerifyingKeys<SC>;

    type Params = PageControllerParams;

    type PageControllers = PageControllers<SC, COMMITMENT_LEN>;

    fn verify(
        &self,
        engine: &dyn StarkEngine<SC>,
        page_controllers: &Self::PageControllers,
        vks: &Self::VerifyingKeys,
        input: Self::Input,
    ) -> Self::Output {
        let k = page_controllers.params.k;
        match self {
            MergeSort::Merge { pre, finish } => {
                let buf = pre.verify(engine, page_controllers, vks, input);
                let m = buf.len() - k;
                finish.verify(
                    engine,
                    page_controllers,
                    vks,
                    InitRemainingGTPInput { buf, m },
                )
            }
            MergeSort::Single() => input.to_vec(),
        }
    }
}

pub fn get_commitment_from_proof<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>(
    proof: &Proof<SC>,
    vk: &MultiStarkPartialVerifyingKey<SC>,
    air_idx: usize,
) -> Vec<u32>
where
    Val<SC>: PrimeField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    let com: [Val<SC>; COMMITMENT_LEN] = proof.commitments.main_trace[vk.per_air[air_idx]
        .main_graph
        .matrix_ptrs
        .first()
        .unwrap()
        .commit_index]
        .clone()
        .into();
    com.iter()
        .map(|f| f.as_canonical_u64() as u32)
        .collect_vec()
}

pub fn verify_merge_sort_main<SC: StarkGenericConfig, const COMMITMENT_LEN: usize>(
    engine: &dyn StarkEngine<SC>,
    page_controllers: &PageControllers<SC, COMMITMENT_LEN>,
    vks: &VerifyingKeys<SC>,
    commitments: Vec<Vec<u32>>,
    main: MergeSortMain<SC, COMMITMENT_LEN>,
) -> Vec<DataFrameRow<u32>>
where
    Val<SC>: PrimeField + PrimeField64,
    Com<SC>: Into<[Val<SC>; COMMITMENT_LEN]>,
{
    let limb_bits = page_controllers.params.limb_bits;
    let idx_len = page_controllers.params.idx_len;
    let mut table = commitments
        .iter()
        .map(|c| DataFrameRow {
            start: vec![0; idx_len],
            end: vec![(1 << limb_bits) - 1; idx_len],
            commitment: c.clone(),
        })
        .collect_vec();
    if table.len() == 1 {
        table.push(DataFrameRow {
            start: vec![0; idx_len],
            end: vec![(1 << limb_bits) - 1; idx_len],
            commitment: page_controllers.params.blank_commit.clone(),
        })
    }
    main.merge_sort
        .verify(engine, page_controllers, vks, &table)
}
