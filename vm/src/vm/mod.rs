use crate::cpu::ExecutionState;
use afs_stark_backend::rap::AnyRap;
use afs_test_utils::config::baby_bear_poseidon2::BabyBearPoseidon2Config;
use p3_baby_bear::BabyBear;
use p3_field::PrimeField32;
use p3_matrix::{dense::DenseMatrix, Matrix};
use p3_util::log2_strict_usize;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::Deref;

mod segment;
pub use segment::{get_chips, ExecutionSegment};

use crate::cpu::{
    trace::{ExecutionError, Instruction},
    CpuOptions,
};

pub mod cycle_tracker;

use self::config::VmConfig;

pub mod config;

/// Parent struct that holds all execution segments, program, config.
///
/// Key method is `vm.execute()` which consumes the VM and returns a `ExecutionResult` struct. Segment switching is handled by
/// `ExecutionSegment::should_segment()`, called every CPU clock cycle, which when `true`
///  triggers `VirtualMachine::next_segment()`.
///
/// Chips, traces, and public values should be retrieved by unpacking the `ExecutionResult` struct.
/// `VirtualMachine::get_chips()` can be used to convert the boxes of chips to concrete chips.
pub struct VirtualMachine<const WORD_SIZE: usize, F: PrimeField32> {
    pub config: VmConfig,
    pub program: Vec<Instruction<F>>,
    pub segments: Vec<ExecutionSegment<WORD_SIZE, F>>,
    pub traces: Vec<DenseMatrix<F>>,
}

/// Enum representing the different types of chips used in the VM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipType {
    Cpu,
    Program,
    Memory,
    RangeChecker,
    FieldArithmetic,
    FieldExtension,
    Poseidon2,
}

/// Struct that holds the return state of the VM. StarkConfig is hardcoded to BabyBearPoseidon2Config.
// pub struct ExecutionResult<const WORD_SIZE: usize> {
//     /// Traces of the VM
//     pub nonempty_traces: Vec<DenseMatrix<BabyBear>>,
//     pub all_traces: Vec<DenseMatrix<BabyBear>>,
//     /// Public inputs of the VM
//     pub nonempty_pis: Vec<Vec<BabyBear>>,
//     /// Boxed chips of the VM
//     pub nonempty_chips: Vec<Box<dyn AnyRap<BabyBearPoseidon2Config>>>,
//     pub unique_chips: Vec<Box<dyn AnyRap<BabyBearPoseidon2Config>>>,
//     /// Types of all nonempty chips
//     pub nonempty_types: Vec<ChipType>,
//     /// Maximum log degree of the VM
//     pub max_log_degree: usize,
//     pub unique_types: Vec<ChipType>,
// }

pub struct ExecutionResult<const WORD_SIZE: usize> {
    /// Traces of the VM
    pub nonempty_traces: Vec<Vec<DenseMatrix<BabyBear>>>,
    pub all_traces: Vec<Vec<DenseMatrix<BabyBear>>>,
    /// Public inputs of the VM
    pub nonempty_pis: Vec<Vec<Vec<BabyBear>>>,
    /// Boxed chips of the VM
    pub nonempty_chips: Vec<Vec<Box<dyn AnyRap<BabyBearPoseidon2Config>>>>,
    pub unique_chips: Vec<Box<dyn AnyRap<BabyBearPoseidon2Config>>>,
    /// Types of all nonempty chips
    pub nonempty_types: Vec<Vec<ChipType>>,
    /// Maximum log degree of the VM
    pub max_log_degree: usize,
    pub unique_types: Vec<ChipType>,
}

/// Struct that holds the current state of the VM. For now, includes memory, input stream, and hint stream.
/// Hint stream cannot be added to during execution, but must be copied because it is popped from.
#[derive(Clone, Default)]
pub struct VirtualMachineState<F: PrimeField32> {
    /// Current state of the CPU
    state: ExecutionState,
    /// Current memory of the CPU
    memory: HashMap<(F, F), F>,
    /// Input stream of the CPU
    input_stream: VecDeque<Vec<F>>,
    /// Hint stream of the CPU
    hint_stream: VecDeque<F>,
}

impl<const WORD_SIZE: usize, F: PrimeField32> VirtualMachine<WORD_SIZE, F> {
    /// Create a new VM with a given config, program, and input stream.
    ///
    /// The VM will start with a single segment, which is created from the initial state of the CPU.
    pub fn new(config: VmConfig, program: Vec<Instruction<F>>, input_stream: Vec<Vec<F>>) -> Self {
        let mut vm = Self {
            config,
            program,
            segments: vec![],
            traces: vec![],
        };
        vm.segment(VirtualMachineState {
            state: ExecutionState::default(),
            memory: HashMap::new(),
            input_stream: VecDeque::from(input_stream),
            hint_stream: VecDeque::new(),
        });
        vm
    }

    /// Create a new segment with a given state.
    ///
    /// The segment will be created from the given state and the program.
    pub fn segment(&mut self, state: VirtualMachineState<F>) {
        let program = self.program.clone();
        let segment = ExecutionSegment::new(self.config, program, state);
        self.segments.push(segment);
    }

    /// Retrieves the current state of the VM by querying the last segment.
    pub fn current_state(&self) -> VirtualMachineState<F> {
        let last_seg = self.segments.last().unwrap();
        VirtualMachineState {
            state: last_seg.cpu_chip.state,
            memory: last_seg.memory_chip.memory_clone(),
            input_stream: last_seg.input_stream.clone(),
            hint_stream: last_seg.hint_stream.clone(),
        }
    }

    /// Retrieves the CPU options from the VM's config.
    pub fn options(&self) -> CpuOptions {
        self.config.cpu_options()
    }
}

/// Executes the VM by calling `ExecutionSegment::generate_traces()` until the CPU hits `TERMINATE`
/// and `cpu_chip.is_done`. Between every segment, the VM will call `generate_commitments()` and then
/// `next_segment()`.
impl<const WORD_SIZE: usize> VirtualMachine<WORD_SIZE, BabyBear> {
    pub fn execute(mut self) -> Result<ExecutionResult<WORD_SIZE>, ExecutionError> {
        let mut traces = vec![];
        loop {
            let last_seg = self.segments.last_mut().unwrap();
            last_seg.has_generation_happened = true;
            let mut new_traces = vec![];
            new_traces.extend(last_seg.generate_traces()?);
            new_traces.extend(last_seg.generate_commitments()?);
            traces.push(new_traces);
            if last_seg.cpu_chip.state.is_done {
                break;
            }
            self.segment(self.current_state());
        }

        let mut pis = vec![];
        let mut chips = vec![];
        let mut types = vec![];
        let num_chips = self.segments[0].get_num_chips();

        let dummy_segment = ExecutionSegment::new(self.config, vec![], self.current_state());
        let unique_chips =
            get_chips::<WORD_SIZE, BabyBearPoseidon2Config>(dummy_segment, &vec![true; num_chips]);
        let dummy_segment: ExecutionSegment<WORD_SIZE, BabyBear> =
            ExecutionSegment::new(self.config, vec![], self.current_state());
        let unique_types = dummy_segment.get_types();

        // Iterate over each segment and add its public inputs, types, and chips to the result,
        // skipping empty traces.
        for (i, segment) in self.segments.into_iter().enumerate() {
            let trace_slice = &traces[i];
            let inclusion_mask = trace_slice
                .iter()
                .map(|trace| !trace.values.is_empty())
                .collect::<Vec<bool>>();

            let segment_pis = segment.get_pis();
            let segment_types = segment.get_types();
            let mut new_pis = vec![];
            let mut new_types = vec![];
            chips.push(get_chips::<WORD_SIZE, BabyBearPoseidon2Config>(
                segment,
                &inclusion_mask,
            ));
            for index in 0..inclusion_mask.len() {
                if inclusion_mask[index] {
                    new_pis.push(segment_pis[index].clone());
                    new_types.push(segment_types[index]);
                }
            }
            pis.push(new_pis);
            types.push(new_types);
        }

        let nonempty_traces = traces
            .clone()
            .into_iter()
            .map(|trace_vec| {
                trace_vec
                    .into_iter()
                    .filter(|trace| !trace.values.is_empty())
                    .collect::<Vec<DenseMatrix<BabyBear>>>()
            })
            .collect::<Vec<Vec<DenseMatrix<BabyBear>>>>();

        let max_log_degree = log2_strict_usize(
            nonempty_traces
                .concat()
                .iter()
                .map(|trace| trace.height())
                .max()
                .unwrap(),
        );

        // Assert that trace heights are within the max_len, except for Program and RangeChecker
        // +31 is needed because Poseidon2Permute adds 32 rows to memory at once
        traces
            .iter()
            .zip(types.iter())
            .for_each(|(sub_traces, sub_chip_types)| {
                sub_traces
                    .iter()
                    .zip(sub_chip_types)
                    .for_each(|(trace, chip_type)| {
                        if *chip_type != ChipType::Program && *chip_type != ChipType::RangeChecker {
                            assert!(
                                trace.height()
                                    <= (self.config.max_segment_len + 31).next_power_of_two(),
                                "Trace height for {:?} exceeds max_len. Height: {}, Max: {}",
                                chip_type,
                                trace.height(),
                                (self.config.max_segment_len + 31).next_power_of_two(),
                            );
                        }
                    });
            });

        let chip_data = ExecutionResult {
            nonempty_traces,
            all_traces: traces,
            nonempty_pis: pis,
            nonempty_chips: chips,
            unique_chips,
            nonempty_types: types,
            max_log_degree,
            unique_types,
        };

        Ok(chip_data)
    }

    /// Convert the VM's chips from Boxes to concrete types.
    pub fn get_chips(
        chips: &[Box<dyn AnyRap<BabyBearPoseidon2Config>>],
    ) -> Vec<&dyn AnyRap<BabyBearPoseidon2Config>> {
        chips.iter().map(|x| x.deref()).collect()
    }
}

impl<const WORD_SIZE: usize> ExecutionResult<WORD_SIZE> {
    pub fn get_chips(&self) -> Vec<Vec<&dyn AnyRap<BabyBearPoseidon2Config>>> {
        self.nonempty_chips
            .iter()
            .map(|x| x.iter().map(|y| y.deref()).collect())
            .collect()
    }
}
