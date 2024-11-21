use ax_circuit_derive::AlignedBorrow;
use ax_hashes::sha256::{Sha256DigestCols, Sha256RoundCols};
use axvm_instructions::riscv::RV32_REGISTER_NUM_LIMBS;

use super::{SHA256_READ_SIZE, SHA256_REGISTER_READS, SHA256_WRITE_SIZE};
use crate::{
    arch::ExecutionState,
    system::memory::offline_checker::{MemoryReadAuxCols, MemoryWriteAuxCols},
};

/// WARNING: the order of fields in the structs is important, do not change it

/// the first 16 rows of every SHA256 block will be of type Sha256VmRoundCols and the last row will be of type Sha256VmDigestCols
#[repr(C)]
#[derive(Clone, Copy, Debug, AlignedBorrow)]
pub struct Sha256VmRoundCols<T> {
    /// This is the length of the entire message divided into (6-bit, 24-bit) limbs, used for constraining the padding
    pub len: [T; 2],
    pub read_ptr: T,
    /// Need the timestamp as well since it has no information about the timestamp otherwise
    pub cur_timestamp: T,
    /// Padding flags which will be used to encode the the number of non-padding cells in the current row
    pub pad_flags: [T; 5],
    /// A flag that indicates if the current row includes a padding cell
    pub is_padding: T,
    /// Note: We will use the buffer in `inner.message_schedule` as the message
    pub inner: Sha256RoundCols<T>,
    pub read_aux: MemoryReadAuxCols<T, SHA256_READ_SIZE>,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, AlignedBorrow)]
pub struct Sha256VmDigestCols<T> {
    /// This is the length of the entire message divided into (6-bit, 24-bit) limbs, used for constraining the padding
    pub len: [T; 2],
    /// read_ptr here is used to propogate the read pointer to the next row
    pub read_ptr: T,
    pub cur_timestamp: T,
    pub pad_flags: [T; 5],
    pub is_padding: T,
    pub inner: Sha256DigestCols<T>,

    pub from_state: ExecutionState<T>,
    /// It is counter intuitive, but we will constrain the register reads on the very last row of every message
    pub rd_ptr: T,
    pub rs1_ptr: T,
    pub rs2_ptr: T,
    pub dst_ptr: [T; RV32_REGISTER_NUM_LIMBS],
    pub src_ptr: [T; RV32_REGISTER_NUM_LIMBS],
    pub len_cells: [T; RV32_REGISTER_NUM_LIMBS],
    pub register_reads_aux: [MemoryReadAuxCols<T, RV32_REGISTER_NUM_LIMBS>; SHA256_REGISTER_READS],
    pub writes_aux: MemoryWriteAuxCols<T, SHA256_WRITE_SIZE>,
}
/// Width of the Sha256VmRoundCols
pub const SHA256VM_ROUND_WIDTH: usize = Sha256VmRoundCols::<u8>::width();
/// Width of the Sha256VmDigestCols
pub const SHA256VM_DIGEST_WIDTH: usize = Sha256VmDigestCols::<u8>::width();
/// Width of the Sha256Cols
pub const SHA256VM_WIDTH: usize = if SHA256VM_ROUND_WIDTH > SHA256VM_DIGEST_WIDTH {
    SHA256VM_ROUND_WIDTH
} else {
    SHA256VM_DIGEST_WIDTH
};
