use afs_compiler::prelude::*;
use p3_field::PrimeField32;
use std::ops::Deref;

use afs_stark_backend::rap::AnyRap
use afs_recursion::stark::DynRapForRecursion;
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

    let dummy_vm = VirtualMachine::<1, _>::new(vm_config, fib_program.clone(), vec![]);
    let unique_rec_raps = get_rec_raps(&dummy_vm.segments[0]);
    let unique_types = dummy_vm.segments[0].get_types();

    let vm = VirtualMachine::<1, _>::new(vm_config, fib_program, vec![]);
    let ExecutionResult {
        nonempty_traces: traces,
        nonempty_chips: chips,
        nonempty_pis: pvs,
        chip_types,
        ..
    } = vm.execute().unwrap();

    let mut chip_ids = vec![];
    let mut segment_index = 0;
    for chip_type in &chip_types {
        let index = unique_types.iter().position(|&x| x == *chip_type).unwrap();
        if *chip_type == ChipType::Program {
            chip_ids.push(vec![index, segment_index]);
            segment_index += 1;
        } else {
            chip_ids.push(vec![index]);
        }
    }
    let chips = chips.iter().map(|x| x.deref()).collect();
    let (chips, unique_rec_raps, traces, pvs, chip_ids) =
        sort_chips_with_id(chips, unique_rec_raps, traces, pvs, chip_ids);

    let vparams = common::make_verification_params(&chips, traces, &pvs);

    let (fib_verification_program, input_stream) =
        common::build_continuations_verification_program(unique_rec_raps, pvs, vparams);

    // let vm = VirtualMachine::<1, _>::new(vm_config, fib_verification_program, input_stream);
    // vm.execute().unwrap();
}

pub fn get_rec_raps<const WORD_SIZE: usize, C: Config>(
    chip_types: &[ChipType],
) -> Vec<&dyn DynRapForRecursion<C>>
where
    C::F: PrimeField32,
{
    let mut result: Vec<&dyn DynRapForRecursion<C>> = vec![
        &vm.cpu_chip.air,
        &vm.program_chip.air,
        &vm.memory_chip.air,
        &vm.range_checker.air,
    ];
    if vm.options().field_arithmetic_enabled {
        result.push(&vm.field_arithmetic_chip.air);
    }
    if vm.options().field_extension_enabled {
        result.push(&vm.field_extension_chip.air);
    }
    if vm.options().poseidon2_enabled() {
        result.push(&vm.poseidon2_chip.air);
    }
    result
}
