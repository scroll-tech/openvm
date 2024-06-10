use std::collections::{HashMap, HashSet};
use std::iter;
use std::panic;
use std::sync::Arc;

use afs_stark_backend::prover::USE_DEBUG_BUILDER;
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

use p3_air::BaseAir;
use p3_baby_bear::BabyBear;
use rand::Rng;

use crate::multitier_page_rw_checker::page_controller::{
    LessThanTupleParams, PageController, PageTreeParams,
};
use crate::page_rw_checker::page_controller::{OpType, Operation};
use crate::pagebtree::PageBTree;
use crate::range_gate::RangeCheckerGateChip;

use super::page_controller;

pub const BABYBEAR_COMMITMENT_LEN: usize = 8;

fn load_page_test(
    engine: &BabyBearPoseidon2Engine,
    init_leaf_pages: Vec<Vec<Vec<u32>>>,
    init_internal_pages: Vec<Vec<Vec<u32>>>,
    init_root_is_leaf: bool,
    init_root_idx: usize,
    final_leaf_pages: Vec<Vec<Vec<u32>>>,
    final_internal_pages: Vec<Vec<Vec<u32>>>,
    final_root_is_leaf: bool,
    final_root_idx: usize,
    idx_len: usize,
    data_len: usize,
    ops: &Vec<Operation>,
    page_controller: &mut page_controller::PageController<
        BabyBearPoseidon2Config,
        BABYBEAR_COMMITMENT_LEN,
    >,
    trace_builder: &mut TraceCommitmentBuilder<BabyBearPoseidon2Config>,
    partial_pk: &MultiStarkPartialProvingKey<BabyBearPoseidon2Config>,
    trace_degree: usize,
) -> Result<(), VerificationError> {
    let (data_trace, main_trace, commits, mut prover_data) = page_controller.load_page_and_ops(
        init_leaf_pages,
        init_internal_pages,
        init_root_is_leaf,
        init_root_idx,
        final_leaf_pages,
        final_internal_pages,
        final_root_is_leaf,
        final_root_idx,
        ops.clone(),
        trace_degree,
        &mut trace_builder.committer,
    );

    let offline_checker_trace = main_trace.offline_checker_trace.clone();
    let init_root = main_trace.init_root_signal_trace.clone();
    let final_root = main_trace.final_root_signal_trace.clone();
    trace_builder.clear();

    for trace in data_trace.init_leaf_chip_traces.iter() {
        trace_builder.load_cached_trace(trace.clone(), prover_data.init_leaf_page.remove(0));
    }

    for trace in data_trace.init_internal_chip_traces.iter() {
        trace_builder.load_cached_trace(trace.clone(), prover_data.init_internal_page.remove(0));
    }

    for trace in data_trace.final_leaf_chip_traces.iter() {
        trace_builder.load_cached_trace(trace.clone(), prover_data.final_leaf_page.remove(0));
    }

    for trace in data_trace.final_internal_chip_traces.iter() {
        println!("data: {:?}", trace);
        trace_builder.load_cached_trace(trace.clone(), prover_data.final_internal_page.remove(0));
    }

    for trace in main_trace.init_leaf_chip_main_traces.iter() {
        trace_builder.load_trace(trace.clone());
    }

    for trace in main_trace.init_internal_chip_main_traces.iter() {
        trace_builder.load_trace(trace.clone());
    }

    for trace in main_trace.final_leaf_chip_main_traces.iter() {
        trace_builder.load_trace(trace.clone());
    }

    for trace in main_trace.final_internal_chip_main_traces.iter() {
        println!("main: {:?}", trace);
        trace_builder.load_trace(trace.clone());
    }

    trace_builder.load_trace(offline_checker_trace.clone());
    trace_builder.load_trace(init_root);
    trace_builder.load_trace(final_root);
    trace_builder.commit_current();

    let mut airs: Vec<&dyn AnyRap<BabyBearPoseidon2Config>> = vec![];
    for chip in &page_controller.init_leaf_chips {
        airs.push(chip);
    }
    for chip in &page_controller.init_internal_chips {
        airs.push(chip);
    }
    for chip in &page_controller.final_leaf_chips {
        airs.push(chip);
    }
    for chip in &page_controller.final_internal_chips {
        airs.push(chip);
    }
    airs.push(&page_controller.offline_checker);
    airs.push(&page_controller.init_root_signal);
    airs.push(&page_controller.final_root_signal);
    println!("{:?}", airs.len());
    let partial_vk = partial_pk.partial_vk();
    let main_trace_data = trace_builder.view(&partial_vk, airs.clone());

    let mut pis = vec![];
    for c in commits.init_leaf_page_commitments {
        let c: [BabyBear; BABYBEAR_COMMITMENT_LEN] = c.into();
        pis.push(c.to_vec());
    }
    for c in commits.init_internal_page_commitments {
        let c: [BabyBear; BABYBEAR_COMMITMENT_LEN] = c.into();
        pis.push(c.to_vec());
    }
    for c in commits.final_leaf_page_commitments {
        let c: [BabyBear; BABYBEAR_COMMITMENT_LEN] = c.into();
        pis.push(c.to_vec());
    }
    for c in commits.final_internal_page_commitments {
        let c: [BabyBear; BABYBEAR_COMMITMENT_LEN] = c.into();
        pis.push(c.to_vec());
    }
    pis.push(vec![]);
    {
        let c: [BabyBear; BABYBEAR_COMMITMENT_LEN] = commits.init_root_commitment.into();
        pis.push(c.to_vec());
    }
    {
        let c: [BabyBear; BABYBEAR_COMMITMENT_LEN] = commits.final_root_commitment.into();
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

#[test]
fn page_read_write_test() {
    let mut rng = create_seeded_rng();
    let data_bus_index = 0;
    let internal_data_bus_index = 1;
    let lt_bus_index = 2;
    let limb_bits = 20;

    const MAX_VAL: u32 = 0x78000001; // The prime used by BabyBear
    const MAX_LIMB_VAL: u32 = 1 << 20;
    let log_page_height = 4;
    let log_num_ops = 3;

    let page_height = 1 << log_page_height;
    let num_ops: usize = 1 << log_num_ops;

    let trace_degree = num_ops * 8;

    let idx_len = 2;
    let data_len = 3;
    let mut btree = PageBTree::<16, 16, BABYBEAR_COMMITMENT_LEN>::new(
        limb_bits,
        idx_len,
        data_len,
        page_height,
    );
    // Generating a random page with distinct indices
    let mut page: Vec<Vec<u32>> = vec![];
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
        page.push(
            iter::once(1)
                .chain(idx.clone())
                .chain(data.clone())
                .collect(),
        );
        btree.update(&idx, &data);
    }

    let engine = config::baby_bear_poseidon2::default_engine(log_page_height.max(3 + log_num_ops));
    let prover = MultiTraceStarkProver::new(&engine.config);
    let mut trace_builder = TraceCommitmentBuilder::new(prover.pcs());
    let init_pages = btree.gen_all_trace(&mut trace_builder.committer);

    // Generating random sorted timestamps for operations
    let mut clks: Vec<usize> = (0..num_ops)
        .map(|_| rng.gen::<usize>() % (MAX_VAL as usize))
        .collect();
    clks.sort();

    let mut ops: Vec<Operation> = vec![];
    for i in 0..num_ops {
        let clk = clks[i];
        let idx = idx_data_map
            .iter()
            .nth(rng.gen::<usize>() % idx_data_map.len())
            .unwrap()
            .0
            .to_vec();

        let op_type = {
            if rng.gen::<bool>() {
                OpType::Read
            } else {
                OpType::Write
            }
        };

        let data = {
            if op_type == OpType::Read {
                idx_data_map[&idx].to_vec()
            } else {
                (0..data_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect()
            }
        };

        if op_type == OpType::Write {
            idx_data_map.insert(idx.clone(), data.clone());
            btree.update(&idx, &data);
        }

        ops.push(Operation::new(clk, idx, data, op_type));
    }

    let init_path_bus = 3;
    let final_path_bus = 4;

    let init_param = PageTreeParams {
        path_bus_index: init_path_bus,
        leaf_cap: 10,
        internal_cap: 2,
        leaf_page_height: page_height,
        internal_page_height: page_height,
    };

    let final_param = PageTreeParams {
        path_bus_index: final_path_bus,
        leaf_cap: 10,
        internal_cap: 2,
        leaf_page_height: page_height,
        internal_page_height: page_height,
    };

    let less_than_tuple_param = LessThanTupleParams {
        range_max: 1 << 20,
        limb_bits: vec![20; idx_len],
        decomp: 8,
    };

    let range_checker = Arc::new(RangeCheckerGateChip::new(lt_bus_index, 1 << 20));

    let mut page_controller: PageController<BabyBearPoseidon2Config, BABYBEAR_COMMITMENT_LEN> =
        PageController::new(
            data_bus_index,
            internal_data_bus_index,
            lt_bus_index,
            idx_len,
            data_len,
            init_param.clone(),
            final_param.clone(),
            less_than_tuple_param,
            range_checker,
        );

    let final_pages = btree.gen_all_trace(&mut trace_builder.committer);

    let mut keygen_builder = MultiStarkKeygenBuilder::new(&engine.config);

    let mut init_leaf_data_ptrs = vec![];
    let mut init_leaf_main_ptrs = vec![];

    let mut init_internal_data_ptrs = vec![];
    let mut init_internal_main_ptrs = vec![];

    let mut final_leaf_data_ptrs = vec![];
    let mut final_leaf_main_ptrs = vec![];

    let mut final_internal_data_ptrs = vec![];
    let mut final_internal_main_ptrs = vec![];

    for _ in 0..init_param.leaf_cap {
        init_leaf_data_ptrs.push(keygen_builder.add_cached_main_matrix(1 + idx_len + data_len));
    }

    for _ in 0..init_param.internal_cap {
        init_internal_data_ptrs
            .push(keygen_builder.add_cached_main_matrix(1 + 2 * idx_len + BABYBEAR_COMMITMENT_LEN));
    }

    for _ in 0..final_param.leaf_cap {
        final_leaf_data_ptrs.push(keygen_builder.add_cached_main_matrix(1 + idx_len + data_len));
    }

    for _ in 0..final_param.internal_cap {
        final_internal_data_ptrs
            .push(keygen_builder.add_cached_main_matrix(1 + 2 * idx_len + BABYBEAR_COMMITMENT_LEN));
    }

    for _ in 0..init_param.leaf_cap {
        init_leaf_main_ptrs.push(keygen_builder.add_main_matrix(
            page_controller.init_leaf_chips[0].air_width() - 1 - idx_len - data_len,
        ));
    }

    for _ in 0..init_param.internal_cap {
        init_internal_main_ptrs.push(keygen_builder.add_main_matrix(
            page_controller.init_internal_chips[0].air_width()
                - 1
                - 2 * idx_len
                - BABYBEAR_COMMITMENT_LEN,
        ));
    }

    for _ in 0..final_param.leaf_cap {
        final_leaf_main_ptrs.push(keygen_builder.add_main_matrix(
            page_controller.final_leaf_chips[0].air_width() - 1 - idx_len - data_len,
        ));
    }

    for _ in 0..final_param.internal_cap {
        final_internal_main_ptrs.push(keygen_builder.add_main_matrix(
            page_controller.final_internal_chips[0].air_width()
                - 1
                - 2 * idx_len
                - BABYBEAR_COMMITMENT_LEN,
        ));
    }

    let ops_ptr = keygen_builder.add_main_matrix(page_controller.offline_checker.air_width());

    let init_root_ptr =
        keygen_builder.add_main_matrix(page_controller.init_root_signal.air_width());
    let final_root_ptr =
        keygen_builder.add_main_matrix(page_controller.final_root_signal.air_width());

    for i in 0..init_param.leaf_cap {
        keygen_builder.add_partitioned_air(
            &page_controller.init_leaf_chips[i],
            page_height,
            BABYBEAR_COMMITMENT_LEN,
            vec![init_leaf_data_ptrs[i], init_leaf_main_ptrs[i]],
        );
    }

    for i in 0..init_param.internal_cap {
        keygen_builder.add_partitioned_air(
            &page_controller.init_internal_chips[i],
            page_height,
            BABYBEAR_COMMITMENT_LEN,
            vec![init_internal_data_ptrs[i], init_internal_main_ptrs[i]],
        );
    }

    for i in 0..final_param.leaf_cap {
        keygen_builder.add_partitioned_air(
            &page_controller.final_leaf_chips[i],
            page_height,
            BABYBEAR_COMMITMENT_LEN,
            vec![final_leaf_data_ptrs[i], final_leaf_main_ptrs[i]],
        );
    }

    for i in 0..final_param.internal_cap {
        keygen_builder.add_partitioned_air(
            &page_controller.final_internal_chips[i],
            page_height,
            BABYBEAR_COMMITMENT_LEN,
            vec![final_internal_data_ptrs[i], final_internal_main_ptrs[i]],
        );
    }

    keygen_builder.add_partitioned_air(
        &page_controller.offline_checker,
        trace_degree,
        0,
        vec![ops_ptr],
    );

    keygen_builder.add_partitioned_air(
        &page_controller.init_root_signal,
        1,
        BABYBEAR_COMMITMENT_LEN,
        vec![init_root_ptr],
    );

    keygen_builder.add_partitioned_air(
        &page_controller.final_root_signal,
        1,
        BABYBEAR_COMMITMENT_LEN,
        vec![final_root_ptr],
    );

    let partial_pk = keygen_builder.generate_partial_pk();
    // Testing a fully allocated page
    load_page_test(
        &engine,
        init_pages.leaf_pages,
        init_pages.internal_pages,
        false,
        0,
        final_pages.leaf_pages,
        final_pages.internal_pages,
        false,
        0,
        idx_len,
        data_len,
        &ops,
        &mut page_controller,
        &mut trace_builder,
        &partial_pk,
        trace_degree,
    )
    .expect("Verification failed");

    // // Testing a partially-allocated page
    // let rows_allocated = rng.gen::<usize>() % (page_height + 1);
    // for i in rows_allocated..page_height {
    //     page[i][0] = 0;

    //     // Making sure the first operation using this index is a write
    //     let idx = page[i][1..idx_len + 1].to_vec();
    //     for op in ops.iter_mut() {
    //         if op.idx == idx {
    //             op.op_type = OpType::Write;
    //             break;
    //         }
    //     }
    // }

    // load_page_test(
    //     &engine,
    //     page.clone(),
    //     idx_len,
    //     data_len,
    //     &ops,
    //     &mut page_controller,
    //     &mut trace_builder,
    //     &partial_pk,
    //     trace_degree,
    // )
    // .expect("Verification failed");

    // // Testing a fully unallocated page
    // for i in 0..page_height {
    //     // Making sure the first operation that uses every index is a write
    //     let idx = page[i][1..idx_len + 1].to_vec();
    //     for op in ops.iter_mut() {
    //         if op.idx == idx {
    //             op.op_type = OpType::Write;
    //             break;
    //         }
    //     }

    //     page[i] = (0..page_width)
    //         .map(|_| rng.gen::<u32>() % MAX_VAL)
    //         .collect();
    //     page[i][0] = 0;
    // }

    // load_page_test(
    //     &engine,
    //     page.clone(),
    //     idx_len,
    //     data_len,
    //     &ops,
    //     &mut page_controller,
    //     &mut trace_builder,
    //     &partial_pk,
    //     trace_degree,
    // )
    // .expect("Verification failed");

    // // Testing writing only 1 index into an unallocated page
    // ops = vec![Operation::new(
    //     10,
    //     (0..idx_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect(),
    //     (0..data_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect(),
    //     OpType::Write,
    // )];

    // load_page_test(
    //     &engine,
    //     page.clone(),
    //     idx_len,
    //     data_len,
    //     &ops,
    //     &mut page_controller,
    //     &mut trace_builder,
    //     &partial_pk,
    //     trace_degree,
    // )
    // .expect("Verification failed");

    // // Negative tests

    // // Testing reading from a non-existing index (in a fully-unallocated page)
    // ops = vec![Operation::new(
    //     1,
    //     (0..idx_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect(),
    //     (0..data_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect(),
    //     OpType::Read,
    // )];

    // USE_DEBUG_BUILDER.with(|debug| {
    //     *debug.lock().unwrap() = false;
    // });
    // assert_eq!(
    //     load_page_test(
    //         &engine,
    //         page.clone(),
    //         idx_len,
    //         data_len,
    //         &ops,
    //         &mut page_controller,
    //         &mut trace_builder,
    //         &partial_pk,
    //         trace_degree,
    //     ),
    //     Err(VerificationError::OodEvaluationMismatch),
    //     "Expected constraints to fail"
    // );

    // // Testing reading wrong data from an existing index
    // let idx: Vec<u32> = (0..idx_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect();
    // let data_1: Vec<u32> = (0..data_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect();
    // let mut data_2 = data_1.clone();
    // data_2[0] += 1; // making sure data_2 is different

    // ops = vec![
    //     Operation::new(1, idx.clone(), data_1, OpType::Write),
    //     Operation::new(2, idx, data_2, OpType::Read),
    // ];

    // assert_eq!(
    //     load_page_test(
    //         &engine,
    //         page.clone(),
    //         idx_len,
    //         data_len,
    //         &ops,
    //         &mut page_controller,
    //         &mut trace_builder,
    //         &partial_pk,
    //         trace_degree,
    //     ),
    //     Err(VerificationError::OodEvaluationMismatch),
    //     "Expected constraints to fail"
    // );

    // // Testing writing too many indices to a fully unallocated page
    // let mut idx_map = HashSet::new();
    // for _ in 0..page_height + 1 {
    //     let mut idx: Vec<u32>;
    //     loop {
    //         idx = (0..idx_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect();
    //         if !idx_map.contains(&idx) {
    //             break;
    //         }
    //     }

    //     idx_map.insert(idx);
    // }

    // ops.clear();
    // for (i, idx) in idx_map.iter().enumerate() {
    //     ops.push(Operation::new(
    //         i + 1,
    //         idx.clone(),
    //         (0..data_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect(),
    //         OpType::Write,
    //     ));
    // }

    // let engine_ref = &engine;
    // let result = panic::catch_unwind(move || {
    //     let _ = load_page_test(
    //         engine_ref,
    //         page.clone(),
    //         idx_len,
    //         data_len,
    //         &ops,
    //         &mut page_controller,
    //         &mut trace_builder,
    //         &partial_pk,
    //         trace_degree,
    //     );
    // });

    // assert!(
    //     result.is_err(),
    //     "Expected to fail when allocating too many indices"
    // );
}
