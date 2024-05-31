use super::super::less_than;

use afs_stark_backend::prover::USE_DEBUG_BUILDER;
use afs_stark_backend::verifier::VerificationError;
use afs_test_utils::config::baby_bear_poseidon2::run_simple_test_no_pis;
use p3_baby_bear::BabyBear;
use p3_matrix::dense::DenseMatrix;

/**
 * Testing strategy for the less than chip:
 *     partition on limb_bits:
 *         limb_bits < 20
 *         limb_bits >= 20
 *     partition on key_vec_len:
 *         key_vec_len < 4
 *         key_vec_len >= 4
 *     partition on decomp:
 *         limb_bits % decomp == 0
 *         limb_bits % decomp != 0
 *     partition on number of rows:
 *         number of rows < 4
 *         number of rows >= 4
 *     partition on row order:
 *         rows are sorted lexicographically
 *         rows are not sorted lexicographically
 */

// covers limb_bits < 20, key_vec_len < 4, limb_bits % decomp == 0, number of rows < 4, each limb has at
// most limb_bits bits, rows are sorted lexicographically
#[test]
fn test_less_than_chip_small_positive() {
    use less_than::LessThanChip;

    const BUS_INDEX: usize = 0;
    const LIMB_BITS: usize = 16;
    const DECOMP: usize = 8;
    const KEY_VEC_LEN: usize = 2;

    const MAX: u32 = 1 << DECOMP;

    let requests = vec![vec![7784, 35423], vec![17558, 44832]];

    let less_than_chip =
        LessThanChip::<MAX>::new(BUS_INDEX, LIMB_BITS, DECOMP, KEY_VEC_LEN, requests.clone());

    let less_than_chip_trace: DenseMatrix<BabyBear> = less_than_chip.generate_trace();
    let less_than_range_chip_trace: DenseMatrix<BabyBear> =
        less_than_chip.range_checker_gate.generate_trace();

    run_simple_test_no_pis(
        vec![&less_than_chip, &less_than_chip.range_checker_gate],
        vec![less_than_chip_trace, less_than_range_chip_trace],
    )
    .expect("Verification failed");
}

// covers limb_bits >= 20, key_vec_len >= 4, limb_bits % decomp != 0, number of rows >= 4, each limb has at
// most limb_bits bits, rows are sorted lexicographically
#[test]
fn test_less_than_chip_large_positive() {
    use less_than::LessThanChip;

    const BUS_INDEX: usize = 0;
    const LIMB_BITS: usize = 30;
    const DECOMP: usize = 8;
    const KEY_VEC_LEN: usize = 4;

    const MAX: u32 = 1 << DECOMP;

    let requests = vec![
        vec![35867, 318434, 12786, 44832],
        vec![704210, 369315, 42421, 487111],
        vec![370183, 37202, 729789, 783571],
        vec![875005, 767547, 196209, 887921],
    ];

    let less_than_chip =
        LessThanChip::<MAX>::new(BUS_INDEX, LIMB_BITS, DECOMP, KEY_VEC_LEN, requests.clone());

    let less_than_chip_trace: DenseMatrix<BabyBear> = less_than_chip.generate_trace();
    let less_than_range_chip_trace: DenseMatrix<BabyBear> =
        less_than_chip.range_checker_gate.generate_trace();

    run_simple_test_no_pis(
        vec![&less_than_chip, &less_than_chip.range_checker_gate],
        vec![less_than_chip_trace, less_than_range_chip_trace],
    )
    .expect("Verification failed");
}

// covers limb_bits >= 20, key_vec_len >= 4, limb_bits % decomp != 0, number of rows >= 4, each limb has at
// most limb_bits bits, rows are not sorted lexicographically
#[test]
fn test_less_than_chip_unsorted_negative() {
    use less_than::LessThanChip;

    const BUS_INDEX: usize = 0;
    const LIMB_BITS: usize = 30;
    const DECOMP: usize = 8;
    const KEY_VEC_LEN: usize = 4;

    const MAX: u32 = 1 << DECOMP;

    // the first and second rows are not in sorted order
    let requests = vec![
        vec![704210, 369315, 42421, 44832],
        vec![35867, 318434, 12786, 44832],
        vec![370183, 37202, 729789, 783571],
        vec![875005, 767547, 196209, 887921],
    ];

    let less_than_chip =
        LessThanChip::<MAX>::new(BUS_INDEX, LIMB_BITS, DECOMP, KEY_VEC_LEN, requests.clone());

    let less_than_chip_trace: DenseMatrix<BabyBear> = less_than_chip.generate_trace();
    let less_than_range_chip_trace: DenseMatrix<BabyBear> =
        less_than_chip.range_checker_gate.generate_trace();

    USE_DEBUG_BUILDER.with(|debug| {
        *debug.lock().unwrap() = false;
    });
    assert_eq!(
        run_simple_test_no_pis(
            vec![&less_than_chip, &less_than_chip.range_checker_gate,],
            vec![less_than_chip_trace, less_than_range_chip_trace],
        ),
        Err(VerificationError::OodEvaluationMismatch),
        "Expected verification to fail, but it passed"
    );
}
