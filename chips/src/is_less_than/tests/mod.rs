use super::super::is_less_than::IsLessThanChip;

use afs_stark_backend::prover::USE_DEBUG_BUILDER;
use afs_stark_backend::verifier::VerificationError;
use afs_test_utils::config::baby_bear_poseidon2::run_simple_test_no_pis;
use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use p3_matrix::dense::DenseMatrix;

#[test]
fn test_is_less_than_chip_lt() {
    const BUS_INDEX: usize = 0;
    const LIMB_BITS: usize = 16;
    const DECOMP: usize = 8;
    const MAX: u32 = 1 << DECOMP;

    let chip = IsLessThanChip::new(BUS_INDEX, MAX, LIMB_BITS, DECOMP);
    let trace = chip.generate_trace(14321, 26883);
    let range_trace: DenseMatrix<BabyBear> = chip.range_checker_gate.generate_trace();

    run_simple_test_no_pis(
        vec![&chip, &chip.range_checker_gate],
        vec![trace, range_trace],
    )
    .expect("Verification failed");
}

#[test]
fn test_is_less_than_chip_gt() {
    const BUS_INDEX: usize = 0;
    const LIMB_BITS: usize = 16;
    const DECOMP: usize = 8;
    const MAX: u32 = 1 << DECOMP;

    let chip = IsLessThanChip::new(BUS_INDEX, MAX, LIMB_BITS, DECOMP);
    let trace = chip.generate_trace(1, 0);
    let range_trace: DenseMatrix<BabyBear> = chip.range_checker_gate.generate_trace();

    run_simple_test_no_pis(
        vec![&chip, &chip.range_checker_gate],
        vec![trace, range_trace],
    )
    .expect("Verification failed");
}

#[test]
fn test_is_less_than_chip_eq() {
    const BUS_INDEX: usize = 0;
    const LIMB_BITS: usize = 16;
    const DECOMP: usize = 8;
    const MAX: u32 = 1 << DECOMP;

    let chip = IsLessThanChip::new(BUS_INDEX, MAX, LIMB_BITS, DECOMP);
    let trace = chip.generate_trace(773, 773);
    let range_trace: DenseMatrix<BabyBear> = chip.range_checker_gate.generate_trace();

    run_simple_test_no_pis(
        vec![&chip, &chip.range_checker_gate],
        vec![trace, range_trace],
    )
    .expect("Verification failed");
}

#[test]
fn test_is_less_than_negative() {
    const BUS_INDEX: usize = 0;
    const LIMB_BITS: usize = 16;
    const DECOMP: usize = 8;
    const MAX: u32 = 1 << DECOMP;

    let chip = IsLessThanChip::new(BUS_INDEX, MAX, LIMB_BITS, DECOMP);
    let mut trace = chip.generate_trace(446, 553);
    let range_trace = chip.range_checker_gate.generate_trace();

    trace.values[2] = AbstractField::from_canonical_u64(0);

    USE_DEBUG_BUILDER.with(|debug| {
        *debug.lock().unwrap() = false;
    });
    assert_eq!(
        run_simple_test_no_pis(
            vec![&chip, &chip.range_checker_gate],
            vec![trace, range_trace],
        ),
        Err(VerificationError::OodEvaluationMismatch),
        "Expected verification to fail, but it passed"
    );
}
