use super::super::assert_sorted;

use afs_stark_backend::prover::USE_DEBUG_BUILDER;
use afs_stark_backend::rap::AnyRap;
use afs_stark_backend::verifier::VerificationError;
use afs_test_utils::config::baby_bear_poseidon2::run_simple_test_no_pis;
use assert_sorted::AssertSortedChip;
use p3_baby_bear::BabyBear;
use p3_matrix::dense::DenseMatrix;

/**
 * Testing strategy for the assert sorted chip:
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
 *     partition on size of each limb:
 *         each limb has at most limb_bits bits
 *         at least one limb has more than limb_bits bits
 *     partition on row order:
 *         rows are sorted lexicographically
 *         rows are not sorted lexicographically
 */

// covers limb_bits < 20, key_vec_len < 4, limb_bits % decomp == 0, number of rows < 4, each limb has at
// most limb_bits bits, rows are sorted lexicographically
#[test]
fn test_assert_sorted_chip_small_positive() {
    let bus_index: usize = 0;
    let limb_bits: Vec<usize> = vec![16, 16];
    let decomp: usize = 8;
    let key_vec_len: usize = 2;

    let range_max: u32 = 1 << decomp;

    let requests = vec![vec![7784, 35423], vec![17558, 44832]];

    let assert_sorted_chip = AssertSortedChip::new(
        bus_index,
        range_max,
        limb_bits,
        decomp,
        key_vec_len,
        requests.clone(),
    );

    let assert_sorted_chip_trace: DenseMatrix<BabyBear> = assert_sorted_chip.generate_trace();
    let assert_sorted_range_chip_trace: DenseMatrix<BabyBear> =
        assert_sorted_chip.range_checker_gate.generate_trace();

    let mut chips: Vec<&dyn AnyRap<_>> =
        vec![&assert_sorted_chip, &assert_sorted_chip.range_checker_gate];

    for is_less_than_chip in assert_sorted_chip
        .is_less_than_tuple_chip
        .is_less_than_chips
        .iter()
    {
        chips.push(&is_less_than_chip.range_checker_gate);
    }

    let mut traces = vec![assert_sorted_chip_trace, assert_sorted_range_chip_trace];

    for is_less_than_chip in assert_sorted_chip
        .is_less_than_tuple_chip
        .is_less_than_chips
        .iter()
    {
        let range_trace: DenseMatrix<BabyBear> =
            is_less_than_chip.range_checker_gate.generate_trace();
        traces.push(range_trace);
    }

    run_simple_test_no_pis(chips, traces).expect("Verification failed");
}

// covers limb_bits >= 20, key_vec_len >= 4, limb_bits % decomp != 0, number of rows >= 4, each limb has at
// most limb_bits bits, rows are sorted lexicographically
#[test]
fn test_assert_sorted_chip_large_positive() {
    let bus_index: usize = 0;
    let limb_bits: Vec<usize> = vec![30, 30, 30, 30];
    let decomp: usize = 8;
    let key_vec_len: usize = 4;

    let range_max: u32 = 1 << decomp;

    let requests = vec![
        vec![35867, 318434, 12786, 44832],
        vec![704210, 369315, 42421, 487111],
        vec![370183, 37202, 729789, 783571],
        vec![875005, 767547, 196209, 887921],
    ];

    let assert_sorted_chip = AssertSortedChip::new(
        bus_index,
        range_max,
        limb_bits,
        decomp,
        key_vec_len,
        requests.clone(),
    );

    let assert_sorted_chip_trace: DenseMatrix<BabyBear> = assert_sorted_chip.generate_trace();
    let assert_sorted_range_chip_trace: DenseMatrix<BabyBear> =
        assert_sorted_chip.range_checker_gate.generate_trace();

    let mut chips: Vec<&dyn AnyRap<_>> =
        vec![&assert_sorted_chip, &assert_sorted_chip.range_checker_gate];

    for is_less_than_chip in assert_sorted_chip
        .is_less_than_tuple_chip
        .is_less_than_chips
        .iter()
    {
        chips.push(&is_less_than_chip.range_checker_gate);
    }

    let mut traces = vec![assert_sorted_chip_trace, assert_sorted_range_chip_trace];

    for is_less_than_chip in assert_sorted_chip
        .is_less_than_tuple_chip
        .is_less_than_chips
        .iter()
    {
        let range_trace: DenseMatrix<BabyBear> =
            is_less_than_chip.range_checker_gate.generate_trace();
        traces.push(range_trace);
    }

    run_simple_test_no_pis(chips, traces).expect("Verification failed");
}

// covers limb_bits >= 20, key_vec_len >= 4, limb_bits % decomp != 0, number of rows >= 4, at least one limb
// has more than limb_bits bits, rows are sorted lexicographically
#[test]
fn test_assert_sorted_chip_largelimb_negative() {
    let bus_index: usize = 0;
    let limb_bits: Vec<usize> = vec![10, 10, 10, 10];
    let decomp: usize = 8;
    let key_vec_len: usize = 4;

    let range_max: u32 = 1 << decomp;

    // the first and second rows are not in sorted order
    let requests = vec![
        vec![223, 448, 15, 587],
        vec![883, 168, 772, 673],
        vec![57, 386, 1025, 694],
        vec![128, 767, 196, 953],
    ];

    let assert_sorted_chip = AssertSortedChip::new(
        bus_index,
        range_max,
        limb_bits,
        decomp,
        key_vec_len,
        requests.clone(),
    );

    let assert_sorted_chip_trace: DenseMatrix<BabyBear> = assert_sorted_chip.generate_trace();
    let assert_sorted_range_chip_trace: DenseMatrix<BabyBear> =
        assert_sorted_chip.range_checker_gate.generate_trace();

    let mut chips: Vec<&dyn AnyRap<_>> =
        vec![&assert_sorted_chip, &assert_sorted_chip.range_checker_gate];

    for is_less_than_chip in assert_sorted_chip
        .is_less_than_tuple_chip
        .is_less_than_chips
        .iter()
    {
        chips.push(&is_less_than_chip.range_checker_gate);
    }

    let mut traces = vec![assert_sorted_chip_trace, assert_sorted_range_chip_trace];

    for is_less_than_chip in assert_sorted_chip
        .is_less_than_tuple_chip
        .is_less_than_chips
        .iter()
    {
        let range_trace: DenseMatrix<BabyBear> =
            is_less_than_chip.range_checker_gate.generate_trace();
        traces.push(range_trace);
    }

    let result = run_simple_test_no_pis(chips, traces);

    assert_eq!(
        result,
        Err(VerificationError::NonZeroCumulativeSum),
        "Expected verification to fail, but it passed"
    );
}

// covers limb_bits >= 20, key_vec_len >= 4, limb_bits % decomp != 0, number of rows >= 4, each limb has at
// most limb_bits bits, rows are not sorted lexicographically
#[test]
fn test_assert_sorted_chip_unsorted_negative() {
    let bus_index: usize = 0;
    let limb_bits: Vec<usize> = vec![30, 30, 30, 30];
    let decomp: usize = 8;
    let key_vec_len: usize = 4;

    let range_max: u32 = 1 << decomp;

    // the first and second rows are not in sorted order
    let requests = vec![
        vec![704210, 369315, 42421, 44832],
        vec![35867, 318434, 12786, 44832],
        vec![370183, 37202, 729789, 783571],
        vec![875005, 767547, 196209, 887921],
    ];

    let assert_sorted_chip = AssertSortedChip::new(
        bus_index,
        range_max,
        limb_bits,
        decomp,
        key_vec_len,
        requests.clone(),
    );

    let assert_sorted_chip_trace: DenseMatrix<BabyBear> = assert_sorted_chip.generate_trace();
    let assert_sorted_range_chip_trace: DenseMatrix<BabyBear> =
        assert_sorted_chip.range_checker_gate.generate_trace();

    let mut chips: Vec<&dyn AnyRap<_>> =
        vec![&assert_sorted_chip, &assert_sorted_chip.range_checker_gate];

    for is_less_than_chip in assert_sorted_chip
        .is_less_than_tuple_chip
        .is_less_than_chips
        .iter()
    {
        chips.push(&is_less_than_chip.range_checker_gate);
    }

    let mut traces = vec![assert_sorted_chip_trace, assert_sorted_range_chip_trace];

    for is_less_than_chip in assert_sorted_chip
        .is_less_than_tuple_chip
        .is_less_than_chips
        .iter()
    {
        let range_trace: DenseMatrix<BabyBear> =
            is_less_than_chip.range_checker_gate.generate_trace();
        traces.push(range_trace);
    }

    USE_DEBUG_BUILDER.with(|debug| {
        *debug.lock().unwrap() = false;
    });
    assert_eq!(
        run_simple_test_no_pis(chips, traces),
        Err(VerificationError::OodEvaluationMismatch),
        "Expected verification to fail, but it passed"
    );
}
