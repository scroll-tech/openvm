use std::sync::Arc;

use itertools::Itertools;
use openvm_circuit::system::memory::{MemoryAuxColsFactory, OfflineMemory};
use openvm_poseidon2_air::{p3_poseidon2_air::Poseidon2Cols, Poseidon2SubCols};
use openvm_stark_backend::{
    p3_air::BaseAir,
    p3_field::{Field, PrimeField32},
    ChipUsageGetter,
};

use crate::verify_batch::{
    chip::{IncorporateRowRecord, IncorporateSiblingRecord, VerifyBatchChip, VerifyBatchRecord},
    columns::{VerifyBatchCellCols, VerifyBatchCols},
    CHUNK,
};

impl<F: Field, const SBOX_REGISTERS: usize> ChipUsageGetter for VerifyBatchChip<F, SBOX_REGISTERS> {
    fn air_name(&self) -> String {
        "VerifyBatchAir".to_string()
    }

    fn current_trace_height(&self) -> usize {
        self.height
    }

    fn trace_width(&self) -> usize {
        VerifyBatchCols::<F, SBOX_REGISTERS>::width()
    }
}

impl<F: PrimeField32, const SBOX_REGISTERS: usize> VerifyBatchChip<F, SBOX_REGISTERS> {
    fn generate_subair_cols(&self, input: [F; 2 * CHUNK], cols: &mut [F]) {
        let inner_trace = self.subchip.generate_trace(vec![input]);
        let inner_width = self.air.subair.width();
        cols[..inner_width].copy_from_slice(&inner_trace.values.as_slice());
    }
    fn incorporate_sibling_record_to_rows(
        &self,
        record: &IncorporateSiblingRecord<F>,
        aux_cols_factory: &MemoryAuxColsFactory<F>,
        slice: &mut [F],
        memory: &OfflineMemory<F>,
        parent: &VerifyBatchRecord<F>,
        proof_index: usize,
        opened_index: usize,
        height: usize,
    ) {
        let &IncorporateSiblingRecord {
            read_sibling_array_start,
            read_root_is_on_right,
            root_is_on_right,
            sibling,
            reads,
            p2_input,
        } = record;

        let read_root_is_on_right = memory.record_by_id(read_root_is_on_right);
        let read_sibling_array_start = memory.record_by_id(read_sibling_array_start);

        self.generate_subair_cols(p2_input, slice);
        let cols: &mut VerifyBatchCols<F, SBOX_REGISTERS> = slice.borrow_mut();
        cols.incorporate_row = F::ZERO;
        cols.incorporate_sibling = F::ONE;
        cols.inside_row = F::ZERO;
        cols.end_inside_row = F::ZERO;
        cols.end_top_level = F::ZERO;
        cols.start = F::ZERO;
        cols.very_first_timestamp = parent.from_state.timestamp;
        cols.start_timestamp = F::from_canonical_u32(read_root_is_on_right.timestamp - 6);
        cols.end_timestamp =
            F::from_canonical_usize(read_root_is_on_right.timestamp as usize + (2 + CHUNK));
        cols.address_space = parent.address_space();
        for (read, cell) in reads.into_iter().zip_eq(cols.cells.iter_mut()) {
            cell.read = aux_cols_factory.make_read_aux_cols(memory.record_by_id(read));
        }
        cols.initial_opened_index = F::from_canonical_usize(opened_index);
        cols.final_opened_index = F::from_canonical_usize(opened_index - 1);
        cols.height = F::from_canonical_usize(height);
        cols.opened_length = F::from_canonical_usize(parent.opened_length);
        cols.dim_base_pointer = F::from_canonical_u32(parent.dim_base_pointer);
        cols.opened_base_pointer = F::from_canonical_u32(parent.opened_base_pointer);
        cols.sibling_base_pointer = F::from_canonical_u32(parent.sibling_base_pointer);
        cols.index_base_pointer = F::from_canonical_u32(parent.index_base_pointer);

        cols.proof_index = F::from_canonical_usize(proof_index);
        cols.read_initial_height_or_root_is_on_right =
            aux_cols_factory.make_read_aux_cols(read_root_is_on_right);
        cols.read_final_height_or_sibling_array_start =
            aux_cols_factory.make_read_aux_cols(read_sibling_array_start);
        cols.root_is_on_right = F::from_bool(root_is_on_right);
        cols.sibling_array_start = read_sibling_array_start.data[0];
    }
    fn correct_last_top_level_row(
        &self,
        record: &VerifyBatchRecord<F>,
        aux_cols_factory: &MemoryAuxColsFactory<F>,
        slice: &mut [F],
        memory: &OfflineMemory<F>,
    ) {
        let &VerifyBatchRecord {
            from_state,
            instruction,
            commit_pointer,
            dim_base_pointer_read,
            opened_base_pointer_and_length_read,
            sibling_base_pointer_read,
            index_base_pointer_read,
            commit_pointer_read,
            commit_read,
            ..
        } = record;
        let cols: &mut VerifyBatchCols<F, SBOX_REGISTERS> = slice.borrow_mut();
        cols.pc = F::from_canonical_u32(from_state.pc);
        cols.dim_register = instruction.a;
        cols.opened_register = instruction.b;
        cols.sibling_register = instruction.c;
        cols.index_register = instruction.d;
        cols.commit_register = instruction.e;
        cols.commit_pointer = F::from_canonical_u32(commit_pointer);
        cols.dim_base_pointer_read = aux_cols_factory.make_read_aux_cols(memory.record_by_id(dim_base_pointer_read));
        cols.opened_base_pointer_and_length_read = aux_cols_factory.make_read_aux_cols(memory.record_by_id(opened_base_pointer_and_length_read));
        cols.sibling_base_pointer_read = aux_cols_factory.make_read_aux_cols(memory.record_by_id(sibling_base_pointer_read));
        cols.index_base_pointer_read = aux_cols_factory.make_read_aux_cols(memory.record_by_id(index_base_pointer_read));
        cols.commit_pointer_read = aux_cols_factory.make_read_aux_cols(memory.record_by_id(commit_pointer_read));
        cols.commit_read = aux_cols_factory.make_read_aux_cols(memory.record_by_id(commit_read));
    }
    fn incorporate_row_record_to_rows(
        &self,
        record: &IncorporateRowRecord<F>,
        aux_cols_factory: &MemoryAuxColsFactory<F>,
        slice: &mut [F],
        memory: &OfflineMemory<F>,
        parent: &VerifyBatchRecord<F>,
        proof_index: usize,
        is_first: bool,
        height: usize,
    ) {
        let &IncorporateRowRecord {
            chunks,
            initial_opened_index,
            final_opened_index,
            initial_height_read,
            final_height_read,
            p2_input,
        } = record;

        let initial_height_read = memory.record_by_id(initial_height_read);
        let final_height_read = memory.record_by_id(final_height_read);

        self.generate_subair_cols(p2_input, slice);
        let cols: &mut VerifyBatchCols<F, SBOX_REGISTERS> = slice.borrow_mut();
        cols.incorporate_row = F::ONE;
        cols.incorporate_sibling = F::ZERO;
        cols.inside_row = F::ZERO;
        cols.end_inside_row = F::ZERO;
        cols.end_top_level = F::ZERO;
        cols.start = F::from_bool(is_first);
        cols.very_first_timestamp = parent.from_state.timestamp;
        cols.start_timestamp = F::from_canonical_u32(
            memory
                .record_by_id(chunks[0].cells[0].read_row_pointer_and_length.unwrap())
                .timestamp
                - 6,
        );
        cols.end_timestamp = F::from_canonical_u32(final_height_read.timestamp + 1);
        cols.address_space = parent.address_space();

        cols.initial_opened_index = F::from_canonical_usize(initial_opened_index);
        cols.final_opened_index = F::from_canonical_usize(final_opened_index);
        cols.height = F::from_canonical_usize(height);
        cols.opened_length = F::from_canonical_usize(parent.opened_length);
        cols.dim_base_pointer = F::from_canonical_u32(parent.dim_base_pointer);
        cols.opened_base_pointer = F::from_canonical_u32(parent.opened_base_pointer);
        cols.sibling_base_pointer = F::from_canonical_u32(parent.sibling_base_pointer);
        cols.index_base_pointer = F::from_canonical_u32(parent.index_base_pointer);

        cols.proof_index = F::from_canonical_usize(proof_index);
        cols.read_initial_height_or_root_is_on_right =
            aux_cols_factory.make_read_aux_cols(initial_height_read);
        cols.read_final_height_or_sibling_array_start =
            aux_cols_factory.make_read_aux_cols(final_height_read);
    }
    fn record_to_rows(
        record: VerifyBatchRecord<F>,
        aux_cols_factory: &MemoryAuxColsFactory<F>,
        slice: &mut [F],
        memory: &OfflineMemory<F>,
    ) {
        let width = VerifyBatchCols::<F, SBOX_REGISTERS>::width();

        let Instruction {
            a: a_ptr_ptr,
            b: b_ptr_ptr,
            c: result_ptr,
            d: addr_space,
            e: length_ptr,
            f: alpha_ptr,
            g: alpha_pow_ptr,
            ..
        } = record.instruction;

        let length_read = memory.record_by_id(record.length_read);
        let alpha_read = memory.record_by_id(record.alpha_read);
        let a_ptr_read = memory.record_by_id(record.a_ptr_read);
        let b_ptr_read = memory.record_by_id(record.b_ptr_read);

        let length = length_read.data[0].as_canonical_u32() as usize;
        let alpha: [F; EXT_DEG] = array::from_fn(|i| alpha_read.data[i]);
        let a_ptr = a_ptr_read.data[0];
        let b_ptr = b_ptr_read.data[0];

        let mut alpha_pow_current = record.alpha_pow_original;
        let mut current = [F::ZERO; EXT_DEG];

        let alpha_aux = aux_cols_factory.make_read_aux_cols(alpha_read);
        let length_aux = aux_cols_factory.make_read_aux_cols(length_read);
        let a_ptr_aux = aux_cols_factory.make_read_aux_cols(a_ptr_read);
        let b_ptr_aux = aux_cols_factory.make_read_aux_cols(b_ptr_read);

        let alpha_pow_aux = aux_cols_factory
            .make_write_aux_cols::<EXT_DEG>(memory.record_by_id(record.alpha_pow_write))
            .get_base();
        let result_aux =
            aux_cols_factory.make_write_aux_cols(memory.record_by_id(record.result_write));

        for i in 0..length {
            let a_read = memory.record_by_id(record.a_reads[i]);
            let b_read = memory.record_by_id(record.b_reads[i]);
            let a = a_read.data[0];
            let b: [F; EXT_DEG] = array::from_fn(|i| b_read.data[i]);
            current = FieldExtension::add(
                current,
                FieldExtension::multiply(
                    FieldExtension::subtract(b, elem_to_ext(a)),
                    alpha_pow_current,
                ),
            );

            let mut idx_is_zero = F::ZERO;
            let mut is_zero_aux = F::ZERO;

            let idx = F::from_canonical_usize(i);
            IsZeroSubAir.generate_subrow(idx, (&mut is_zero_aux, &mut idx_is_zero));

            let cols: &mut VerifyBatchCols<F> = slice[i * width..(i + 1) * width].borrow_mut();
            *cols = VerifyBatchCols {
                enabled: F::ONE,
                pc: record.pc,
                a_ptr_ptr,
                b_ptr_ptr,
                result_ptr,
                addr_space,
                length_ptr,
                alpha_ptr,
                alpha_pow_ptr,
                start_timestamp: record.start_timestamp,
                a_ptr_aux,
                b_ptr_aux,
                a_aux: aux_cols_factory.make_read_aux_cols(a_read),
                b_aux: aux_cols_factory.make_read_aux_cols(b_read),
                alpha_aux,
                length_aux,
                alpha_pow_aux,
                result_aux,
                a_ptr,
                b_ptr,
                a,
                b,
                alpha,
                alpha_pow_original: record.alpha_pow_original,
                alpha_pow_current,
                idx,
                idx_is_zero,
                is_zero_aux,
                current,
            };

            alpha_pow_current = FieldExtension::multiply(alpha, alpha_pow_current);
        }
    }

    fn generate_trace(self) -> RowMajorMatrix<F> {
        let width = self.trace_width();
        let height = next_power_of_two_or_zero(self.height);
        let mut flat_trace = F::zero_vec(width * height);

        let memory = self.offline_memory.lock().unwrap();

        let aux_cols_factory = memory.aux_cols_factory();

        let mut idx = 0;
        for record in self.records {
            let length = record.a_reads.len();
            Self::record_to_rows(
                record,
                &aux_cols_factory,
                &mut flat_trace[idx..idx + (length * width)],
                &memory,
            );
            idx += length * width;
        }
        // In padding rows, need idx_is_zero = 1 so IsZero constraints pass, and also because next.idx_is_zero is used
        // to determine the last row per instruction, so the last non-padding row needs next.idx_is_zero = 1
        flat_trace[self.height * width..]
            .par_chunks_mut(width)
            .for_each(|row| {
                let row: &mut VerifyBatchCols<F> = row.borrow_mut();
                row.idx_is_zero = F::ONE;
            });

        RowMajorMatrix::new(flat_trace, width)
    }
}

impl<SC: StarkGenericConfig> Chip<SC> for VerifyBatchChip<Val<SC>>
where
    Val<SC>: PrimeField32,
{
    fn air(&self) -> Arc<dyn AnyRap<SC>> {
        Arc::new(self.air)
    }
    fn generate_air_proof_input(self) -> AirProofInput<SC> {
        AirProofInput::simple_no_pis(self.air(), self.generate_trace())
    }
}
