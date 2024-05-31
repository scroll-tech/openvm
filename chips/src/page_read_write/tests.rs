use std::collections::{HashMap, HashSet};
use std::iter;

use crate::page_read_write;
use crate::page_read_write::checker_controller::{OpType, Operation};
use afs_stark_backend::verifier::VerificationError;
use afs_stark_backend::{
    keygen::{types::MultiStarkPartialProvingKey, MultiStarkKeygenBuilder},
    prover::{trace::TraceCommitmentBuilder, MultiTraceStarkProver},
};
use afs_test_utils::config::{
    self,
    baby_bear_poseidon2::{BabyBearPoseidon2Config, BabyBearPoseidon2Engine},
};
use afs_test_utils::{
    engine::StarkEngine, interaction::dummy_interaction_air::DummyInteractionAir,
    utils::create_seeded_rng,
};
use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use p3_matrix::dense::RowMajorMatrix;
use rand::Rng;

type Val = BabyBear;

fn load_page_test(
    engine: &BabyBearPoseidon2Engine,
    page_to_receive: &Vec<Vec<u32>>,
    page_to_send: &Vec<Vec<u32>>,
    page_controller: &mut page_controller::PageController<BabyBearPoseidon2Config>,
    page_requester: &DummyInteractionAir,
    trace_builder: &mut TraceCommitmentBuilder<BabyBearPoseidon2Config>,
    partial_pk: &MultiStarkPartialProvingKey<BabyBearPoseidon2Config>,
    num_requests: usize,
) -> Result<(), VerificationError> {
    let mut rng = create_seeded_rng();

    let page_height = page_to_receive.len();
    assert!(page_height > 0);
    let page_width = page_to_receive[0].len();

    let requests = (0..num_requests)
        .map(|_| rng.gen::<usize>() % page_height)
        .collect::<Vec<usize>>();

    let (page_trace, prover_data) =
        page_controller.load_page(&mut trace_builder.committer, page_to_receive.clone());

    let requester_trace = RowMajorMatrix::new(
        requests
            .iter()
            .flat_map(|i| {
                page_controller.request(*i);
                iter::once(1 as u32)
                    .chain(iter::once(*i as u32))
                    .chain(page_to_send[*i].clone())
            })
            .map(Val::from_wrapped_u32)
            .collect(),
        2 + page_width,
    );

    let page_metadata_trace = page_controller.generate_trace();

    trace_builder.clear();

    trace_builder.load_cached_trace(page_trace, prover_data);
    trace_builder.load_trace(page_metadata_trace);
    trace_builder.load_trace(requester_trace);

    trace_builder.commit_current();

    let partial_vk = partial_pk.partial_vk();

    let main_trace_data = trace_builder.view(
        &partial_vk,
        vec![&page_controller.page_read_chip, page_requester],
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
        vec![&page_controller.page_read_chip, page_requester],
        proof,
        &pis,
    );

    result
}

#[test]
fn page_read_chip_test() {
    let mut rng = create_seeded_rng();
    let bus_index = 0;

    use page_read_write::checker_controller::CheckerController;

    let log_page_height = 3;
    let log_num_ops = 5;

    let page_width = 4;
    let page_height = 1 << log_page_height;
    let num_ops: usize = 1 << log_num_ops;

    let key_len = rng.gen::<usize>() % (page_width - 1) + 1;
    let val_len = page_width - key_len;

    let page = (0..page_height)
        .map(|_| {
            (0..page_width)
                .map(|_| rng.gen::<u32>())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    // Generating random sorted timestamps for operations
    let mut clks: Vec<usize> = (0..num_ops).map(|_| rng.gen::<usize>()).collect();
    clks.sort();

    let mut all_keys = HashMap::new();
    for (i, row) in page.iter().enumerate() {
        let key = row[..key_len].to_vec().clone();
        all_keys.insert(key, i);
    }

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
                page[all_keys[&key]][key_len..].to_vec()
            } else {
                (0..val_len).map(|_| rng.gen::<u32>()).collect()
            }
        };

        ops.push(Operation::new(clk, key, val, op_type));
    }

    let mut checker_controller: CheckerController<BabyBearPoseidon2Config> =
        CheckerController::new(bus_index);
    let engine = config::baby_bear_poseidon2::default_engine(log_page_height.max(log_num_ops));

    checker_controller.load_page_and_ops(page, key_len, val_len, ops);
}
