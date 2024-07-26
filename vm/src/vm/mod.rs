use crate::cpu::ExecutionState;
use afs_stark_backend::rap::AnyRap;
use afs_test_utils::config::baby_bear_poseidon2::BabyBearPoseidon2Config;
use p3_baby_bear::BabyBear;
use p3_field::PrimeField32;
use p3_matrix::{dense::DenseMatrix, Matrix};
use p3_util::log2_strict_usize;
use std::collections::HashMap;
use std::collections::VecDeque;

mod segment;
pub use segment::{get_chips, ExecutionSegment};

use crate::cpu::{
    trace::{ExecutionError, Instruction},
    CpuOptions,
};

use self::config::VmConfig;

pub mod config;

pub const DEFAULT_MAX_LEN: usize = (1 << 20) - 100;

/// Parent struct that holds all execution segments, program, config.
///
/// Key method is `execute()` which runs the VM. Segment switching is handled by
/// `ExecutionSegment::should_segment()`, called every CPU clock cycle, which when `true`
///  triggers `VirtualMachine::next_segment()`.
///
/// Traces are stored in the VM, but chips, traces, and public values should be retrieved using
/// `get_chips()`, `get_traces()`, and `get_pis()` respectively.
pub struct VirtualMachine<const WORD_SIZE: usize, F: PrimeField32> {
    pub config: VmConfig,
    pub program: Vec<Instruction<F>>,
    pub segments: Vec<ExecutionSegment<WORD_SIZE, F>>,
    pub traces: Vec<DenseMatrix<F>>,

    max_len: usize,
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

pub struct ChipData<'a, const WORD_SIZE: usize> {
    pub traces: Vec<DenseMatrix<BabyBear>>,
    pub pis: Vec<Vec<BabyBear>>,
    pub chips: Vec<Box<dyn AnyRap<BabyBearPoseidon2Config> + 'a>>,
    pub chip_types: Vec<ChipType>,
    pub max_log_degree: usize,
}

/// Struct that holds the current state of the VM. For now, includes memory, input stream, and hint stream.
/// Hint stream cannot be added to during execution, but must be copied because it is popped from.
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
            max_len: DEFAULT_MAX_LEN,
        };
        vm.new_segment(VirtualMachineState {
            state: ExecutionState::default(),
            memory: HashMap::new(),
            input_stream: VecDeque::from(input_stream),
            hint_stream: VecDeque::new(),
        });
        vm
    }

    /// Sets the max length of the VM.
    pub fn adjust_max_len(&mut self, max_len: usize) {
        self.max_len = max_len;
        self.segments[0].adjust_max_len(max_len);
    }

    /// Create a new segment with a given state.
    ///
    /// The segment will be created from the given state and the program.
    pub fn new_segment(&mut self, state: VirtualMachineState<F>) {
        let program = self.program.clone();
        let segment = ExecutionSegment::new(self, program, state);
        self.segments.push(segment);
    }

    /// Retrieves the current state of the VM by querying the last segment.
    pub fn get_state(&self) -> VirtualMachineState<F> {
        let last_seg = self.segments.last().unwrap();
        VirtualMachineState {
            state: last_seg.cpu_chip.state,
            memory: last_seg.memory_chip.memory_clone(),
            input_stream: last_seg.input_stream.clone(),
            hint_stream: last_seg.hint_stream.clone(),
        }
    }

    /// Create a new segment with the current state of the VM.
    ///
    /// This is done by querying the last segment's state and creating a new segment from it.
    pub fn next_segment(&mut self) {
        let state = self.get_state();
        self.new_segment(state);
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
    pub fn execute<'a>(mut self) -> Result<ChipData<'a, WORD_SIZE>, ExecutionError> {
        let mut traces = vec![];
        loop {
            let last_seg = self.segments.last_mut().unwrap();
            traces.extend(last_seg.generate_traces()?);
            traces.extend(last_seg.generate_commitments()?);
            if last_seg.cpu_chip.state.is_done {
                break;
            }
            self.next_segment();
        }

        let mut pis = vec![];
        let mut chips: Vec<Box<dyn AnyRap<BabyBearPoseidon2Config>>> = vec![];
        let mut types = vec![];

        for (i, segment) in self.segments.into_iter().enumerate() {
            if !traces[i].values.is_empty() {
                pis.extend(segment.get_pis());
                types.extend(segment.get_types());
                chips.extend(get_chips::<WORD_SIZE, BabyBearPoseidon2Config>(segment));
            }
        }

        // Assert that trace heights are within the max_len, except for Program and RangeChecker
        // +31 is needed because Poseidon2Permute adds 32 rows to memory at once
        traces
            .iter()
            .zip(types.iter())
            .filter(|(_, chip_type)| {
                **chip_type != ChipType::Program && **chip_type != ChipType::RangeChecker
            })
            .for_each(|(trace, chip_type)| {
                assert!(
                    trace.height() <= (self.max_len + 31).next_power_of_two(),
                    "Trace height for {:?} exceeds max_len. Height: {}, Max: {}",
                    chip_type,
                    trace.height(),
                    self.max_len
                );
            });

        let max_log_degree =
            log2_strict_usize(traces.iter().map(|trace| trace.height()).max().unwrap());

        let chip_data = ChipData {
            traces,
            pis,
            chips,
            chip_types: types,
            max_log_degree,
        };

        Ok(chip_data)
    }

    pub fn get_chips(
        chips: &[Box<dyn AnyRap<BabyBearPoseidon2Config>>],
    ) -> Vec<&dyn AnyRap<BabyBearPoseidon2Config>> {
        chips.iter().map(|x| &**x).collect()
    }
}
