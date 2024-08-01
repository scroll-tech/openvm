use afs_compiler::prelude::*;
use itertools::izip;
use p3_baby_bear::BabyBear;
use p3_field::PrimeField32;

use afs_recursion::stark::DynRapForRecursion;
use stark_vm::vm::config::VmConfig;
use stark_vm::vm::{ChipType, ExecutionResult, ExecutionSegment, VirtualMachine};

use crate::common::{fibonacci_program, sort_chips};

mod common;

#[test]
fn test_fibonacci_program_verify() {
    let fib_program = fibonacci_program(0, 1, 32);

    let vm_config = VmConfig {
        max_segment_len: 1000,
        ..Default::default()
    };

    let vm = VirtualMachine::<1, _>::new(vm_config, fib_program, vec![]);
    let ExecutionResult {
        nonempty_traces: traces,
        nonempty_chips: chips,
        nonempty_pis: pvs,
        nonempty_types: chip_types,
        unique_types,
        ..
    } = vm.execute().unwrap();

    let mut dummy_vm = VirtualMachine::<1, BabyBear>::new(vm_config, vec![], vec![]);
    for _ in &chip_types {
        dummy_vm.segment(Default::default());
    }
    let rec_raps = get_rec_raps(&chip_types, &dummy_vm.segments);

    let mut all_vparams = vec![];
    for (chips, rec_raps, traces, pvs) in izip!(chips, rec_raps, traces, pvs) {
        let chips = chips.iter().map(|x| x.deref()).collect();
        let (chips, rec_raps, traces, pvs) = sort_chips(chips, rec_raps, traces, pvs);
        let vparams = common::make_verification_params(&chips, traces, &pvs);
        all_vparams.push(vparams);
    }

    let (fib_verification_program, input_stream) = common::build_continuations_verification_program(
        [rec_raps[0], rec_raps[1]],
        [pvs[0], pvs[1]],
        [all_vparams[0], all_vparams[1]],
    );

    let vm = VirtualMachine::<1, _>::new(vm_config, fib_verification_program, input_stream);
    vm.execute().unwrap();
}

pub fn get_rec_raps<'a, C: Config>(
    chip_types: &[Vec<ChipType>],
    segments: &'a Vec<ExecutionSegment<1, BabyBear>>,
) -> Vec<Vec<&'a dyn DynRapForRecursion<C>>>
where
    C::F: PrimeField32,
{
    let mut result: Vec<Vec<&dyn DynRapForRecursion<C>>> = vec![];
    for (i, sub_chip_types) in chip_types.iter().enumerate() {
        let mut sub_result: Vec<&dyn DynRapForRecursion<C>> = vec![];
        for (j, chip_type) in sub_chip_types.iter().enumerate() {
            match chip_type {
                ChipType::Cpu => sub_result.push(&segments[i].cpu_chip.air),
                ChipType::Program => sub_result.push(&segments[i].program_chip.air),
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
