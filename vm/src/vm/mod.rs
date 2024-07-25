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

    // NOT PUBLIC by design, adjust only for testing
    max_len: usize,
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
        VirtualMachineState {
            state: self.segments.last().unwrap().cpu_chip.state,
            memory: self.segments.last().unwrap().memory_chip.get_memory(),
            input_stream: self.segments.last().unwrap().input_stream.clone(),
            hint_stream: self.segments.last().unwrap().hint_stream.clone(),
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

    /// Executes the VM by calling `ExecutionSegment::generate_traces()` until the CPU hits `TERMINATE`
    /// and `cpu_chip.is_done`. Between every segment, the VM will call `generate_commitments()` and then
    /// `next_segment()`.
    pub fn execute(&mut self) -> Result<(), ExecutionError> {
        let mut result = vec![];
        loop {
            result.extend(self.segments.last_mut().unwrap().generate_traces()?);
            result.extend(self.segments.last_mut().unwrap().generate_commitments()?);
            if self.segments.last_mut().unwrap().cpu_chip.state.is_done {
                break;
            }
            self.next_segment();
        }
        self.traces = result;
        let prog_height = self.traces[1].height();
        let range_checker_height = self.traces[3].height();
        for trace in &self.traces {
            assert!(
                (trace.height() <= (self.max_len + 1).next_power_of_two())
                    || (trace.height() == prog_height)
                    || (trace.height() == range_checker_height),
                "Trace height exceeds max_len"
            );
        }
        Ok(())
    }

    /// Retrieves the non-empty traces from the VM.
    pub fn get_traces(&self) -> Vec<DenseMatrix<F>> {
        self.traces
            .clone()
            .into_iter()
            .filter(|trace| !trace.values.is_empty())
            .collect()
    }

    /// Retrieves the maximum log degree of the VM's trace.
    pub fn max_log_degree(&self) -> Result<usize, ExecutionError> {
        let mut checker_trace_degree = 0;
        for trace in &self.traces {
            checker_trace_degree = std::cmp::max(checker_trace_degree, trace.height());
        }
        Ok(log2_strict_usize(checker_trace_degree))
    }

    /// Retrieves the public inputs from the VM's segments, filtering by nonempty traces.
    pub fn get_pis(&self) -> Vec<Vec<F>> {
        let initial: Vec<Vec<F>> = self
            .segments
            .iter()
            .flat_map(|segment| segment.get_pis())
            .collect();

        initial
            .into_iter()
            .zip(self.traces.iter())
            .filter(|(_, trace)| !trace.values.is_empty())
            .map(|(initial_elem, _)| initial_elem)
            .collect()
    }
}

/// Retrieves all chips from the VM's segments, filtering by nonempty traces. Cannot be made into
/// struct method because of generics.
pub fn get_all_chips<const WORD_SIZE: usize, SC: StarkGenericConfig>(
    vm: &VirtualMachine<WORD_SIZE, Val<SC>>,
) -> Vec<&dyn AnyRap<SC>>
where
    Val<SC>: PrimeField32,
{
    let chips: Vec<_> = vm
        .segments
        .iter()
        .flat_map(|segment| get_chips::<WORD_SIZE, SC>(segment))
        .collect();

    let chips: Vec<_> = vm
        .traces
        .iter()
        .zip(chips)
        .filter(|(trace, _)| !trace.values.is_empty())
        .map(|(_, chip)| chip)
        .collect();
    chips
}
