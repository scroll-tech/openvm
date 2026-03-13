use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    mem::size_of,
    sync::{Arc, Mutex},
};

use openvm_circuit::system::memory::offline_checker::HintBus;
use openvm_circuit_primitives_derive::AlignedBorrow;
use openvm_stark_backend::{
    config::{StarkGenericConfig, Val},
    interaction::InteractionBuilder,
    p3_air::{Air, AirBuilder, BaseAir},
    p3_field::{Field, FieldAlgebra, PrimeField32},
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
    pub multiplicity: T,
    /// Inverse of multiplicity when nonzero; 0 for padding rows.
    pub mult_inv: T,
    /// Boolean: 1 if hint_id changes between this row and the next non-padding row.
    pub hint_id_changed: T,
    /// When hint_id_changed = 1: inverse of (next.hint_id - hint_id), proving they differ.
    /// When hint_id_changed = 0: unused (zero).
    pub diff_hint_id_inv: T,
    /// Boolean: 1 if this row is not a padding row (multiplicity > 0).
    pub curr_is_non_padding: T,
    /// Boolean: 1 if the next row is not a padding row.
    pub next_is_non_padding: T,
    /// Boolean: curr_is_non_padding * next_is_non_padding.
    pub both_non_padding: T,
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
        let curr = main.row_slice(0);
        let curr: &HintSpaceProviderCols<AB::Var> = (*curr).borrow();
        let next = main.row_slice(1);
        let next: &HintSpaceProviderCols<AB::Var> = (*next).borrow();

        // curr_is_non_padding is boolean and tied to multiplicity via mult_inv.
        builder.assert_bool(curr.curr_is_non_padding);
        builder.assert_eq(
            curr.curr_is_non_padding,
            curr.multiplicity * curr.mult_inv,
        );
        // Padding rows must have multiplicity = 0.
        builder.assert_zero(
            (AB::Expr::ONE - curr.curr_is_non_padding) * curr.multiplicity,
        );

        builder.assert_bool(curr.hint_id_changed);

        // Tie next_is_non_padding and both_non_padding columns to their definitions.
        builder
            .when_transition()
            .assert_eq(curr.next_is_non_padding, next.curr_is_non_padding);
        builder.when_transition().assert_eq(
            curr.both_non_padding,
            curr.curr_is_non_padding * curr.next_is_non_padding,
        );

        // Non-padding rows must appear before padding rows (non-increasing).
        builder
            .when_transition()
            .when(curr.next_is_non_padding)
            .assert_one(curr.curr_is_non_padding);

        // Uniqueness of (hint_id, offset) among non-padding rows.
        // Rows are sorted by (hint_id, offset). For consecutive non-padding rows:
        //   - Same hint_id (hint_id_changed=0): offset must increase by exactly 1.
        //   - Different hint_id (hint_id_changed=1): hint_id must actually differ
        //     (proven via inverse), and the new block starts at offset 0.
        let d_id: AB::Expr = next.hint_id - curr.hint_id;

        // hint_id_changed = 0 => same hint_id, offset increases by 1
        builder
            .when_transition()
            .when(curr.both_non_padding)
            .when_ne(curr.hint_id_changed, AB::Expr::ONE)
            .assert_zero(d_id.clone());
        builder
            .when_transition()
            .when(curr.both_non_padding)
            .when_ne(curr.hint_id_changed, AB::Expr::ONE)
            .assert_eq(next.offset, curr.offset + AB::Expr::ONE);

        // Combined inverse check: d_id * diff_hint_id_inv = hint_id_changed.
        // When hint_id_changed = 0: trivially 0 = 0 (since d_id = 0 from above).
        // When hint_id_changed = 1: proves d_id != 0 (hint_id actually changed).
        builder
            .when_transition()
            .when(curr.both_non_padding)
            .assert_eq(d_id * curr.diff_hint_id_inv, curr.hint_id_changed);

        // hint_id_changed = 1 => new block starts at offset 0
        builder
            .when_transition()
            .when(curr.both_non_padding)
            .when(curr.hint_id_changed)
            .assert_zero(next.offset);

        self.hint_bus.provide(
            builder,
            curr.hint_id,
            curr.offset,
            curr.value,
            curr.multiplicity,
        );
    }
}

pub struct HintSpaceProviderChip<F> {
    pub air: HintSpaceProviderAir,
    /// Maps (hint_id, offset) -> (value, multiplicity).
    /// Deduplicates keys and tracks how many times each is looked up.
    data: Mutex<HashMap<(F, F), (F, F)>>,
}

pub type SharedHintSpaceProviderChip<F> = Arc<HintSpaceProviderChip<F>>;

impl<F> HintSpaceProviderChip<F> {
    pub fn new(hint_bus: HintBus) -> Self {
        Self {
            air: HintSpaceProviderAir { hint_bus },
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl<F: Field> HintSpaceProviderChip<F> {
    /// Register a (hint_id, offset, value) triple for the provider trace.
    /// Called by consumer chips during trace filling to match each lookup.
    /// Deduplicates by (hint_id, offset) and increments the multiplicity counter.
    pub fn request(&self, hint_id: F, offset: F, value: F) {
        self.data
            .lock()
            .unwrap()
            .entry((hint_id, offset))
            .and_modify(|(v, m)| {
                debug_assert_eq!(*v, value, "conflicting values for same (hint_id, offset)");
                *m += F::ONE;
            })
            .or_insert((value, F::ONE));
    }
}

impl<F: PrimeField32> HintSpaceProviderChip<F> {
    pub fn generate_trace(&self) -> RowMajorMatrix<F> {
        let data = std::mem::take(&mut *self.data.lock().unwrap());
        // Collect into a Vec and sort by (hint_id, offset) to satisfy the AIR ordering constraints.
        let mut entries: Vec<_> = data.into_iter().collect();
        entries.sort_by_key(|((h, o), _)| (h.as_canonical_u64(), o.as_canonical_u64()));

        let num_non_padding_rows = entries.len();
        let trace_height = num_non_padding_rows.next_power_of_two().max(2);

        let mut rows = F::zero_vec(trace_height * NUM_HINT_SPACE_PROVIDER_COLS);
        for (n, ((hint_id, offset), (value, multiplicity))) in entries.iter().enumerate() {
            let row =
                &mut rows[n * NUM_HINT_SPACE_PROVIDER_COLS..(n + 1) * NUM_HINT_SPACE_PROVIDER_COLS];
            let cols: &mut HintSpaceProviderCols<F> = row.borrow_mut();
            cols.hint_id = *hint_id;
            cols.offset = *offset;
            cols.value = *value;
            cols.multiplicity = *multiplicity;
            cols.mult_inv = multiplicity.try_inverse().unwrap();
            cols.curr_is_non_padding = F::ONE;
            cols.next_is_non_padding =
                if n + 1 < num_non_padding_rows { F::ONE } else { F::ZERO };
            cols.both_non_padding =
                if n + 1 < num_non_padding_rows { F::ONE } else { F::ZERO };

            // Fill auxiliary columns for the uniqueness constraint.
            if n + 1 < num_non_padding_rows {
                let next_hint_id = entries[n + 1].0 .0;
                let d_id = next_hint_id - *hint_id;
                if d_id != F::ZERO {
                    cols.hint_id_changed = F::ONE;
                    cols.diff_hint_id_inv = d_id.try_inverse().unwrap();
                } else {
                    debug_assert_eq!(
                        entries[n + 1].0 .1,
                        *offset + F::ONE,
                        "Offsets for hint_id {:?} are not consecutive: {:?} -> {:?}",
                        hint_id,
                        offset,
                        entries[n + 1].0 .1
                    );
                    // hint_id_changed = 0, diff_hint_id_inv = 0 (defaults)
                }
            }
            // Last non-padding row: aux columns stay zero (no next non-padding row to compare)
        }
        // padding rows are already zero (multiplicity = 0)
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
    use openvm_cuda_backend::{
        chip::cpu_proving_ctx_to_gpu, prover_backend::GpuBackend, types::F, types::SC,
    };
    use openvm_stark_backend::{
        prover::{cpu::CpuBackend, types::AirProvingContext},
        Chip,
    };

    use super::HintSpaceProviderChip;

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
            let cpu_ctx: AirProvingContext<CpuBackend<SC>> =
                AirProvingContext::simple_no_pis(Arc::new(self.cpu_chip.generate_trace()));
            cpu_proving_ctx_to_gpu(cpu_ctx)
        }
    }
}
