use std::{borrow::Borrow, mem::size_of, slice::from_raw_parts, sync::Arc};

use openvm_circuit::{arch::DenseRecordArena, utils::next_power_of_two_or_zero};
use openvm_circuit_primitives::var_range::VariableRangeCheckerChipGPU;
use openvm_cuda_backend::{
    base::DeviceMatrix, chip::get_empty_air_proving_ctx, prover_backend::GpuBackend, types::F,
};
use openvm_cuda_common::copy::MemCopyH2D;
use openvm_stark_backend::{prover::types::AirProvingContext, Chip};
use p3_field::{Field, FieldAlgebra, PrimeField32};

use super::columns::{MultiObserveCols, NativePoseidon2Cols};
use crate::{
    cuda_abi::poseidon2_cuda,
    hint_space_provider::SharedHintSpaceProviderChip,
};

pub struct NativePoseidon2ChipGpu<const SBOX_REGISTERS: usize> {
    pub range_checker: Arc<VariableRangeCheckerChipGPU>,
    pub timestamp_max_bits: usize,
    pub hint_space_provider: Option<SharedHintSpaceProviderChip<F>>,
}

impl<const SBOX_REGISTERS: usize> NativePoseidon2ChipGpu<SBOX_REGISTERS> {
    pub fn new(range_checker: Arc<VariableRangeCheckerChipGPU>, timestamp_max_bits: usize) -> Self {
        Self {
            range_checker,
            timestamp_max_bits,
            hint_space_provider: None,
        }
    }

    pub fn new_with_hint_space_provider(
        range_checker: Arc<VariableRangeCheckerChipGPU>,
        timestamp_max_bits: usize,
        hint_space_provider: SharedHintSpaceProviderChip<F>,
    ) -> Self {
        Self {
            range_checker,
            timestamp_max_bits,
            hint_space_provider: Some(hint_space_provider),
        }
    }

    /// Scans multi-observe execution records to populate the hint provider with
    /// (hint_id, offset, value) triples for hint-mode rows.
    fn populate_hint_provider(&self, records: &[u8]) {
        let Some(hint_space_provider) = &self.hint_space_provider else {
            return;
        };

        let width = NativePoseidon2Cols::<F, SBOX_REGISTERS>::width();
        let record_size = width * size_of::<F>();
        if records.len() % record_size != 0 {
            return;
        }
        let height = records.len() / record_size;

        let row_slice = unsafe {
            let ptr = records.as_ptr() as *const F;
            from_raw_parts(ptr, height * width)
        };

        let mut row_idx = 0;
        while row_idx < height {
            let start = row_idx * width;
            let cols: &NativePoseidon2Cols<F, SBOX_REGISTERS> =
                row_slice[start..(start + width)].borrow();

            if cols.multi_observe_row.is_one() {
                let num_rows = cols.inner.export.as_canonical_u32() as usize;
                if num_rows > 1 {
                    let head_multi_observe_cols: &MultiObserveCols<F> =
                        cols.specific[..MultiObserveCols::<u8>::width()].borrow();
                    let is_hint = head_multi_observe_cols.ctx[2] != F::ZERO;
                    if is_hint {
                        let hint_id = head_multi_observe_cols.hint_id;
                        for local_row in 1..num_rows {
                            let chunk_cols: &NativePoseidon2Cols<F, SBOX_REGISTERS> =
                                row_slice[(row_idx + local_row) * width
                                    ..(row_idx + local_row + 1) * width]
                                    .borrow();
                            let multi_observe_cols: &MultiObserveCols<F> = chunk_cols.specific
                                [..MultiObserveCols::<u8>::width()]
                                .borrow();

                            let chunk_start = multi_observe_cols.start_idx.as_canonical_u32();
                            let chunk_end = multi_observe_cols.end_idx.as_canonical_u32();
                            let curr_len = multi_observe_cols.curr_len.as_canonical_u32();

                            for j in chunk_start..chunk_end {
                                let input_idx = curr_len + (j - chunk_start);
                                let val = multi_observe_cols.data[j as usize];
                                hint_space_provider.request(
                                    hint_id,
                                    F::from_canonical_u32(input_idx),
                                    val,
                                );
                            }
                        }
                    }
                }
                row_idx += num_rows.max(1);
                continue;
            }

            if cols.simple.is_one() {
                row_idx += 1;
            } else {
                let num_non_inside_row = cols.inner.export.as_canonical_u32() as usize;
                let non_inside_start = start + (num_non_inside_row - 1) * width;
                let last_non_inside_cols: &NativePoseidon2Cols<F, SBOX_REGISTERS> =
                    row_slice[non_inside_start..(non_inside_start + width)].borrow();
                let total_num_row = last_non_inside_cols.inner.export.as_canonical_u32() as usize;
                row_idx += total_num_row;
            }
        }
    }
}

impl<const SBOX_REGISTERS: usize> Chip<DenseRecordArena, GpuBackend>
    for NativePoseidon2ChipGpu<SBOX_REGISTERS>
{
    fn generate_proving_ctx(&self, arena: DenseRecordArena) -> AirProvingContext<GpuBackend> {
        let records = arena.allocated();
        if records.is_empty() {
            return get_empty_air_proving_ctx::<GpuBackend>();
        }

        // Populate hint space provider from multi-observe records before GPU upload.
        self.populate_hint_provider(records);

        // For Poseidon2, the records are already the trace rows
        // Use the columns width directly
        let width = NativePoseidon2Cols::<F, SBOX_REGISTERS>::width();

        let record_size = width * size_of::<F>();
        assert_eq!(records.len() % record_size, 0);

        let height = records.len() / record_size;
        let padded_height = next_power_of_two_or_zero(height);

        let d_chunk_start = {
            let mut row_idx = 0;
            let row_slice = unsafe {
                let raw_ptr = records.as_ptr();
                from_raw_parts(raw_ptr as *const F, records.len() / size_of::<F>())
            };
            let mut chunk_start = Vec::new();
            // Allocated rows are not empty. Determine the chunk start indices.
            while row_idx < height {
                let start = row_idx * width;
                let cols: &NativePoseidon2Cols<F, SBOX_REGISTERS> =
                    row_slice[start..(start + width)].borrow();
                chunk_start.push(row_idx as u32);
                if cols.simple.is_one() {
                    row_idx += 1;
                } else if cols.multi_observe_row.is_one() {
                    let num_rows = cols.inner.export.as_canonical_u32() as usize;
                    row_idx += num_rows;
                } else {
                    let num_non_inside_row = cols.inner.export.as_canonical_u32() as usize;
                    let non_inside_start = start + (num_non_inside_row - 1) * width;
                    let cols: &NativePoseidon2Cols<F, SBOX_REGISTERS> =
                        row_slice[non_inside_start..(non_inside_start + width)].borrow();
                    let total_num_row = cols.inner.export.as_canonical_u32() as usize;
                    row_idx += total_num_row;
                };
            }
            chunk_start.to_device().unwrap()
        };

        let trace = DeviceMatrix::<F>::with_capacity(padded_height, width);

        let d_records = records.to_device().unwrap();

        unsafe {
            poseidon2_cuda::tracegen(
                trace.buffer(),
                padded_height,
                width,
                &d_records,
                height,
                &d_chunk_start,
                d_chunk_start.len() as u32,
                &self.range_checker.count,
                SBOX_REGISTERS as u32,
                self.timestamp_max_bits as u32,
            )
            .unwrap();
        }

        AirProvingContext::simple_no_pis(trace)
    }
}
