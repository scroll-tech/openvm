use crate::cpu::ExecutionState;
use afs_stark_backend::rap::AnyRap;
use p3_field::PrimeField32;
use p3_matrix::{dense::DenseMatrix, Matrix};
use p3_uni_stark::{StarkGenericConfig, Val};
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

pub struct ExecutionResult<const WORD_SIZE: usize, SC: StarkGenericConfig>
where
    Val<SC>: PrimeField32,
{
    segments: Vec<ExecutionSegment<WORD_SIZE, Val<SC>>>,
    traces: Vec<DenseMatrix<Val<SC>>>,
    pis: Vec<Vec<Val<SC>>>,
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

pub struct ChipData<'a, const WORD_SIZE: usize, SC: StarkGenericConfig> {
    pub traces: Vec<DenseMatrix<Val<SC>>>,
    pub pis: Vec<Vec<Val<SC>>>,
    pub chips: Vec<&'a dyn AnyRap<SC>>,
    pub chip_types: Vec<ChipType>,
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
pub fn execute<const WORD_SIZE: usize, SC: StarkGenericConfig>(
    mut vm: VirtualMachine<WORD_SIZE, Val<SC>>,
) -> Result<ExecutionResult<WORD_SIZE, SC>, ExecutionError>
where
    Val<SC>: PrimeField32,
{
    let mut traces = vec![];
    loop {
        let last_seg = vm.segments.last_mut().unwrap();
        traces.extend(last_seg.generate_traces()?);
        traces.extend(last_seg.generate_commitments()?);
        if last_seg.cpu_chip.state.is_done {
            break;
        }
        vm.next_segment();
    }
    let pis = vm
        .segments
        .iter()
        .flat_map(|segment| segment.get_pis())
        .collect::<Vec<_>>();

    Ok(ExecutionResult {
        segments: vm.segments,
        traces,
        pis,
        max_len: vm.max_len,
    })
}

impl<const WORD_SIZE: usize, SC: StarkGenericConfig> ExecutionResult<WORD_SIZE, SC>
where
    Val<SC>: PrimeField32,
{
    pub fn get_results(&self) -> ChipData<WORD_SIZE, SC> {
        let traces = self.traces.clone();
        let pis = self.pis.clone();
        let types = self
            .segments
            .iter()
            .flat_map(|segment| segment.get_types())
            .collect::<Vec<ChipType>>();
        let chips = self
            .segments
            .iter()
            .flat_map(|segment| get_chips::<WORD_SIZE, SC>(segment))
            .collect::<Vec<_>>();

        let non_empty_indices: Vec<usize> = traces
            .iter()
            .enumerate()
            .filter_map(|(i, trace)| (!trace.values.is_empty()).then_some(i))
            .collect();

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

        ChipData {
            traces: non_empty_indices
                .iter()
                .map(|&i| traces[i].clone())
                .collect(),
            pis: non_empty_indices.iter().map(|&i| pis[i].clone()).collect(),
            chips: non_empty_indices.iter().map(|&i| chips[i]).collect(),
            chip_types: non_empty_indices.iter().map(|&i| types[i]).collect(),
        }
    }

    /// Retrieves the maximum log degree of the VM's trace.
    pub fn max_log_degree(&self) -> usize {
        let mut checker_trace_degree = 0;
        for trace in &self.traces {
            checker_trace_degree = std::cmp::max(checker_trace_degree, trace.height());
        }
        log2_strict_usize(checker_trace_degree)
    }
}
