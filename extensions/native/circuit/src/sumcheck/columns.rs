use openvm_circuit::system::memory::offline_checker::{MemoryReadAuxCols, MemoryWriteAuxCols};
use openvm_circuit_primitives_derive::AlignedBorrow;

use crate::{field_extension::EXT_DEG, utils::const_max};

const fn max3(a: usize, b: usize, c: usize) -> usize {
    const_max(a, const_max(b, c))
}

#[repr(C)]
#[derive(AlignedBorrow)]
pub struct NativeSumcheckCols<T> {
    /// Indicates that this row is the header for a layer sum operation
    pub header_row: T,
    /// Indicates that this row is a step for prod_spec in the layer sum operation
    pub prod_row: T,
    /// Indicates that this row is a step for logup_spec in the layer sum operation
    pub logup_row: T,
    /// Indicates that this row is the end of the entire layer sum operation
    pub is_end: T,

    pub prod_continued: T,
    pub logup_continued: T,

    /// Indicates what type of evaluation constraints should be applied
    pub prod_in_round_evaluation: T,
    pub prod_next_round_evaluation: T,
    pub logup_in_round_evaluation: T,
    pub logup_next_round_evaluation: T,

    /// Indicates if evaluations are accumulated
    pub prod_acc: T,
    pub logup_acc: T,

    /// Timestamps
    pub first_timestamp: T,
    pub start_timestamp: T,
    pub last_timestamp: T,

    // Register values
    pub register_ptrs: [T; 3],

    // Context variables
    // [
    //     round,
    //     num_prod_spec,
    //     num_logup_spec,
    //     prod_spec_inner_len,
    //     prod_spec_inner_inner_len,
    //     logup_spec_inner_len,
    //     logup_spec_inner_inner_len,
    //     in_layer,
    // ]
    pub ctx: [T; EXT_DEG * 2],

    pub prod_nested_len: T,
    pub logup_nested_len: T,

    pub curr_prod_n: T,
    pub curr_logup_n: T,

    pub alpha: [T; EXT_DEG],
    // alpha1, c1, c2, alpha2 (for logup rows)
    pub challenges: [T; EXT_DEG * 4],

    // Specific to each row
    pub max_round: T,
    // Is this round within max_round
    pub within_round_limit: T,
    // Should the evaluation be accumualted
    pub should_acc: T,

    // The current final evaluation accumulator. Extension element.
    pub eval_acc: [T; EXT_DEG],

    // /// 1. For header row, 5 registers, ctx, challenges
    // /// 2. For the rest: max_variables, p1, p2, q1, q2
    // pub read_records: [MemoryReadAuxCols<T>; 7],
    // /// 1. For header row, write final result
    // /// 2. For prod rows: write prod_evals
    // /// 3. For logup rows: write q_evals, p_evals
    // pub write_records: [MemoryWriteAuxCols<T, EXT_DEG>; 2],
    pub specific: [T; max3(
        HeaderSpecificCols::<usize>::width(),
        ProdSpecificCols::<usize>::width(),
        LogupSpecificCols::<usize>::width(),
    )],
}

#[repr(C)]
#[derive(AlignedBorrow)]
pub struct HeaderSpecificCols<T> {
    pub pc: T,
    pub registers: [T; 3],
    pub prod_id: T,
    pub logup_id: T,
    /// 3 register reads + ctx read + max round read + challenges read
    pub read_records: [MemoryReadAuxCols<T>; 6],
    /// Write the final evaluation
    pub write_records: MemoryWriteAuxCols<T, EXT_DEG>,
}

#[repr(C)]
#[derive(AlignedBorrow)]
pub struct ProdSpecificCols<T> {
    /// Pointer
    pub data_ptr: T,
    /// 2 extension elements
    pub p: [T; EXT_DEG * 2],
    /// read 2 p values
    pub read_records: [MemoryReadAuxCols<T>; 1],
    /// Calculated p evals
    pub p_evals: [T; EXT_DEG],
    /// write p_evals
    pub write_record: MemoryWriteAuxCols<T, EXT_DEG>,
    /// p_evals * alpha^i
    pub eval_rlc: [T; EXT_DEG],
}

#[repr(C)]
#[derive(AlignedBorrow)]
pub struct LogupSpecificCols<T> {
    /// Pointer
    pub data_ptr: T,
    /// 4 extension elements
    pub pq: [T; EXT_DEG * 4],
    /// read 4 values: p1, p2, q1, q2
    pub read_records: [MemoryReadAuxCols<T>; 1],
    /// Calculated p evals
    pub p_evals: [T; EXT_DEG],
    /// Calculated q evals
    pub q_evals: [T; EXT_DEG],
    /// write both p_evals and q_evals
    pub write_records: [MemoryWriteAuxCols<T, EXT_DEG>; 2],
    /// Evaluation for the accumulator
    pub eval_rlc: [T; EXT_DEG],
}
