use std::{borrow::Borrow, mem::size_of, slice::from_raw_parts, sync::Arc};

use derive_new::new;
use openvm_circuit::{arch::DenseRecordArena, utils::next_power_of_two_or_zero};
use openvm_circuit_primitives::var_range::VariableRangeCheckerChipGPU;
use openvm_cuda_backend::{
    base::DeviceMatrix, chip::get_empty_air_proving_ctx, prover_backend::GpuBackend, types::F,
};
use openvm_cuda_common::copy::MemCopyH2D;
use openvm_stark_backend::{p3_field::PrimeField32, prover::types::AirProvingContext, Chip};

use super::columns::{LogupSpecificCols, NativeSumcheckCols, ProdSpecificCols};
use crate::{
    cuda_abi::sumcheck_cuda,
    hint_space_provider::SharedHintSpaceProviderChip,
    utils::{OPENVM_NATIVE_GPU_DEBUG_ID, debug_log_native_gpu_tracegen_input},
};
use p3_field::FieldAlgebra;

#[derive(new)]
pub struct NativeSumcheckChipGpu {
    pub range_checker: Arc<VariableRangeCheckerChipGPU>,
    pub timestamp_max_bits: usize,
    pub hint_space_provider: SharedHintSpaceProviderChip<F>,
}

impl NativeSumcheckChipGpu {
    /// Scans execution records to populate the hint space provider with
    /// (hint_id, offset, value) triples for each hint element referenced
    /// by prod and logup rows. This bridges the gap between CPU execution
    /// (which produces the records) and GPU trace generation.
    fn populate_hint_provider(&self, records: &[u8]) {
        let width = NativeSumcheckCols::<F>::width();
        let record_size = width * size_of::<F>();
        if records.len() % record_size != 0 {
            return;
        }
        let num_rows = records.len() / record_size;

        let row_slice = unsafe {
            let ptr = records.as_ptr() as *const F;
            from_raw_parts(ptr, num_rows * width)
        };

        for i in 0..num_rows {
            let row_data = &row_slice[i * width..(i + 1) * width];
            let cols: &NativeSumcheckCols<F> = row_data.borrow();

            if cols.within_round_limit != F::ONE {
                continue;
            }

            if cols.prod_row == F::ONE {
                let prod_specific: &ProdSpecificCols<F> =
                    cols.specific[..ProdSpecificCols::<F>::width()].borrow();
                for (j, &val) in prod_specific.p.iter().enumerate() {
                    self.hint_space_provider.request(
                        cols.prod_hint_id,
                        prod_specific.data_ptr + F::from_canonical_usize(j),
                        val,
                    );
                }
            } else if cols.logup_row == F::ONE {
                let logup_specific: &LogupSpecificCols<F> =
                    cols.specific[..LogupSpecificCols::<F>::width()].borrow();
                for (j, &val) in logup_specific.pq.iter().enumerate() {
                    self.hint_space_provider.request(
                        cols.logup_hint_id,
                        logup_specific.data_ptr + F::from_canonical_usize(j),
                        val,
                    );
                }
            }
        }
    }
}

impl Chip<DenseRecordArena, GpuBackend> for NativeSumcheckChipGpu {
    fn generate_proving_ctx(&self, arena: DenseRecordArena) -> AirProvingContext<GpuBackend> {
        let records = arena.allocated();
        if records.is_empty() {
            return get_empty_air_proving_ctx::<GpuBackend>();
        }

        // Populate hint space provider from execution records before GPU upload.
        self.populate_hint_provider(records);

        let width = NativeSumcheckCols::<F>::width();
        let record_size = width * size_of::<F>();
        assert_eq!(records.len() % record_size, 0);

        let height = records.len() / record_size;
        let padded_height = next_power_of_two_or_zero(height);
        let trace = DeviceMatrix::<F>::with_capacity(padded_height, width);

        let records_hash = debug_log_native_gpu_tracegen_input(
            "native_sumcheck",
            records,
            record_size,
            height,
            padded_height,
            width,
        );

        let record_slice = unsafe {
            let ptr = records.as_ptr();
            from_raw_parts(ptr as *const F, records.len() / size_of::<F>())
        };
        let d_records = record_slice.to_device().unwrap();

        unsafe {
            if let Err(err) = sumcheck_cuda::tracegen(
                trace.buffer(),
                padded_height,
                width,
                &d_records,
                height,
                &self.range_checker.count,
                self.timestamp_max_bits as u32,
            ) {
                panic!(
                    "native_sumcheck cuda tracegen failed [{}]: err={:?}, height={}, padded_height={}, width={}, timestamp_max_bits={}, hash=0x{:016x}",
                    OPENVM_NATIVE_GPU_DEBUG_ID,
                    err,
                    height,
                    padded_height,
                    width,
                    self.timestamp_max_bits,
                    records_hash,
                );
            }
        }

        println!(
            "[openvm-gpu-debug][{}][native_sumcheck] tracegen ok: height={} padded_height={} width={} hash=0x{:016x}",
            OPENVM_NATIVE_GPU_DEBUG_ID,
            height,
            padded_height,
            width,
            records_hash,
        );

        AirProvingContext::simple_no_pis(trace)
    }
}
