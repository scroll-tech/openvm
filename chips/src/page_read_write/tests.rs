use std::collections::HashMap;
use std::iter;

use crate::page_read_write;
use crate::page_read_write::page_controller::{OpType, Operation};
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

use crate::page_read_write::page_controller;

// TODO: make ops not a parameter and generate inside
// TODO: don't forget to clear trace builder
fn load_page_test(
    engine: &BabyBearPoseidon2Engine,
    page_init: &Vec<Vec<u32>>,
    key_len: usize,
    val_len: usize,
    ops: &Vec<Operation>,
    page_controller: &mut page_controller::PageController<BabyBearPoseidon2Config>,
    trace_builder: &mut TraceCommitmentBuilder<BabyBearPoseidon2Config>,
    partial_pk: &MultiStarkPartialProvingKey<BabyBearPoseidon2Config>,
) -> Result<(), VerificationError> {
    let page_height = page_init.len();
    assert!(page_height > 0);

    println!("In load_page_test");

    let (page_traces, mut prover_data) = page_controller.load_page_and_ops(
        page_init.clone(),
        key_len,
        val_len,
        ops.clone(),
        &mut trace_builder.committer,
    );

    println!("Committed to the traces of initial and final pages");

    // TODO: maybe define a getter for this
    let middle_chip_trace = page_controller.middle_chip_trace.clone().unwrap();

    trace_builder.clear();

    trace_builder.load_cached_trace(page_traces[0].clone(), prover_data.remove(0));
    trace_builder.load_cached_trace(page_traces[1].clone(), prover_data.remove(0));
    trace_builder.load_trace(middle_chip_trace);

    println!("loaded all the traces");

    trace_builder.commit_current();

    println!("committed to all loaded traces");

    let partial_vk = partial_pk.partial_vk();

    let main_trace_data = trace_builder.view(
        &partial_vk,
        vec![
            &page_controller.init_chip,
            &page_controller.final_chip,
            &page_controller.middle_chip,
        ],
    );

    println!("viewed trace successfully");

    let pis = vec![vec![]; partial_vk.per_air.len()];

    let prover = engine.prover();
    let verifier = engine.verifier();

    println!("defined prover and verifier");

    let mut challenger = engine.new_challenger();
    let proof = prover.prove(&mut challenger, &partial_pk, main_trace_data, &pis);

    println!("defined challenger and proof");

    let mut challenger = engine.new_challenger();
    let result = verifier.verify(
        &mut challenger,
        partial_vk,
        vec![&page_controller.middle_chip],
        proof,
        &pis,
    );

    result
}

// TODO: Make sure all the keys generated in the test are distinct
#[test]
fn page_read_write_test() {
    let mut rng = create_seeded_rng();
    let bus_index = 0;

    use page_read_write::page_controller::PageController;

    // TODO: remove this
    let max_val = 10;

    let log_page_height = 3;
    let log_num_ops = 5;

    let page_width = 4;
    let page_height = 1 << log_page_height;
    let num_ops: usize = 1 << log_num_ops;

    let key_len = rng.gen::<usize>() % ((page_width - 1) - 1) + 1;
    let val_len = (page_width - 1) - key_len;

    println!("page_width: {}, page_height: {}", page_width, page_height);

    println!("key_len: {}, val_len: {}", key_len, val_len);

    let page = (0..page_height)
        .map(|_| {
            iter::once(1)
                .chain(0..page_width - 1)
                .map(|_| rng.gen::<u32>() % max_val)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    println!("page: {:?}", page);

    // Generating random sorted timestamps for operations
    let mut clks: Vec<usize> = (0..num_ops)
        .map(|_| rng.gen::<usize>() % (max_val as usize))
        .collect();
    clks.sort();

    let mut all_keys = HashMap::new();
    for (i, row) in page.iter().enumerate() {
        let key = row[1..key_len + 1].to_vec().clone();
        all_keys.insert(key, i);
    }

    println!("Number of keys: {}", all_keys.len());

    let mut ops: Vec<Operation> = vec![];
    for i in 0..num_ops {
        let clk = clks[i];
        let key = all_keys
            .iter()
            .nth(rng.gen::<usize>() % all_keys.len())
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
                page[all_keys[&key]][key_len + 1..].to_vec()
            } else {
                (0..val_len).map(|_| rng.gen::<u32>() % max_val).collect()
            }
        };

        ops.push(Operation::new(clk, key, val, op_type));
    }

    println!("ops: {:?}", ops);

    let mut page_controller: PageController<BabyBearPoseidon2Config> =
        PageController::new(bus_index);
    let engine = config::baby_bear_poseidon2::default_engine(log_page_height.max(3 + log_num_ops));

    println!("Initialized page_controller");

    let mut keygen_builder = MultiStarkKeygenBuilder::new(&engine.config);

    let init_page_ptr = keygen_builder.add_cached_main_matrix(page_width);
    let final_page_ptr = keygen_builder.add_cached_main_matrix(page_width);
    let ops_ptr = keygen_builder.add_main_matrix(7 + page_width);

    println!(
        "ops trace width should be {} because page_width is {}",
        7 + page_width,
        page_width
    );

    // TODO: fix the degree
    // keygen_builder.add_partitioned_air(
    //     &page_controller.middle_chip,
    //     num_ops * 8,
    //     0,
    //     vec![init_page_ptr, final_page_ptr, ops_ptr],
    // );

    keygen_builder.add_air(&page_controller.init_chip, page_height, 0);
    keygen_builder.add_air(&page_controller.final_chip, page_height, 0);
    keygen_builder.add_air(&page_controller.middle_chip, num_ops * 8, 0);

    let partial_pk = keygen_builder.generate_partial_pk();

    let prover = MultiTraceStarkProver::new(&engine.config);
    let mut trace_builder = TraceCommitmentBuilder::new(prover.pcs());

    println!("Done everything just calling load_page_test next");

    load_page_test(
        &engine,
        &page,
        key_len,
        val_len,
        &ops,
        &mut page_controller,
        &mut trace_builder,
        &partial_pk,
    )
    .expect("Verification failed");
}
