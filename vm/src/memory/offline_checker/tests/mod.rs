use std::{collections::HashMap, iter, sync::Arc};

use afs_primitives::range_gate::RangeCheckerGateChip;
use afs_stark_backend::{prover::USE_DEBUG_BUILDER, verifier::VerificationError};
use afs_test_utils::{
    config::baby_bear_poseidon2::run_simple_test_no_pis,
    interaction::dummy_interaction_air::DummyInteractionAir,
};
use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use p3_matrix::dense::RowMajorMatrix;

use crate::cpu::{MEMORY_BUS, MEMORY_INTERACTION_BUS, RANGE_CHECKER_BUS};

use crate::memory::offline_checker::columns::MemoryOfflineCheckerCols;
use crate::memory::offline_checker::{MemoryAccess, MemoryChip, OpType};

use super::utils::gen_dummy_oc_interaction_trace;

const WORD_SIZE: usize = 3;
const ADDR_SPACE_LIMB_BITS: usize = 8;
const POINTER_LIMB_BITS: usize = 8;
const CLK_LIMB_BITS: usize = 8;
const DECOMP: usize = 4;
const RANGE_MAX: u32 = 1 << DECOMP;
const IDX_LEN: usize = 2;
const DATA_LEN: usize = 3;

const TRACE_DEGREE: usize = 16;

#[test]
fn test_flatten_fromslice_roundtrip() {
    let memory_chip = MemoryChip::<WORD_SIZE, BabyBear>::new(
        ADDR_SPACE_LIMB_BITS,
        POINTER_LIMB_BITS,
        CLK_LIMB_BITS,
        DECOMP,
        HashMap::new(),
    );

    let num_cols = MemoryOfflineCheckerCols::<usize>::width(&memory_chip.air);
    let all_cols = (0..num_cols).collect::<Vec<usize>>();

    let cols_numbered = MemoryOfflineCheckerCols::<usize>::from_slice(&all_cols, &memory_chip.air);
    let flattened = cols_numbered.flatten();

    assert_eq!(flattened, all_cols);

    assert_eq!(num_cols, flattened.len());
}

fn gen_requester_trace(
    ops: &[MemoryAccess<WORD_SIZE, BabyBear>],
    width: usize,
) -> RowMajorMatrix<BabyBear> {
    RowMajorMatrix::new(
        ops.iter()
            .flat_map(|op: &MemoryAccess<WORD_SIZE, BabyBear>| {
                [
                    BabyBear::one(),
                    BabyBear::from_canonical_usize(op.timestamp),
                    BabyBear::from_canonical_u8(op.op_type as u8),
                    op.address_space,
                    op.address,
                ]
                .into_iter()
                .chain(op.data.iter().cloned())
            })
            .chain(
                iter::repeat_with(|| iter::repeat(BabyBear::zero()).take(width))
                    .take(TRACE_DEGREE - ops.len())
                    .flatten(),
            )
            .collect(),
        width,
    )
}

#[test]
fn test_offline_checker() {
    let range_checker = Arc::new(RangeCheckerGateChip::new(RANGE_CHECKER_BUS, RANGE_MAX));
    let mut memory_chip = MemoryChip::new(
        ADDR_SPACE_LIMB_BITS,
        POINTER_LIMB_BITS,
        CLK_LIMB_BITS,
        DECOMP,
        HashMap::new(),
    );
    let requester = DummyInteractionAir::new(
        2 + memory_chip.air.offline_checker.idx_data_width(),
        true,
        MEMORY_BUS,
    );
    let dummy_oc_interaction_air =
        DummyInteractionAir::new(1 + IDX_LEN + DATA_LEN, true, MEMORY_INTERACTION_BUS);

    let ops: Vec<MemoryAccess<WORD_SIZE, BabyBear>> = vec![
        MemoryAccess {
            timestamp: 1,
            op_type: OpType::Write,
            address_space: BabyBear::one(),
            address: BabyBear::one(),
            data: [
                BabyBear::from_canonical_usize(232),
                BabyBear::from_canonical_usize(888),
                BabyBear::from_canonical_usize(5954),
            ],
        },
        MemoryAccess {
            timestamp: 0,
            op_type: OpType::Write,
            address_space: BabyBear::one(),
            address: BabyBear::zero(),
            data: [
                BabyBear::from_canonical_usize(2324),
                BabyBear::from_canonical_usize(433),
                BabyBear::from_canonical_usize(1778),
            ],
        },
        MemoryAccess {
            timestamp: 4,
            op_type: OpType::Write,
            address_space: BabyBear::one(),
            address: BabyBear::zero(),
            data: [
                BabyBear::from_canonical_usize(231),
                BabyBear::from_canonical_usize(3883),
                BabyBear::from_canonical_usize(17),
            ],
        },
        MemoryAccess {
            timestamp: 2,
            op_type: OpType::Read,
            address_space: BabyBear::one(),
            address: BabyBear::one(),
            data: [
                BabyBear::from_canonical_usize(232),
                BabyBear::from_canonical_usize(888),
                BabyBear::from_canonical_usize(5954),
            ],
        },
        MemoryAccess {
            timestamp: 6,
            op_type: OpType::Read,
            address_space: BabyBear::two(),
            address: BabyBear::zero(),
            data: [
                BabyBear::from_canonical_usize(4382),
                BabyBear::from_canonical_usize(8837),
                BabyBear::from_canonical_usize(192),
            ],
        },
        MemoryAccess {
            timestamp: 5,
            op_type: OpType::Write,
            address_space: BabyBear::two(),
            address: BabyBear::zero(),
            data: [
                BabyBear::from_canonical_usize(4382),
                BabyBear::from_canonical_usize(8837),
                BabyBear::from_canonical_usize(192),
            ],
        },
        MemoryAccess {
            timestamp: 3,
            op_type: OpType::Write,
            address_space: BabyBear::one(),
            address: BabyBear::one(),
            data: [
                BabyBear::from_canonical_usize(3243),
                BabyBear::from_canonical_usize(3214),
                BabyBear::from_canonical_usize(6639),
            ],
        },
    ];
    let mut ops_sorted = ops.clone();
    ops_sorted.sort_by_key(|op| op.timestamp);

    for op in ops_sorted.iter() {
        match op.op_type {
            OpType::Read => {
                assert_eq!(
                    memory_chip.read_word(op.timestamp, op.address_space, op.address),
                    op.data
                );
            }
            OpType::Write => {
                memory_chip.write_word(op.timestamp, op.address_space, op.address, op.data);
            }
        }
    }

    let trace = memory_chip.generate_trace(range_checker.clone());
    let range_checker_trace = range_checker.generate_trace();
    let requester_trace = gen_requester_trace(&memory_chip.accesses, requester.field_width() + 1);
    let dummy_oc_interaction_trace = gen_dummy_oc_interaction_trace(
        &mut memory_chip.accesses,
        dummy_oc_interaction_air.field_width() + 1,
    );

    run_simple_test_no_pis(
        vec![
            &memory_chip.air,
            &range_checker.air,
            &requester,
            &dummy_oc_interaction_air,
        ],
        vec![
            trace,
            range_checker_trace,
            requester_trace,
            dummy_oc_interaction_trace,
        ],
    )
    .expect("Verification failed");
}

#[test]
fn test_offline_checker_valid_first_read() {
    let range_checker = Arc::new(RangeCheckerGateChip::new(RANGE_CHECKER_BUS, RANGE_MAX));
    let mut memory_chip = MemoryChip::new(
        ADDR_SPACE_LIMB_BITS,
        POINTER_LIMB_BITS,
        CLK_LIMB_BITS,
        DECOMP,
        HashMap::new(),
    );
    let requester = DummyInteractionAir::new(
        2 + memory_chip.air.offline_checker.idx_data_width(),
        true,
        MEMORY_BUS,
    );
    let dummy_oc_interaction_air =
        DummyInteractionAir::new(1 + IDX_LEN + DATA_LEN, true, MEMORY_INTERACTION_BUS);

    memory_chip.write_word(
        0,
        BabyBear::one(),
        BabyBear::zero(),
        [BabyBear::zero(), BabyBear::zero(), BabyBear::zero()],
    );
    // read before writing, but first operation in block so should pass
    memory_chip.accesses[0].op_type = OpType::Read;

    let memory_trace = memory_chip.generate_trace(range_checker.clone());
    let range_checker_trace = range_checker.generate_trace();
    let requester_trace = gen_requester_trace(&memory_chip.accesses, requester.field_width() + 1);
    let dummy_oc_interaction_trace = gen_dummy_oc_interaction_trace(
        &mut memory_chip.accesses,
        dummy_oc_interaction_air.field_width() + 1,
    );

    run_simple_test_no_pis(
        vec![
            &memory_chip.air,
            &range_checker.air,
            &requester,
            &dummy_oc_interaction_air,
        ],
        vec![
            memory_trace,
            range_checker_trace,
            requester_trace,
            dummy_oc_interaction_trace,
        ],
    )
    .expect("Verification failed");
}

#[test]
fn test_offline_checker_negative_data_mismatch() {
    let range_checker = Arc::new(RangeCheckerGateChip::new(RANGE_CHECKER_BUS, RANGE_MAX));
    let mut memory_chip = MemoryChip::new(
        ADDR_SPACE_LIMB_BITS,
        POINTER_LIMB_BITS,
        CLK_LIMB_BITS,
        DECOMP,
        HashMap::new(),
    );
    let requester = DummyInteractionAir::new(
        2 + memory_chip.air.offline_checker.idx_data_width(),
        true,
        MEMORY_BUS,
    );
    let dummy_oc_interaction_air =
        DummyInteractionAir::new(1 + IDX_LEN + DATA_LEN, true, MEMORY_INTERACTION_BUS);

    let ops: Vec<MemoryAccess<WORD_SIZE, BabyBear>> = vec![
        MemoryAccess {
            timestamp: 0,
            op_type: OpType::Write,
            address_space: BabyBear::one(),
            address: BabyBear::zero(),
            data: [
                BabyBear::from_canonical_usize(2324),
                BabyBear::from_canonical_usize(433),
                BabyBear::from_canonical_usize(1778),
            ],
        },
        MemoryAccess {
            timestamp: 1,
            op_type: OpType::Write,
            address_space: BabyBear::one(),
            address: BabyBear::one(),
            data: [
                BabyBear::from_canonical_usize(232),
                BabyBear::from_canonical_usize(888),
                BabyBear::from_canonical_usize(5954),
            ],
        },
        // data read does not match write from previous operation
        MemoryAccess {
            timestamp: 2,
            op_type: OpType::Read,
            address_space: BabyBear::one(),
            address: BabyBear::one(),
            data: [
                BabyBear::from_canonical_usize(233),
                BabyBear::from_canonical_usize(888),
                BabyBear::from_canonical_usize(5954),
            ],
        },
    ];

    memory_chip.accesses.clone_from(&ops);

    let trace = memory_chip.generate_trace(range_checker.clone());

    let range_checker_trace = range_checker.generate_trace();
    let requester_trace = gen_requester_trace(&memory_chip.accesses, requester.field_width() + 1);
    let dummy_oc_interaction_trace = gen_dummy_oc_interaction_trace(
        &mut memory_chip.accesses,
        dummy_oc_interaction_air.field_width() + 1,
    );

    USE_DEBUG_BUILDER.with(|debug| {
        *debug.lock().unwrap() = false;
    });
    assert_eq!(
        run_simple_test_no_pis(
            vec![
                &memory_chip.air,
                &range_checker.air,
                &requester,
                &dummy_oc_interaction_air,
            ],
            vec![
                trace,
                range_checker_trace,
                requester_trace,
                dummy_oc_interaction_trace
            ],
        ),
        Err(VerificationError::OodEvaluationMismatch),
        "Expected verification to fail, but it passed"
    );
}
