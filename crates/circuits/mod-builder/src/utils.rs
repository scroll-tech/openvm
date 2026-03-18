use num_bigint::BigUint;

use crate::FieldExpressionCoreAir;

pub const OPENVM_GPU_DEBUG_ID: &str = "OVM-GPU-DBG-20260318";

// Use this when num_limbs is not a constant.
// little endian.
// Warning: This function only returns the last NUM_LIMBS bytes of
//          the input, while the input can have more than that.
#[inline(always)]
pub fn biguint_to_limbs_vec(x: &BigUint, num_limbs: usize) -> Vec<u8> {
    x.to_bytes_le()
        .into_iter()
        .chain(std::iter::repeat(0u8))
        .take(num_limbs)
        .collect()
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub fn debug_log_field_expr_gpu_input(
    chip_label: &str,
    record_size: usize,
    num_records: usize,
    adapter_width: usize,
    adapter_blocks: usize,
    pointer_max_bits: u32,
    timestamp_max_bits: u32,
    local_opcode_idx: &[usize],
    opcode_flag_idx: &[usize],
    air: &FieldExpressionCoreAir,
    records: &[u8],
) {
    let byte_len = records.len();
    let hash = fnv1a64(records);
    let sample_len = byte_len.min(64);

    println!(
        "[openvm-gpu-debug][{}] chip={} num_records={} record_size={} bytes={} hash=0x{:016x}",
        OPENVM_GPU_DEBUG_ID, chip_label, num_records, record_size, byte_len, hash
    );
    println!(
        "[openvm-gpu-debug][{}] chip={} adapter_width={} adapter_blocks={} pointer_max_bits={} timestamp_max_bits={}",
        OPENVM_GPU_DEBUG_ID, chip_label, adapter_width, adapter_blocks, pointer_max_bits, timestamp_max_bits
    );
    println!(
        "[openvm-gpu-debug][{}] chip={} local_opcode_idx={:?} opcode_flag_idx={:?}",
        OPENVM_GPU_DEBUG_ID, chip_label, local_opcode_idx, opcode_flag_idx
    );
    println!(
        "[openvm-gpu-debug][{}] chip={} expr: num_inputs={} num_vars={} num_flags={} outputs={} computes={} constraints={} prime_limbs={} limb_bits={} canonical_num_limbs={}",
        OPENVM_GPU_DEBUG_ID,
        chip_label,
        air.num_inputs(),
        air.num_vars(),
        air.num_flags(),
        air.output_indices().len(),
        air.expr.builder.computes.len(),
        air.expr.builder.constraints.len(),
        air.expr.builder.prime_limbs.len(),
        air.expr.canonical_limb_bits(),
        air.expr.canonical_num_limbs(),
    );
    println!(
        "[openvm-gpu-debug][{}] chip={} records_head({})={:?}",
        OPENVM_GPU_DEBUG_ID,
        chip_label,
        sample_len,
        &records[..sample_len]
    );
}
