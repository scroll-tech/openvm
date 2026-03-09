use std::{
    borrow::{Borrow, BorrowMut},
    mem::size_of,
    sync::{Arc, Mutex},
};

use openvm_circuit::system::memory::offline_checker::HintBus;
use openvm_circuit_primitives_derive::AlignedBorrow;
use openvm_stark_backend::{
    config::{StarkGenericConfig, Val},
    interaction::InteractionBuilder,
    p3_air::{Air, BaseAir},
    p3_field::{Field, PrimeField32},
    p3_matrix::{dense::RowMajorMatrix, Matrix},
    prover::{cpu::CpuBackend, types::AirProvingContext},
    rap::{get_air_name, BaseAirWithPublicValues, PartitionedBaseAir},
    Chip, ChipUsageGetter,
};

#[derive(Default, AlignedBorrow, Copy, Clone)]
#[repr(C)]
pub struct HintSpaceProviderCols<T> {
    pub hint_id: T,
    pub offset: T,
    pub value: T,
    pub is_valid: T,
}

pub const NUM_HINT_SPACE_PROVIDER_COLS: usize = size_of::<HintSpaceProviderCols<u8>>();

#[derive(Clone, Debug)]
pub struct HintSpaceProviderAir {
    pub hint_bus: HintBus,
}

impl<F: Field> BaseAirWithPublicValues<F> for HintSpaceProviderAir {}
impl<F: Field> PartitionedBaseAir<F> for HintSpaceProviderAir {}

impl<F: Field> BaseAir<F> for HintSpaceProviderAir {
    fn width(&self) -> usize {
        NUM_HINT_SPACE_PROVIDER_COLS
    }
}

impl<AB: InteractionBuilder> Air<AB> for HintSpaceProviderAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let local: &HintSpaceProviderCols<AB::Var> = (*local).borrow();

        builder.assert_bool(local.is_valid);

        self.hint_bus.provide(
            builder,
            local.hint_id,
            local.offset,
            local.value,
            local.is_valid,
        );
    }
}

pub struct HintSpaceProviderChip<F> {
    pub air: HintSpaceProviderAir,
    data: Mutex<Vec<(F, F, F)>>,
}

pub type SharedHintSpaceProviderChip<F> = Arc<HintSpaceProviderChip<F>>;

impl<F> HintSpaceProviderChip<F> {
    pub fn new(hint_bus: HintBus) -> Self {
        Self {
            air: HintSpaceProviderAir { hint_bus },
            data: Mutex::new(Vec::new()),
        }
    }

    /// Register a (hint_id, offset, value) triple for the provider trace.
    /// Called by consumer chips during trace filling to match each lookup.
    pub fn request(&self, hint_id: F, offset: F, value: F) {
        self.data.lock().unwrap().push((hint_id, offset, value));
    }
}

impl<F: PrimeField32> HintSpaceProviderChip<F> {
    pub fn generate_trace(&self) -> RowMajorMatrix<F> {
        let data = std::mem::take(&mut *self.data.lock().unwrap());
        let num_real_rows = data.len();
        let trace_height = num_real_rows.next_power_of_two().max(2);

        let mut rows = F::zero_vec(trace_height * NUM_HINT_SPACE_PROVIDER_COLS);
        for (n, row) in rows
            .chunks_exact_mut(NUM_HINT_SPACE_PROVIDER_COLS)
            .enumerate()
        {
            if n < num_real_rows {
                let cols: &mut HintSpaceProviderCols<F> = row.borrow_mut();
                cols.hint_id = data[n].0;
                cols.offset = data[n].1;
                cols.value = data[n].2;
                cols.is_valid = F::ONE;
            }
            // padding rows are already zero (is_valid = 0)
        }
        RowMajorMatrix::new(rows, NUM_HINT_SPACE_PROVIDER_COLS)
    }
}

impl<R, SC: StarkGenericConfig> Chip<R, CpuBackend<SC>> for HintSpaceProviderChip<Val<SC>>
where
    Val<SC>: PrimeField32,
{
    fn generate_proving_ctx(&self, _: R) -> AirProvingContext<CpuBackend<SC>> {
        let trace = self.generate_trace();
        AirProvingContext::simple_no_pis(Arc::new(trace))
    }
}

impl<F: PrimeField32> ChipUsageGetter for HintSpaceProviderChip<F> {
    fn air_name(&self) -> String {
        get_air_name(&self.air)
    }
    fn constant_trace_height(&self) -> Option<usize> {
        None
    }
    fn current_trace_height(&self) -> usize {
        self.data.lock().unwrap().len().next_power_of_two().max(2)
    }
    fn trace_width(&self) -> usize {
        NUM_HINT_SPACE_PROVIDER_COLS
    }
}

#[cfg(feature = "cuda")]
pub mod cuda {
    use std::sync::Arc;

    use openvm_circuit::arch::DenseRecordArena;
    use openvm_cuda_backend::{base::DeviceMatrix, prover_backend::GpuBackend, types::F};
    use openvm_cuda_common::copy::MemCopyH2D;
    use openvm_stark_backend::{prover::types::AirProvingContext, Chip};

    use super::{HintSpaceProviderChip, NUM_HINT_SPACE_PROVIDER_COLS};
    use crate::cuda_abi::hint_space_provider_cuda;

    pub struct HintSpaceProviderChipGpu {
        pub cpu_chip: Arc<HintSpaceProviderChip<F>>,
    }

    impl HintSpaceProviderChipGpu {
        pub fn new(cpu_chip: Arc<HintSpaceProviderChip<F>>) -> Self {
            Self { cpu_chip }
        }
    }

    impl Chip<DenseRecordArena, GpuBackend> for HintSpaceProviderChipGpu {
        fn generate_proving_ctx(&self, _: DenseRecordArena) -> AirProvingContext<GpuBackend> {
            let data = std::mem::take(&mut *self.cpu_chip.data.lock().unwrap());
            let rows_used = data.len();
            let height = rows_used.next_power_of_two().max(2);

            // Flatten (hint_id, offset, value) triples into a contiguous [F] buffer
            let flat: Vec<F> = data
                .into_iter()
                .flat_map(|(h, o, v)| [h, o, v])
                .collect();

            let d_records = flat.to_device().unwrap();
            let trace = DeviceMatrix::<F>::with_capacity(height, NUM_HINT_SPACE_PROVIDER_COLS);

            unsafe {
                hint_space_provider_cuda::tracegen(
                    trace.buffer(),
                    height,
                    NUM_HINT_SPACE_PROVIDER_COLS,
                    &d_records,
                    rows_used,
                )
                .unwrap();
            }

            AirProvingContext::simple_no_pis(trace)
        }
    }
}
