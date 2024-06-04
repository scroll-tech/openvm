use super::super::is_less_than_tuple::IsLessThanTupleChip;

use afs_stark_backend::prover::USE_DEBUG_BUILDER;
use afs_stark_backend::verifier::VerificationError;
use afs_test_utils::config::baby_bear_poseidon2::run_simple_test_no_pis;
use p3_field::AbstractField;

#[test]
fn test_is_less_than_tuple_chip_lt() {
    let bus_index: usize = 0;
    let limb_bits: Vec<usize> = vec![16, 8];
    let decomp: usize = 8;
    let range_max: u32 = 1 << decomp;
    let tuple_len: usize = 2;

    let chip = IsLessThanTupleChip::new(bus_index, range_max, limb_bits, decomp, tuple_len);
    let trace = chip.generate_trace(vec![14321, 123], vec![26678, 233]);

    run_simple_test_no_pis(vec![&chip], vec![trace]).expect("Verification failed");
}

#[test]
fn test_is_less_than_tuple_chip_gt() {
    let bus_index: usize = 0;
    let limb_bits: Vec<usize> = vec![16, 8];
    let decomp: usize = 8;
    let range_max: u32 = 1 << decomp;
    let tuple_len: usize = 2;

    let chip = IsLessThanTupleChip::new(bus_index, range_max, limb_bits, decomp, tuple_len);
    let trace = chip.generate_trace(vec![14321, 244], vec![26678, 233]);

    println!("{:?}", trace);

    run_simple_test_no_pis(vec![&chip], vec![trace]).expect("Verification failed");
}

#[test]
fn test_is_less_than_tuple_chip_eq() {
    let bus_index: usize = 0;
    let limb_bits: Vec<usize> = vec![16, 8];
    let decomp: usize = 8;
    let range_max: u32 = 1 << decomp;
    let tuple_len: usize = 2;

    let chip = IsLessThanTupleChip::new(bus_index, range_max, limb_bits, decomp, tuple_len);
    let trace = chip.generate_trace(vec![14321, 244], vec![14321, 244]);

    run_simple_test_no_pis(vec![&chip], vec![trace]).expect("Verification failed");
}

#[test]
fn test_is_less_than_tuple_chip_negative() {
    let bus_index: usize = 0;
    let limb_bits: Vec<usize> = vec![16, 8];
    let decomp: usize = 8;
    let range_max: u32 = 1 << decomp;
    let tuple_len: usize = 2;

    let chip = IsLessThanTupleChip::new(bus_index, range_max, limb_bits, decomp, tuple_len);
    let mut trace = chip.generate_trace(vec![14321, 123], vec![26678, 233]);

    trace.values[2] = AbstractField::from_canonical_u64(0);

    USE_DEBUG_BUILDER.with(|debug| {
        *debug.lock().unwrap() = false;
    });
    assert_eq!(
        run_simple_test_no_pis(vec![&chip], vec![trace],),
        Err(VerificationError::OodEvaluationMismatch),
        "Expected verification to fail, but it passed"
    );
}
