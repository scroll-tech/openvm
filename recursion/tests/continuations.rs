use afs_compiler::prelude::*;
use afs_stark_backend::rap::AnyRap;
use itertools::izip;
use p3_baby_bear::BabyBear;
use p3_field::PrimeField32;
use std::ops::Deref;

use afs_recursion::stark::DynRapForRecursion;
use stark_vm::vm::config::VmConfig;
use stark_vm::vm::{ChipType, ExecutionResult, ExecutionSegment, VirtualMachine};

use crate::common::{fibonacci_program, sort_chips_mut};

mod common;

#[test]
fn test_fibonacci_program_continuations_verify() {
    let fib_program = fibonacci_program(0, 1, 16);

    let vm_config = VmConfig {
        max_segment_len: 10,
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

    let mut dummy_vm = VirtualMachine::<1, BabyBear>::new(vm_config, vec![], vec![]);
    for _ in &chip_types {
        dummy_vm.segment(Default::default());
    }
    let mut rec_raps = get_rec_raps(&chip_types, &dummy_vm.segments);

    let mut all_vparams = vec![];
    let mut chips: Vec<Vec<&dyn AnyRap<_>>> = chips
        .iter()
        .map(|x| x.iter().map(|y| y.deref()).collect())
        .collect();

    for (chips, rec_raps, traces, pvs) in izip!(
        chips.iter_mut(),
        rec_raps.iter_mut(),
        traces.iter_mut(),
        pvs.iter_mut()
    ) {
        sort_chips_mut(chips, rec_raps, traces, pvs);
    }

    for (chips, traces, pvs) in izip!(chips, traces, pvs.clone()) {
        let vparams = common::make_verification_params(&chips, traces, &pvs);
        all_vparams.push(vparams);
    }

    let rec_raps_arr = [rec_raps.remove(0), rec_raps.remove(0)];
    let pvs_arr = [pvs.remove(0), pvs.remove(0)];
    let vparams_arr = [all_vparams.remove(0), all_vparams.remove(0)];

    let (fib_verification_program, input_stream) =
        common::build_continuations_verification_program(rec_raps_arr, pvs_arr, vparams_arr);

    let vm = VirtualMachine::<1, _>::new(vm_config, fib_verification_program, input_stream);
    vm.execute().unwrap();
}

pub fn get_rec_raps<'a, const WORD_SIZE: usize, C: Config>(
    chip_types: &[Vec<ChipType>],
    segments: &'a [ExecutionSegment<WORD_SIZE, C::F>],
) -> Vec<Vec<&'a dyn DynRapForRecursion<C>>>
where
    C::F: PrimeField32,
{
    let mut result: Vec<Vec<&dyn DynRapForRecursion<C>>> = vec![];

    for (i, sub_chip_types) in chip_types.iter().enumerate() {
        let mut sub_result: Vec<&dyn DynRapForRecursion<C>> = vec![];
        for chip_type in sub_chip_types {
            match chip_type {
                ChipType::Cpu => sub_result.push(&segments[i].cpu_chip.air),
                ChipType::Program => {
                    sub_result.push(&segments[i].program_chip.air as &dyn DynRapForRecursion<C>)
                }
                ChipType::Memory => sub_result.push(&segments[i].memory_chip.air),
                ChipType::RangeChecker => sub_result.push(&segments[i].range_checker.air),
                ChipType::FieldArithmetic => {
                    sub_result.push(&segments[i].field_arithmetic_chip.air)
                }
                ChipType::FieldExtension => sub_result.push(&segments[i].field_extension_chip.air),
                ChipType::Poseidon2 => sub_result.push(&segments[i].poseidon2_chip.air),
            }
        }
        result.push(sub_result);
    }
    result
}
