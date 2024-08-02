use afs_stark_backend::rap::AnyRap;
use afs_test_utils::config::fri_params::{
    fri_params_fast_testing, fri_params_with_80_bits_of_security,
};
use itertools::izip;
use p3_baby_bear::BabyBear;
use std::ops::Deref;

use stark_vm::vm::config::VmConfig;
use stark_vm::vm::{ExecutionResult, VirtualMachine};

use crate::common::{fibonacci_program, sort_chips_mut};
use afs_recursion::stark::get_rec_raps_by_type;

mod common;

#[ignore = "test is too slow (6min on local machine)"]
#[test]
fn test_fibonacci_program_continuations_verify() {
    // original program being run
    let fib_program = fibonacci_program(0, 1, 32);
    let vm_config = VmConfig {
        max_segment_len: 100,
        num_public_values: 2,
        ..Default::default()
    };
    let vm = VirtualMachine::<1, _>::new(vm_config, fib_program, vec![]);
    let ExecutionResult {
        nonempty_traces: mut traces,
        nonempty_chips: chips,
        nonempty_pis: mut pvs,
        nonempty_types: chip_types,
        ..
    } = vm.execute().unwrap();

    // 3 segments are created
    assert_eq!(chips.len(), 3);

    // get rec_raps
    let mut dummy_vm = VirtualMachine::<1, BabyBear>::new(vm_config, vec![], vec![]);
    for _ in &chip_types {
        dummy_vm.segment(Default::default());
    }
    let mut rec_raps = get_rec_raps_by_type(&chip_types, &dummy_vm.segments);

    let mut chips: Vec<Vec<&dyn AnyRap<_>>> = chips
        .iter()
        .map(|x| x.iter().map(|y| y.deref()).collect())
        .collect();

    // sort objects by trace height in each segment
    for (chips, rec_raps, traces, pvs) in izip!(
        chips.iter_mut(),
        rec_raps.iter_mut(),
        traces.iter_mut(),
        pvs.iter_mut()
    ) {
        sort_chips_mut(chips, rec_raps, traces, pvs);
    }

    // blowup factor = 3
    let fri_params = if matches!(std::env::var("AXIOM_FAST_TEST"), Ok(x) if &x == "1") {
        fri_params_fast_testing()[1]
    } else {
        fri_params_with_80_bits_of_security()[1]
    };
    let mut all_vparams = vec![];
    for (chips, traces, pvs) in izip!(chips, traces, pvs.clone()) {
        let vparams = common::make_verification_params(&chips, traces, &pvs, fri_params);
        all_vparams.push(vparams);
    }

    // get pairs
    let rec_raps_arr = [rec_raps.remove(0), rec_raps.remove(0)];
    let pvs_arr = [pvs.remove(0), pvs.remove(0)];
    let vparams_arr = [all_vparams.remove(0), all_vparams.remove(0)];

    // aggregate first two segments
    let (fib_verification_program, input_stream) = common::build_continuations_verification_program(
        rec_raps_arr,
        pvs_arr,
        vparams_arr,
        false,
        false,
    );

    let vm_config = VmConfig {
        max_segment_len: 6000000,
        num_public_values: 2,
        ..Default::default()
    };

    // collect aggregation results
    let vm = VirtualMachine::<1, _>::new(vm_config, fib_verification_program, input_stream);
    let ExecutionResult {
        nonempty_traces: mut agg_traces,
        nonempty_chips: agg_chips,
        nonempty_pis: mut agg_pvs,
        nonempty_types: agg_chip_types,
        ..
    } = vm.execute().unwrap();

    // setup for final aggregation
    let dummy_vm = VirtualMachine::<1, BabyBear>::new(vm_config, vec![], vec![]);
    let mut agg_chips: Vec<Vec<&dyn AnyRap<_>>> = agg_chips
        .iter()
        .map(|x| x.iter().map(|y| y.deref()).collect())
        .collect();
    let mut agg_rec_raps = get_rec_raps_by_type(&agg_chip_types, &dummy_vm.segments);
    for (chips, rec_raps, traces, pvs) in izip!(
        agg_chips.iter_mut(),
        agg_rec_raps.iter_mut(),
        agg_traces.iter_mut(),
        agg_pvs.iter_mut()
    ) {
        sort_chips_mut(chips, rec_raps, traces, pvs);
    }

    // arranging objects in the right order
    let vparams = common::make_verification_params(
        &agg_chips[0],
        agg_traces.remove(0),
        &agg_pvs[0],
        fri_params,
    );
    all_vparams.insert(0, vparams);
    agg_pvs.extend(pvs);
    pvs = agg_pvs;
    agg_rec_raps.extend(rec_raps);
    rec_raps = agg_rec_raps;

    assert_eq!(pvs.len(), 2);
    assert_eq!(rec_raps.len(), 2);
    assert_eq!(all_vparams.len(), 2);

    let rec_raps_arr = [rec_raps.remove(0), rec_raps.remove(0)];
    let pvs_arr = [pvs.remove(0), pvs.remove(0)];
    let vparams_arr = [all_vparams.remove(0), all_vparams.remove(0)];

    // final aggregation
    let (fib_verification_program, input_stream) = common::build_continuations_verification_program(
        rec_raps_arr,
        pvs_arr,
        vparams_arr,
        true,
        false,
    );
    let vm = VirtualMachine::<1, _>::new(vm_config, fib_verification_program, input_stream);
    vm.execute().unwrap();
}
