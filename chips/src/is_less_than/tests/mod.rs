use std::sync::Arc;

use crate::range_gate::RangeCheckerGateChip;

use super::super::is_less_than::IsLessThanChip;

use afs_stark_backend::prover::USE_DEBUG_BUILDER;
use afs_stark_backend::verifier::VerificationError;
use afs_test_utils::config::baby_bear_poseidon2::run_simple_test_no_pis;
use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use p3_matrix::dense::DenseMatrix;

#[test]
fn test_is_less_than_chip_lt() {
    let bus_index: usize = 0;
    let limb_bits: usize = 16;
    let decomp: usize = 8;
    let range_max: u32 = 1 << decomp;

    let range_checker = Arc::new(RangeCheckerGateChip::new(bus_index, range_max));

    let chip = IsLessThanChip::new(bus_index, range_max, limb_bits, decomp, range_checker);
    let trace = chip.generate_trace(14321, 26883);
    let range_trace: DenseMatrix<BabyBear> = chip.range_checker.generate_trace();

    run_simple_test_no_pis(
        vec![&chip.air, chip.range_checker.as_ref()],
        vec![trace, range_trace],
    )
    .expect("Verification failed");
}

#[test]
fn test_is_less_than_chip_gt() {
    let bus_index: usize = 0;
    let limb_bits: usize = 16;
    let decomp: usize = 8;
    let range_max: u32 = 1 << decomp;

    let range_checker = Arc::new(RangeCheckerGateChip::new(bus_index, range_max));

    let chip = IsLessThanChip::new(bus_index, range_max, limb_bits, decomp, range_checker);
    let trace = chip.generate_trace(1, 0);
    let range_trace: DenseMatrix<BabyBear> = chip.range_checker.generate_trace();

    run_simple_test_no_pis(
        vec![&chip.air, chip.range_checker.as_ref()],
        vec![trace, range_trace],
    )
    .expect("Verification failed");
}

#[test]
fn test_is_less_than_chip_eq() {
    let bus_index: usize = 0;
    let limb_bits: usize = 16;
    let decomp: usize = 8;
    let range_max: u32 = 1 << decomp;

    let range_checker = Arc::new(RangeCheckerGateChip::new(bus_index, range_max));

    let chip = IsLessThanChip::new(bus_index, range_max, limb_bits, decomp, range_checker);
    let trace = chip.generate_trace(773, 773);
    let range_trace: DenseMatrix<BabyBear> = chip.range_checker.generate_trace();

    run_simple_test_no_pis(
        vec![&chip.air, chip.range_checker.as_ref()],
        vec![trace, range_trace],
    )
    .expect("Verification failed");
}

#[test]
fn test_is_less_than_negative() {
    let bus_index: usize = 0;
    let limb_bits: usize = 16;
    let decomp: usize = 8;
    let range_max: u32 = 1 << decomp;

    let range_checker = Arc::new(RangeCheckerGateChip::new(bus_index, range_max));

    let chip = IsLessThanChip::new(bus_index, range_max, limb_bits, decomp, range_checker);
    let mut trace = chip.generate_trace(446, 553);
    let range_trace = chip.range_checker.generate_trace();

    trace.values[2] = AbstractField::from_canonical_u64(0);

    USE_DEBUG_BUILDER.with(|debug| {
        *debug.lock().unwrap() = false;
    });
    assert_eq!(
        run_simple_test_no_pis(
            vec![&chip.air, chip.range_checker.as_ref()],
            vec![trace, range_trace],
        ),
        Err(VerificationError::OodEvaluationMismatch),
        "Expected verification to fail, but it passed"
    );
}
