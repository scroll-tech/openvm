use std::{mem::size_of, sync::Arc};

use derive_new::new;
use openvm_circuit::{arch::DenseRecordArena, utils::next_power_of_two_or_zero};
use openvm_circuit_primitives::var_range::VariableRangeCheckerChipGPU;
use openvm_cuda_backend::{
    base::DeviceMatrix, chip::get_empty_air_proving_ctx, prover_backend::GpuBackend, types::F,
};
use openvm_cuda_common::copy::MemCopyH2D;
use openvm_stark_backend::{prover::types::AirProvingContext, Chip};

use super::{CastFCoreCols, CastFCoreRecord};
use crate::{
    adapters::{ConvertAdapterCols, ConvertAdapterRecord},
    cuda_abi::castf_cuda,
    utils::{OPENVM_NATIVE_GPU_DEBUG_ID, debug_log_native_gpu_tracegen_input},
};

#[derive(new)]
pub struct CastFChipGpu {
    pub range_checker: Arc<VariableRangeCheckerChipGPU>,
    pub timestamp_max_bits: usize,
}

impl Chip<DenseRecordArena, GpuBackend> for CastFChipGpu {
    fn generate_proving_ctx(&self, arena: DenseRecordArena) -> AirProvingContext<GpuBackend> {
        const RECORD_SIZE: usize = size_of::<(ConvertAdapterRecord<F, 1, 4>, CastFCoreRecord)>();
        let records = arena.allocated();
        if records.is_empty() {
            return get_empty_air_proving_ctx::<GpuBackend>();
        }
        assert_eq!(records.len() % RECORD_SIZE, 0);

        let height = records.len() / RECORD_SIZE;
        let padded_height = next_power_of_two_or_zero(height);
        let trace_width = ConvertAdapterCols::<F, 1, 4>::width() + CastFCoreCols::<F>::width();
        let trace = DeviceMatrix::<F>::with_capacity(padded_height, trace_width);

        let records_hash = debug_log_native_gpu_tracegen_input(
            "native_castf",
            records,
            RECORD_SIZE,
            height,
            padded_height,
            trace_width,
        );

        let d_records = records.to_device().unwrap();

        unsafe {
            if let Err(err) = castf_cuda::tracegen(
                trace.buffer(),
                padded_height,
                trace_width,
                &d_records,
                &self.range_checker.count,
                self.timestamp_max_bits as u32,
            ) {
                panic!(
                    "native_castf cuda tracegen failed [{}]: err={:?}, height={}, padded_height={}, trace_width={}, timestamp_max_bits={}, hash=0x{:016x}",
                    OPENVM_NATIVE_GPU_DEBUG_ID,
                    err,
                    height,
                    padded_height,
                    trace_width,
                    self.timestamp_max_bits,
                    records_hash,
                );
            }
        }

        println!(
            "[openvm-gpu-debug][{}][native_castf] tracegen ok: height={} padded_height={} trace_width={} hash=0x{:016x}",
            OPENVM_NATIVE_GPU_DEBUG_ID,
            height,
            padded_height,
            trace_width,
            records_hash,
        );

        AirProvingContext::simple_no_pis(trace)
    }
}
