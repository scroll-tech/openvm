use std::collections::HashMap;

use std::sync::Arc;

use afs_stark_backend::prover::trace::TraceCommitter;
use afs_stark_backend::rap::AnyRap;
use afs_stark_backend::verifier::VerificationError;
use afs_stark_backend::{
    keygen::{types::MultiStarkPartialProvingKey, MultiStarkKeygenBuilder},
    prover::{trace::TraceCommitmentBuilder, MultiTraceStarkProver},
};
use afs_test_utils::config::{
    self,
    baby_bear_poseidon2::{BabyBearPoseidon2Config, BabyBearPoseidon2Engine},
};
use afs_test_utils::{engine::StarkEngine, utils::create_seeded_rng};

use p3_baby_bear::BabyBear;
use rand::Rng;

use crate::multitier_readonly::page_controller::{PageController, PageTreeParams};

use crate::multitier_readonly::page_requester::PageRequester;
use crate::pagebtree::{PageBTree, PageBTreePages};

use super::page_controller;

pub const BABYBEAR_COMMITMENT_LEN: usize = 8;
pub const DECOMP_BITS: usize = 6;

#[test]
fn multitier_page_readonly_no_new_keys() {
    multitier_page_readonly_test(generate_no_new_keys, false, 4);
}

fn multitier_page_readonly_test<F>(generate_inputs: F, should_fail: bool, log_page_height: usize)
where
    F: Fn(
        usize,
        usize,
        usize,
        usize,
        usize,
        &mut TraceCommitter<BabyBearPoseidon2Config>,
    ) -> (PageBTreePages, bool, Vec<Vec<u32>>),
{
    let data_bus_index = 0;
    let limb_bits = 20;

    let log_num_ops = 3;

    let page_height = 1 << log_page_height;
    let num_ops: usize = 1 << log_num_ops;

    let idx_len = 2;
    let data_len = 3;

    let engine = config::baby_bear_poseidon2::default_engine(log_page_height.max(3 + log_num_ops));
    let prover = MultiTraceStarkProver::new(&engine.config);
    let mut trace_builder = TraceCommitmentBuilder::new(prover.pcs());

    let init_path_bus = 1;

    let tree_param = PageTreeParams {
        path_bus_index: init_path_bus,
        leaf_cap: 8,
        internal_cap: 24,
        leaf_page_height: page_height,
        internal_page_height: page_height,
    };

    let mut page_controller: PageController<BABYBEAR_COMMITMENT_LEN> =
        PageController::new(data_bus_index, idx_len, data_len, tree_param.clone());

    let mut keygen_builder = MultiStarkKeygenBuilder::new(&engine.config);

    let mut leaf_data_ptrs = vec![];
    let mut leaf_main_ptrs = vec![];

    let mut internal_data_ptrs = vec![];
    let mut internal_main_ptrs = vec![];

    for _ in 0..tree_param.leaf_cap {
        leaf_data_ptrs.push(keygen_builder.add_cached_main_matrix(2 + idx_len + data_len));
    }

    for _ in 0..tree_param.internal_cap {
        internal_data_ptrs
            .push(keygen_builder.add_cached_main_matrix(2 + 2 * idx_len + BABYBEAR_COMMITMENT_LEN));
    }

    for _ in 0..tree_param.leaf_cap {
        leaf_main_ptrs.push(
            keygen_builder.add_main_matrix(
                page_controller.leaf_chips[0].air_width() - 2 - idx_len - data_len,
            ),
        );
    }

    for _ in 0..tree_param.internal_cap {
        internal_main_ptrs.push(keygen_builder.add_main_matrix(
            page_controller.internal_chips[0].air_width()
                - 2
                - 2 * idx_len
                - BABYBEAR_COMMITMENT_LEN,
        ));
    }

    for i in 0..tree_param.leaf_cap {
        keygen_builder.add_partitioned_air(
            &page_controller.leaf_chips[i],
            page_height,
            BABYBEAR_COMMITMENT_LEN,
            vec![leaf_data_ptrs[i], leaf_main_ptrs[i]],
        );
    }

    for i in 0..tree_param.internal_cap {
        keygen_builder.add_partitioned_air(
            &page_controller.internal_chips[i],
            page_height,
            BABYBEAR_COMMITMENT_LEN,
            vec![internal_data_ptrs[i], internal_main_ptrs[i]],
        );
    }
    let page_requester = PageRequester::new(data_bus_index, idx_len, data_len);
    keygen_builder.add_air(&page_requester, num_ops, 0);

    keygen_builder.add_air(&page_controller.root_signal, 1, BABYBEAR_COMMITMENT_LEN);

    let partial_pk = keygen_builder.generate_partial_pk();
    let (pages, root_is_leaf, requests) = generate_inputs(
        idx_len,
        data_len,
        page_height,
        limb_bits,
        num_ops,
        &mut trace_builder.committer,
    );
    let res = load_page_test(
        &engine,
        pages.leaf_pages,
        pages.internal_pages,
        root_is_leaf,
        0,
        &page_requester,
        &requests,
        &mut page_controller,
        &mut trace_builder,
        &partial_pk,
    );
    assert!(should_fail == res.is_err());
    if !should_fail {
        res.unwrap();
    }
}

fn load_page_test(
    engine: &BabyBearPoseidon2Engine,
    leaf_pages: Vec<Vec<Vec<u32>>>,
    internal_pages: Vec<Vec<Vec<u32>>>,
    root_is_leaf: bool,
    root_idx: usize,
    requester: &PageRequester,
    requests: &Vec<Vec<u32>>,
    page_controller: &mut page_controller::PageController<BABYBEAR_COMMITMENT_LEN>,
    trace_builder: &mut TraceCommitmentBuilder<BabyBearPoseidon2Config>,
    partial_pk: &MultiStarkPartialProvingKey<BabyBearPoseidon2Config>,
) -> Result<(), VerificationError> {
    page_controller.clear();
    page_controller.load_pages(&leaf_pages);
    let requester_trace = requester.generate_trace::<BabyBear, BABYBEAR_COMMITMENT_LEN>(
        requests.clone(),
        Arc::new(page_controller),
    );
    let mut tree_prods = page_controller.generate_trace_data(
        &mut trace_builder.committer,
        leaf_pages,
        internal_pages,
        root_is_leaf,
        root_idx,
    );
    let root_trace = tree_prods.root.main_traces;
    trace_builder.clear();

    for trace in tree_prods.leaf.data_traces.iter() {
        trace_builder.load_cached_trace(trace.clone(), tree_prods.leaf.prover_data.remove(0));
    }

    for trace in tree_prods.internal.data_traces.iter() {
        trace_builder.load_cached_trace(trace.clone(), tree_prods.internal.prover_data.remove(0));
    }

    for trace in tree_prods.leaf.main_traces.iter() {
        trace_builder.load_trace(trace.clone());
    }

    for trace in tree_prods.internal.main_traces.iter() {
        trace_builder.load_trace(trace.clone());
    }

    trace_builder.load_trace(requester_trace.clone());
    trace_builder.load_trace(root_trace);
    trace_builder.commit_current();

    let mut airs: Vec<&dyn AnyRap<BabyBearPoseidon2Config>> = vec![];
    for chip in &page_controller.leaf_chips {
        airs.push(chip);
    }
    for chip in &page_controller.internal_chips {
        airs.push(chip);
    }
    airs.push(requester);
    airs.push(&page_controller.root_signal);
    let partial_vk = partial_pk.partial_vk();
    let main_trace_data = trace_builder.view(&partial_vk, airs.clone());

    let mut pis = vec![];
    for c in tree_prods.leaf.commitments {
        let c: [BabyBear; BABYBEAR_COMMITMENT_LEN] = c.into();
        pis.push(c.to_vec());
    }
    for c in tree_prods.internal.commitments {
        let c: [BabyBear; BABYBEAR_COMMITMENT_LEN] = c.into();
        pis.push(c.to_vec());
    }
    pis.push(vec![]);
    {
        let c: [BabyBear; BABYBEAR_COMMITMENT_LEN] = tree_prods.root.commitments.into();
        pis.push(c.to_vec());
    }
    let prover = engine.prover();
    let verifier = engine.verifier();

    let mut challenger = engine.new_challenger();
    let proof = prover.prove(&mut challenger, &partial_pk, main_trace_data, &pis);

    let mut challenger = engine.new_challenger();
    let result = verifier.verify(&mut challenger, partial_vk, airs, proof, &pis);

    result
}

fn generate_no_new_keys(
    idx_len: usize,
    data_len: usize,
    page_height: usize,
    limb_bits: usize,
    num_ops: usize,
    committer: &mut TraceCommitter<BabyBearPoseidon2Config>,
) -> (PageBTreePages, bool, Vec<Vec<u32>>) {
    const MAX_VAL: u32 = 0x78000001; // The prime used by BabyBear
    const MAX_LIMB_VAL: u32 = 1 << 20;
    let mut rng = create_seeded_rng();
    let mut btree = PageBTree::<16, 16, BABYBEAR_COMMITMENT_LEN>::new(
        limb_bits,
        idx_len,
        data_len,
        page_height,
        page_height,
    );
    // Generating a random page with distinct indices
    let mut idx_data_map = HashMap::new();
    for _ in 0..4 * page_height {
        let mut idx;
        loop {
            idx = (0..idx_len)
                .map(|_| rng.gen::<u32>() % MAX_LIMB_VAL)
                .collect::<Vec<u32>>();
            if !idx_data_map.contains_key(&idx) {
                break;
            }
        }

        let data: Vec<u32> = (0..data_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect();
        idx_data_map.insert(idx.clone(), data.clone());
        btree.update(&idx, &data);
    }
    let init_pages = btree.gen_all_trace(committer);
    // check for correctness
    for (idx, _) in idx_data_map.iter() {
        btree.search(idx).unwrap();
    }

    let mut requests: Vec<Vec<u32>> = vec![];
    for _ in 0..num_ops {
        let idx = idx_data_map
            .iter()
            .nth(rng.gen::<usize>() % idx_data_map.len())
            .unwrap()
            .0
            .to_vec();

        requests.push(idx);
    }
    (init_pages, false, requests)
}
