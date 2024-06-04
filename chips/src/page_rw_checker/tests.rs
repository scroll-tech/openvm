use std::collections::{HashMap, HashSet};
use std::iter;
use std::panic;

use crate::page_rw_checker;
use crate::page_rw_checker::page_controller::{OpType, Operation};
use afs_stark_backend::prover::USE_DEBUG_BUILDER;
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
use rand::Rng;

use crate::page_rw_checker::page_controller;

fn load_page_test(
    engine: &BabyBearPoseidon2Engine,
    page_init: Vec<Vec<u32>>,
    key_len: usize,
    val_len: usize,
    ops: &Vec<Operation>,
    page_controller: &mut page_controller::PageController<BabyBearPoseidon2Config>,
    trace_builder: &mut TraceCommitmentBuilder<BabyBearPoseidon2Config>,
    partial_pk: &MultiStarkPartialProvingKey<BabyBearPoseidon2Config>,
    trace_degree: usize,
) -> Result<(), VerificationError> {
    let page_height = page_init.len();
    assert!(page_height > 0);

    let (page_traces, mut prover_data) = page_controller.load_page_and_ops(
        page_init.clone(),
        key_len,
        val_len,
        ops.clone(),
        trace_degree,
        &mut trace_builder.committer,
    );

    let offline_checker_trace = page_controller.offline_checker_trace();

    trace_builder.clear();

    trace_builder.load_cached_trace(page_traces[0].clone(), prover_data.remove(0));
    trace_builder.load_cached_trace(page_traces[1].clone(), prover_data.remove(0));
    trace_builder.load_trace(offline_checker_trace.clone());

    trace_builder.commit_current();

    let partial_vk = partial_pk.partial_vk();

    let main_trace_data = trace_builder.view(
        &partial_vk,
        vec![
            &page_controller.init_chip,
            &page_controller.final_chip,
            &page_controller.offline_checker,
        ],
    );

    let pis = vec![vec![]; partial_vk.per_air.len()];

    let prover = engine.prover();
    let verifier = engine.verifier();

    let mut challenger = engine.new_challenger();
    let proof = prover.prove(&mut challenger, &partial_pk, main_trace_data, &pis);

    let mut challenger = engine.new_challenger();
    let result = verifier.verify(
        &mut challenger,
        partial_vk,
        vec![
            &page_controller.init_chip,
            &page_controller.final_chip,
            &page_controller.offline_checker,
        ],
        proof,
        &pis,
    );

    result
}

#[test]
fn page_read_write_test() {
    let mut rng = create_seeded_rng();
    let bus_index = 0;

    use page_rw_checker::page_controller::PageController;

    const MAX_VAL: u32 = 0x78000001; // The prime used by BabyBear

    let log_page_height = 4;
    let log_num_ops = 3;

    let page_width = 6;
    let page_height = 1 << log_page_height;
    let num_ops: usize = 1 << log_num_ops;

    let trace_degree = num_ops * 8;

    let key_len = rng.gen::<usize>() % ((page_width - 1) - 1) + 1;
    let val_len = (page_width - 1) - key_len;

    // Generating a random page with distinct keys
    let mut page: Vec<Vec<u32>> = vec![];
    let mut key_val_map = HashMap::new();
    for _ in 0..page_height {
        let mut key;
        loop {
            key = (0..key_len)
                .map(|_| rng.gen::<u32>() % MAX_VAL)
                .collect::<Vec<u32>>();
            if !key_val_map.contains_key(&key) {
                break;
            }
        }

        let val: Vec<u32> = (0..val_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect();
        key_val_map.insert(key.clone(), val.clone());
        page.push(iter::once(1).chain(key).chain(val).collect());
    }

    // Generating random sorted timestamps for operations
    let mut clks: Vec<usize> = (0..num_ops)
        .map(|_| rng.gen::<usize>() % (MAX_VAL as usize))
        .collect();
    clks.sort();

    let mut ops: Vec<Operation> = vec![];
    for i in 0..num_ops {
        let clk = clks[i];
        let key = key_val_map
            .iter()
            .nth(rng.gen::<usize>() % key_val_map.len())
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

        let val = {
            if op_type == OpType::Read {
                key_val_map[&key].to_vec()
            } else {
                (0..val_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect()
            }
        };

        if op_type == OpType::Write {
            key_val_map.insert(key.clone(), val.clone());
        }

        ops.push(Operation::new(clk, key, val, op_type));
    }

    let mut page_controller: PageController<BabyBearPoseidon2Config> =
        PageController::new(bus_index, key_len, val_len);
    let engine = config::baby_bear_poseidon2::default_engine(log_page_height.max(3 + log_num_ops));

    let mut keygen_builder = MultiStarkKeygenBuilder::new(&engine.config);

    let init_page_ptr = keygen_builder.add_cached_main_matrix(page_width);
    let final_page_ptr = keygen_builder.add_cached_main_matrix(page_width);
    let ops_ptr = keygen_builder.add_main_matrix(7 + page_width + 2 * (key_len + val_len));

    keygen_builder.add_partitioned_air(
        &page_controller.init_chip,
        page_height,
        0,
        vec![init_page_ptr],
    );

    keygen_builder.add_partitioned_air(
        &page_controller.final_chip,
        page_height,
        0,
        vec![final_page_ptr],
    );

    keygen_builder.add_partitioned_air(
        &page_controller.offline_checker,
        trace_degree,
        0,
        vec![ops_ptr],
    );

    let partial_pk = keygen_builder.generate_partial_pk();

    let prover = MultiTraceStarkProver::new(&engine.config);
    let mut trace_builder = TraceCommitmentBuilder::new(prover.pcs());

    // Testing a fully allocated page
    load_page_test(
        &engine,
        page.clone(),
        key_len,
        val_len,
        &ops,
        &mut page_controller,
        &mut trace_builder,
        &partial_pk,
        trace_degree,
    )
    .expect("Verification failed");

    // Testing a partially-allocated page
    let rows_allocated = rng.gen::<usize>() % (page_height + 1);
    for i in rows_allocated..page_height {
        page[i][0] = 0;

        // Making sure the first operation using this key is a write
        let key = page[i][1..key_len + 1].to_vec();
        for op in ops.iter_mut() {
            if op.key == key {
                op.op_type = OpType::Write;
                break;
            }
        }
    }

    load_page_test(
        &engine,
        page.clone(),
        key_len,
        val_len,
        &ops,
        &mut page_controller,
        &mut trace_builder,
        &partial_pk,
        trace_degree,
    )
    .expect("Verification failed");

    // Testing a fully unallocated page
    for i in 0..page_height {
        // Making sure the first operation that uses every key is a write
        let key = page[i][1..key_len + 1].to_vec();
        for op in ops.iter_mut() {
            if op.key == key {
                op.op_type = OpType::Write;
                break;
            }
        }

        page[i] = (0..page_width)
            .map(|_| rng.gen::<u32>() % MAX_VAL)
            .collect();
        page[i][0] = 0;
    }

    load_page_test(
        &engine,
        page.clone(),
        key_len,
        val_len,
        &ops,
        &mut page_controller,
        &mut trace_builder,
        &partial_pk,
        trace_degree,
    )
    .expect("Verification failed");

    // Testing writing only 1 key into an unallocated page
    ops = vec![Operation::new(
        10,
        (0..key_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect(),
        (0..val_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect(),
        OpType::Write,
    )];

    load_page_test(
        &engine,
        page.clone(),
        key_len,
        val_len,
        &ops,
        &mut page_controller,
        &mut trace_builder,
        &partial_pk,
        trace_degree,
    )
    .expect("Verification failed");

    // Negative tests

    // Testing reading from a non-existing key (in a fully-unallocated page)
    ops = vec![Operation::new(
        1,
        (0..key_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect(),
        (0..val_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect(),
        OpType::Read,
    )];

    USE_DEBUG_BUILDER.with(|debug| {
        *debug.lock().unwrap() = false;
    });
    assert_eq!(
        load_page_test(
            &engine,
            page.clone(),
            key_len,
            val_len,
            &ops,
            &mut page_controller,
            &mut trace_builder,
            &partial_pk,
            trace_degree,
        ),
        Err(VerificationError::OodEvaluationMismatch),
        "Expected constraints to fail"
    );

    // Testing reading a wrong value from an existing key
    let key: Vec<u32> = (0..key_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect();
    let val_1: Vec<u32> = (0..val_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect();
    let mut val_2 = val_1.clone();
    val_2[0] += 1; // making sure val_2 is different

    ops = vec![
        Operation::new(1, key.clone(), val_1, OpType::Write),
        Operation::new(2, key, val_2, OpType::Read),
    ];

    assert_eq!(
        load_page_test(
            &engine,
            page.clone(),
            key_len,
            val_len,
            &ops,
            &mut page_controller,
            &mut trace_builder,
            &partial_pk,
            trace_degree,
        ),
        Err(VerificationError::OodEvaluationMismatch),
        "Expected constraints to fail"
    );

    // Testing writing too many keys to a fully unallocated page
    let mut key_map = HashSet::new();
    for _ in 0..page_height + 1 {
        let mut key: Vec<u32>;
        loop {
            key = (0..key_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect();
            if !key_map.contains(&key) {
                break;
            }
        }

        key_map.insert(key);
    }

    ops.clear();
    for (i, key) in key_map.iter().enumerate() {
        ops.push(Operation::new(
            i + 1,
            key.clone(),
            (0..val_len).map(|_| rng.gen::<u32>() % MAX_VAL).collect(),
            OpType::Write,
        ));
    }

    let engine_ref = &engine;
    let result = panic::catch_unwind(move || {
        let _ = load_page_test(
            engine_ref,
            page.clone(),
            key_len,
            val_len,
            &ops,
            &mut page_controller,
            &mut trace_builder,
            &partial_pk,
            trace_degree,
        );
    });

    assert!(
        result.is_err(),
        "Expected to fail when allocating too many keys"
    );
}
