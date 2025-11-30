use std::{mem::size_of, slice::from_raw_parts, sync::Arc};

use derive_new::new;
use openvm_circuit::{arch::DenseRecordArena, utils::next_power_of_two_or_zero};
use openvm_circuit_primitives::var_range::VariableRangeCheckerChipGPU;
use openvm_cuda_backend::{
    base::DeviceMatrix, chip::get_empty_air_proving_ctx, prover_backend::GpuBackend, types::F,
};
use openvm_cuda_common::copy::MemCopyH2D;
use openvm_stark_backend::{prover::types::AirProvingContext, Chip};

use super::columns::NativeSumcheckCols;
use crate::cuda_abi::sumcheck_cuda;

#[derive(new)]
pub struct NativeSumcheckChipGpu {
    pub range_checker: Arc<VariableRangeCheckerChipGPU>,
    pub timestamp_max_bits: usize,
}

impl Chip<DenseRecordArena, GpuBackend> for NativeSumcheckChipGpu {
    fn generate_proving_ctx(&self, arena: DenseRecordArena) -> AirProvingContext<GpuBackend> {
        let records = arena.allocated();
        if records.is_empty() {
            return get_empty_air_proving_ctx::<GpuBackend>();
        }

        let width = NativeSumcheckCols::<F>::width();
        let record_size = width * size_of::<F>();
        assert_eq!(records.len() % record_size, 0);

        let height = records.len() / record_size;
        let padded_height = next_power_of_two_or_zero(height);
        let trace = DeviceMatrix::<F>::with_capacity(padded_height, width);

        let record_slice = unsafe {
            let ptr = records.as_ptr();
            from_raw_parts(ptr as *const F, records.len() / size_of::<F>())
        };
        let d_records = record_slice.to_device().unwrap();

        unsafe {
            sumcheck_cuda::tracegen(
                trace.buffer(),
                padded_height,
                width,
                &d_records,
                height,
                &self.range_checker.count,
                self.timestamp_max_bits as u32,
            )
            .unwrap();
        }

        AirProvingContext::simple_no_pis(trace)
    }
}
