use openvm_circuit::system::memory::offline_checker::MemoryReadAuxCols;
use openvm_circuit_primitives_derive::AlignedBorrow;
use openvm_poseidon2_air::Poseidon2SubCols;

use crate::verify_batch::CHUNK;

#[repr(C)]
#[derive(AlignedBorrow)]
pub struct VerifyBatchCols<T, const SBOX_REGISTERS: usize> {
    // poseidon2
    pub inner: Poseidon2SubCols<T, SBOX_REGISTERS>,

    // flags - at most 1 is true, if none is true then row is disabled
    pub incorporate_row: T,
    pub incorporate_sibling: T,
    pub inside_row: T,

    pub end_inside_row: T,
    pub end_top_level: T,
    pub start_top_level: T,

    // execution state (pc can be shared (saves 1 col), very_first_timestamp is used in a different way inside row)
    pub pc: T,
    pub very_first_timestamp: T,
    
    pub start_timestamp: T,
    pub end_timestamp: T, // only used for top level (so can be shared (saves 1 col)

    // instruction (a, b, c, d, e)
    // all other than address_space can be shared (saves 5 cols)
    pub dim_register: T,
    pub opened_register: T,
    pub sibling_register: T,
    pub index_register: T,
    pub commit_register: T,
    pub address_space: T,

    pub cells: [VerifyBatchCellCols<T, SBOX_REGISTERS>; CHUNK],
    // initial/final opened index for a subsegment with same height
    // initial is used in both, final is used only in top level
    pub initial_opened_index: T,
    // so then this one can be shared as well (saves 1 col)
    pub final_opened_index: T,

    // these two ig? (saves 2 cols)
    pub height: T,
    pub opened_length: T,

    pub dim_base_pointer: T,
    pub opened_base_pointer: T,
    pub sibling_base_pointer: T,
    pub index_base_pointer: T,

    // these can be optimized to be shared with cells[i].read_row_pointer_and_length (saves 15 cols)
    pub dim_base_pointer_read: MemoryReadAuxCols<T, 1>,
    pub opened_base_pointer_and_length_read: MemoryReadAuxCols<T, 2>,
    pub sibling_base_pointer_read: MemoryReadAuxCols<T, 1>,
    pub index_base_pointer_read: MemoryReadAuxCols<T, 1>,
    pub commit_pointer_read: MemoryReadAuxCols<T, 1>,

    // this with the other ones (saves 1 col)
    pub proof_index: T,

    // these as well (saves 6 cols)
    pub read_initial_height_or_root_is_on_right: MemoryReadAuxCols<T, 1>,
    pub read_final_height_or_sibling_array_start: MemoryReadAuxCols<T, 1>,

    // i guess these can be shared with like the other ones (saves 2 cols)
    pub root_is_on_right: T,
    pub sibling_array_start: T,

    // plus this (saves 1 col)
    pub commit_pointer: T,
    // this as well (saves 3 cols)
    pub commit_read: MemoryReadAuxCols<T, CHUNK>,
}

#[repr(C)]
#[derive(AlignedBorrow, Copy, Clone)]
pub struct VerifyBatchCellCols<T, const SBOX_REGISTERS: usize> {
    pub read: MemoryReadAuxCols<T, 1>,
    pub opened_index: T,
    pub read_row_pointer_and_length: MemoryReadAuxCols<T, 2>,
    pub row_pointer: T,
    pub row_end: T,
    pub is_first_in_row: T,
    pub is_exhausted: T,
}
