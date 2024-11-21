use std::{array, borrow::BorrowMut, cell::RefCell, sync::Arc};

use ax_circuit_primitives::utils::next_power_of_two_or_zero;
use ax_hashes::sha256::{
    limbs_into_u32, Sha256Air, Sha256DigestCols, Sha256RoundCols, SHA256_BLOCK_WORDS,
    SHA256_DIGEST_WIDTH, SHA256_H, SHA256_ROUNDS_PER_ROW, SHA256_ROUND_WIDTH,
    SHA256_ROWS_PER_BLOCK, SHA256_WORD_U16S, SHA256_WORD_U8S,
};
use ax_stark_backend::{
    config::{StarkGenericConfig, Val},
    prover::types::AirProofInput,
    rap::{get_air_name, AnyRap},
    Chip, ChipUsageGetter,
};
use axvm_instructions::riscv::RV32_REGISTER_NUM_LIMBS;
use p3_air::BaseAir;
use p3_field::{AbstractField, PrimeField32};
use p3_matrix::dense::RowMajorMatrix;

use super::{
    Sha256VmChip, Sha256VmDigestCols, Sha256VmRoundCols, SHA256VM_DIGEST_WIDTH,
    SHA256VM_ROUND_WIDTH,
};
use crate::intrinsics::hashes::sha256::SHA256_READ_SIZE;

impl<SC: StarkGenericConfig> Chip<SC> for Sha256VmChip<Val<SC>>
where
    Val<SC>: PrimeField32,
{
    fn air(&self) -> Arc<dyn AnyRap<SC>> {
        Arc::new(self.air.clone())
    }

    fn generate_air_proof_input(self) -> AirProofInput<SC> {
        let air = self.air();
        let non_padded_height = self.current_trace_height();
        let height = next_power_of_two_or_zero(non_padded_height);
        let width = self.trace_width();
        let mut values = Val::<SC>::zero_vec(height * width);
        let records = self.records;
        let memory_aux_cols_factory = RefCell::borrow(&self.memory_controller).aux_cols_factory();

        let mut global_block_idx = 0;
        for record in records {
            let mut prev_hash = SHA256_H;
            let message_len = limbs_into_u32(record.len_read.data.map(|x| x.as_canonical_u32()));

            for (i, block_reads) in record.input_message.iter().enumerate() {
                let start_row_idx = global_block_idx * SHA256_ROWS_PER_BLOCK;
                let is_last_block = i == record.input_message.len() - 1;

                let buffer: [[u32; SHA256_WORD_U16S * SHA256_ROUNDS_PER_ROW * 2]; 4] =
                    array::from_fn(|j| {
                        array::from_fn(|k| block_reads[j].data[k].as_canonical_u32())
                    });
                let message: [u32; SHA256_BLOCK_WORDS] = array::from_fn(|j| {
                    limbs_into_u32::<RV32_REGISTER_NUM_LIMBS>(array::from_fn(|k| {
                        block_reads[j / SHA256_ROUNDS_PER_ROW].data[SHA256_WORD_U8S
                            * (j % SHA256_ROUNDS_PER_ROW)
                            + SHA256_WORD_U8S
                            - k
                            - 1]
                        .as_canonical_u32()
                    }))
                });
                // TODO: pad the message
                let (mut inner_trace, final_hash) =
                    self.air.sha256_subair.generate_block_trace::<Val<SC>>(
                        &message,
                        &prev_hash,
                        is_last_block,
                        global_block_idx as u32,
                        i as u32,
                        &buffer,
                    );

                let mut read_ptr = block_reads[0].pointer.as_canonical_u32();
                let mut cur_timestamp = block_reads[0].timestamp;
                for row in 0..SHA256_ROWS_PER_BLOCK {
                    let idx_in_values = width * (start_row_idx + row);
                    if row < 16 {
                        let cols: &mut Sha256VmRoundCols<Val<SC>> = values
                            [idx_in_values..idx_in_values + SHA256VM_ROUND_WIDTH]
                            .borrow_mut();
                        let inner: &mut Sha256RoundCols<Val<SC>> =
                            inner_trace[row][..SHA256_ROUND_WIDTH].borrow_mut();
                        cols.inner = *inner;
                        cols.len[0] = Val::<SC>::from_canonical_u32(message_len & (1 << 6 - 1));
                        cols.len[1] = Val::<SC>::from_canonical_u32(message_len >> 6);
                        cols.read_ptr = Val::<SC>::from_canonical_u32(read_ptr);
                        cols.cur_timestamp = Val::<SC>::from_canonical_u32(cur_timestamp);
                        // TODO: pad_flags
                        if row < 4 {
                            read_ptr += SHA256_READ_SIZE as u32;
                            cur_timestamp += 1;
                            cols.read_aux =
                                memory_aux_cols_factory.make_read_aux_cols(block_reads[row]);
                        }
                    } else {
                        let cols: &mut Sha256VmDigestCols<Val<SC>> = values
                            [idx_in_values..idx_in_values + SHA256VM_DIGEST_WIDTH]
                            .borrow_mut();
                        let inner: &mut Sha256DigestCols<Val<SC>> =
                            inner_trace[row][..SHA256_DIGEST_WIDTH].borrow_mut();
                        cols.inner = *inner;
                        cols.len[0] = Val::<SC>::from_canonical_u32(message_len & (1 << 6 - 1));
                        cols.len[1] = Val::<SC>::from_canonical_u32(message_len >> 6);
                        cols.read_ptr = Val::<SC>::from_canonical_u32(read_ptr);
                        cols.cur_timestamp = Val::<SC>::from_canonical_u32(cur_timestamp);
                        // TODO: pad_flags
                        if is_last_block {
                            cols.from_state = record.from_state;
                            cols.rd_ptr = record.dst_read.pointer;
                            cols.rs1_ptr = record.src_read.pointer;
                            cols.rs2_ptr = record.len_read.pointer;
                            cols.dst_ptr = record.dst_read.data;
                            cols.src_ptr = record.src_read.data;
                            cols.len_cells = record.len_read.data;
                            cols.register_reads_aux = [
                                memory_aux_cols_factory.make_read_aux_cols(record.dst_read),
                                memory_aux_cols_factory.make_read_aux_cols(record.src_read),
                                memory_aux_cols_factory.make_read_aux_cols(record.len_read),
                            ];
                            cols.writes_aux =
                                memory_aux_cols_factory.make_write_aux_cols(record.digest_write);
                        }
                    }
                }
                prev_hash = final_hash;
                global_block_idx += 1;
            }
        }

        // Fill in the padding rows
        for i in non_padded_height..height {
            let rows = &mut values[(i - 1) * width..(i + 1) * width];
            let (local, next) = rows.split_at_mut(width);
            let local_cols: &mut Sha256VmRoundCols<Val<SC>> = local.borrow_mut();
            let next_cols: &mut Sha256VmRoundCols<Val<SC>> = next.borrow_mut();
            Sha256Air::default_row(&local_cols.inner, &mut next_cols.inner);
        }

        // Fill in the count_corrections
        for i in (0..height).step_by(width) {
            let rows = &mut values[i..i + width * 2];
            let (local, next) = rows.split_at_mut(width);
            let local_cols: &mut Sha256VmRoundCols<Val<SC>> = local.borrow_mut();
            let next_cols: &mut Sha256VmRoundCols<Val<SC>> = next.borrow_mut();
            Sha256Air::generate_count_correction::<Val<SC>>(
                &local_cols.inner,
                &mut next_cols.inner,
                if i < non_padded_height { i % 17 } else { 17 },
            );
        }
        AirProofInput::simple(air, RowMajorMatrix::new(values, width), vec![])
    }
}

impl<F: PrimeField32> ChipUsageGetter for Sha256VmChip<F> {
    fn air_name(&self) -> String {
        get_air_name(&self.air)
    }
    fn current_trace_height(&self) -> usize {
        self.records.iter().fold(0, |acc, record| {
            acc + record.input_message.len() * SHA256_ROWS_PER_BLOCK
        })
    }

    fn trace_width(&self) -> usize {
        BaseAir::<F>::width(&self.air)
    }
}
