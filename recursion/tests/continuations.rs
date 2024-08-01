use afs_compiler::prelude::*;
use p3_baby_bear::BabyBear;
use p3_field::PrimeField32;
use std::ops::Deref;

use afs_recursion::stark::DynRapForRecursion;
use afs_stark_backend::rap::AnyRap;
use stark_vm::vm::config::VmConfig;
use stark_vm::vm::{ChipType, ExecutionResult, ExecutionSegment, VirtualMachine};

use crate::common::{fibonacci_program, sort_chips_with_id};

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
    let rec_raps = get_rec_raps(&chip_types, dummy_vm.segments);

    let mut split_traces = vec![];
    let mut split_chips: Vec<Vec<&dyn AnyRap<_>>> = vec![];
    let mut split_pvs = vec![];
    let mut split_rec_raps = vec![];

    let mut chip_ids = vec![];
    let mut segment_index = 0;
    for &chip_type in &chip_types {
        let index = unique_types.iter().position(|&x| x == chip_type).unwrap();
        if chip_type == ChipType::Program {
            chip_ids.push(vec![index, segment_index]);
            segment_index += 1;
        } else {
            chip_ids.push(vec![index]);
        }
    }
    let chips = chips.iter().map(|x| x.deref()).collect();
    let (chips, rec_raps, traces, pvs, chip_ids) =
        sort_chips_with_id(chips, rec_raps, traces, pvs, chip_ids);

    let vparams = common::make_verification_params(&chips, traces, &pvs);

    let (fib_verification_program, input_stream) = common::build_continuations_verification_program(
        [split_rec_raps[0], split_rec_raps[1]],
        [split_pvs[0], split_pvs[1]],
        [vparams, vparams],
    );

    let vm = VirtualMachine::<1, _>::new(vm_config, fib_verification_program, input_stream);
    vm.execute().unwrap();
}

pub fn get_rec_raps<C: Config>(
    chip_types: &[ChipType],
    segments: Vec<ExecutionSegment<1, BabyBear>>,
) -> Vec<&dyn DynRapForRecursion<C>>
where
    C::F: PrimeField32,
{
    let mut result: Vec<&dyn DynRapForRecursion<C>> = vec![];
    for (i, chip_type) in chip_types.iter().enumerate() {
        match chip_type {
            ChipType::Cpu => result.push(&segments[i].cpu_chip.air),
            ChipType::Program => result.push(&segments[i].program_chip.air),
            ChipType::Memory => result.push(&segments[i].memory_chip.air),
            ChipType::RangeChecker => result.push(&segments[i].range_checker.air),
            ChipType::FieldArithmetic => result.push(&segments[i].field_arithmetic_chip.air),
            ChipType::FieldExtension => result.push(&segments[i].field_extension_chip.air),
            ChipType::Poseidon2 => result.push(&segments[i].poseidon2_chip.air),
        }
    }
    result
}
