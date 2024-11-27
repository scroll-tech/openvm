use std::{array, borrow::BorrowMut, cell::RefCell, sync::Arc};

use ax_circuit_primitives::utils::next_power_of_two_or_zero;
use ax_hashes::sha256::{
    get_flag_pt_array, limbs_into_u32, Sha256Air, Sha256DigestCols, Sha256RoundCols,
    SHA256_BLOCK_WORDS, SHA256_DIGEST_WIDTH, SHA256_H, SHA256_ROUNDS_PER_ROW, SHA256_ROUND_WIDTH,
    SHA256_ROWS_PER_BLOCK, SHA256_WORD_U16S, SHA256_WORD_U8S,
};
use ax_stark_backend::{
    config::{StarkGenericConfig, Val},
    prover::types::AirProofInput,
    rap::{get_air_name, AnyRap},
    Chip, ChipUsageGetter,
};
use axvm_instructions::riscv::{RV32_CELL_BITS, RV32_REGISTER_NUM_LIMBS};
use p3_air::BaseAir;
use p3_field::{AbstractField, PrimeField32};
use p3_matrix::dense::RowMajorMatrix;

use super::{
    Sha256VmChip, Sha256VmDigestCols, Sha256VmRoundCols, SHA256VM_DIGEST_WIDTH,
    SHA256VM_ROUND_WIDTH,
};
use crate::intrinsics::hashes::sha256::{PaddingFlags, SHA256_BLOCK_CELLS, SHA256_READ_SIZE};

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
        let mem_ptr_shift: u32 =
            1 << (RV32_REGISTER_NUM_LIMBS * RV32_CELL_BITS - self.air.ptr_max_bits);
        for record in records {
            let mut prev_hash = SHA256_H;

            self.bitwise_lookup_chip.request_range(
                record.dst_read.data[RV32_REGISTER_NUM_LIMBS - 1].as_canonical_u32()
                    * mem_ptr_shift,
                record.src_read.data[RV32_REGISTER_NUM_LIMBS - 1].as_canonical_u32()
                    * mem_ptr_shift,
            );
            let message_len = limbs_into_u32(record.len_read.data.map(|x| x.as_canonical_u32()));
            for (i, block_reads) in record.input_message.iter().enumerate() {
                let start_row_idx = global_block_idx * SHA256_ROWS_PER_BLOCK;
                let is_last_block = i == (record.input_message.len() - 1);

                let buffer: [[u32; SHA256_WORD_U16S * SHA256_ROUNDS_PER_ROW * 2]; 4] =
                    array::from_fn(|j| {
                        array::from_fn(|k| block_reads[j].data[k].as_canonical_u32())
                    });

                let padded_message_bytes: [u32; SHA256_BLOCK_WORDS * SHA256_WORD_U8S] =
                    array::from_fn(|j| {
                        if i * SHA256_BLOCK_CELLS + j < message_len as usize {
                            block_reads[j / SHA256_READ_SIZE].data[j % SHA256_READ_SIZE]
                                .as_canonical_u32()
                        } else if i * SHA256_BLOCK_CELLS + j == message_len as usize {
                            1 << (RV32_CELL_BITS - 1)
                        } else {
                            let mask_idx = SHA256_BLOCK_WORDS * SHA256_WORD_U8S - j - 1;
                            (message_len * RV32_CELL_BITS as u32)
                                .checked_shr((RV32_CELL_BITS * mask_idx) as u32)
                                .unwrap_or(0)
                                & ((1 << RV32_CELL_BITS) - 1)
                        }
                    });

                println!("padded_message_bytes: {:?}", padded_message_bytes);
                let padded_message: [u32; SHA256_BLOCK_WORDS] = array::from_fn(|j| {
                    limbs_into_u32::<RV32_REGISTER_NUM_LIMBS>(array::from_fn(|k| {
                        padded_message_bytes[(j + 1) * SHA256_WORD_U8S - k - 1]
                    }))
                });

                // TODO: Padding flags are not correct!!
                let last_block_len = message_len as usize % (SHA256_BLOCK_WORDS * SHA256_WORD_U8S);

                let (mut inner_trace, final_hash) =
                    self.air.sha256_subair.generate_block_trace::<Val<SC>>(
                        &padded_message,
                        self.bitwise_lookup_chip.clone(),
                        &prev_hash,
                        is_last_block,
                        global_block_idx as u32 + 1,
                        i as u32,
                        &buffer,
                    );

                println!("final_hash: {:?}", final_hash);
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
                        cols.control.len[0] =
                            Val::<SC>::from_canonical_u32(message_len & ((1 << 6) - 1));
                        cols.control.len[1] = Val::<SC>::from_canonical_u32(message_len >> 6);
                        cols.control.read_ptr = Val::<SC>::from_canonical_u32(read_ptr);
                        cols.control.cur_timestamp = Val::<SC>::from_canonical_u32(cur_timestamp);
                        if row < 4 {
                            read_ptr += SHA256_READ_SIZE as u32;
                            cur_timestamp += 1;
                            cols.read_aux =
                                memory_aux_cols_factory.make_read_aux_cols(block_reads[row]);
                            if !is_last_block || (row + 1) * SHA256_ROUNDS_PER_ROW <= last_block_len
                            {
                                cols.control.pad_flags = get_flag_pt_array(
                                    &self.air.padding_encoder,
                                    PaddingFlags::NotPadding as usize,
                                )
                                .map(Val::<SC>::from_canonical_u32);
                            } else if row * SHA256_ROUNDS_PER_ROW > last_block_len {
                                cols.control.pad_flags = get_flag_pt_array(
                                    &self.air.padding_encoder,
                                    if row == 4 {
                                        PaddingFlags::EntirePadding4thRow
                                    } else {
                                        PaddingFlags::EntirePadding
                                    } as usize,
                                )
                                .map(Val::<SC>::from_canonical_u32);
                            } else {
                                let last_row_len = last_block_len % SHA256_READ_SIZE;
                                cols.control.pad_flags = get_flag_pt_array(
                                    &self.air.padding_encoder,
                                    if row == 4 {
                                        PaddingFlags::FirstPadding0_4thRow
                                    } else {
                                        PaddingFlags::FirstPadding0
                                    } as usize
                                        + last_row_len,
                                )
                                .map(Val::<SC>::from_canonical_u32);
                            }
                        } else {
                            cols.control.pad_flags = get_flag_pt_array(
                                &self.air.padding_encoder,
                                PaddingFlags::NotConsidered as usize,
                            )
                            .map(Val::<SC>::from_canonical_u32);
                        }
                        // println!("pad_flags: {:?}", cols.pad_flags);
                    } else {
                        let cols: &mut Sha256VmDigestCols<Val<SC>> = values
                            [idx_in_values..idx_in_values + SHA256VM_DIGEST_WIDTH]
                            .borrow_mut();
                        let inner: &mut Sha256DigestCols<Val<SC>> =
                            inner_trace[row][..SHA256_DIGEST_WIDTH].borrow_mut();
                        cols.inner = *inner;
                        cols.control.len[0] =
                            Val::<SC>::from_canonical_u32(message_len & ((1 << 6) - 1));
                        cols.control.len[1] = Val::<SC>::from_canonical_u32(message_len >> 6);
                        cols.control.read_ptr = Val::<SC>::from_canonical_u32(read_ptr);
                        cols.control.cur_timestamp = Val::<SC>::from_canonical_u32(cur_timestamp);
                        cols.control.pad_flags = get_flag_pt_array(
                            &self.air.padding_encoder,
                            PaddingFlags::NotConsidered as usize,
                        )
                        .map(Val::<SC>::from_canonical_u32);
                        if is_last_block {
                            cols.from_state = record.from_state;
                            cols.rd_ptr = record.dst_read.pointer;
                            cols.rs1_ptr = record.src_read.pointer;
                            cols.rs2_ptr = record.len_read.pointer;
                            cols.dst_ptr = record.dst_read.data;
                            cols.src_ptr = record.src_read.data;
                            cols.len_data = record.len_read.data;
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

        println!(
            "non_padded_height: {}, height: {}, width: {}, values.len(): {}",
            non_padded_height,
            height,
            width,
            values.len()
        );
        // Fill in the padding rows
        for i in non_padded_height..height {
            let rows = &mut values[(i - 1) * width..(i + 1) * width];
            let (local, next) = rows.split_at_mut(width);
            let local_cols: &mut Sha256VmRoundCols<Val<SC>> = local.borrow_mut();
            let next_cols: &mut Sha256VmRoundCols<Val<SC>> = next.borrow_mut();
            self.air
                .sha256_subair
                .default_row(&local_cols.inner, &mut next_cols.inner);
        }

        // // Fill in the w_3 and intermed_4
        // for i in (0..height - 1).step_by(width) {
        //     let rows = &mut values[i * width..(i + 2) * width];
        //     let (local, next) = rows.split_at_mut(width);
        //     let local_cols: &mut Sha256VmRoundCols<Val<SC>> = local.borrow_mut();
        //     let next_cols: &mut Sha256VmRoundCols<Val<SC>> = next.borrow_mut();
        //     Sha256Air::generate_w_3::<Val<SC>>(&local_cols.inner, &mut next_cols.inner);
        //     Sha256Air::generate_intermed_4::<Val<SC>>(&local_cols.inner, &mut next_cols.inner);
        // }
        // // Fill in w_3 and intermed_4 for the last row
        // let (first, rest) = values.split_at_mut(width);
        // let rest_len = rest.len();
        // let last = &mut rest[rest_len - width..];
        // let local_cols: &mut Sha256VmRoundCols<Val<SC>> = first.borrow_mut();
        // let next_cols: &mut Sha256VmRoundCols<Val<SC>> = last.borrow_mut();
        // Sha256Air::generate_w_3::<Val<SC>>(&local_cols.inner, &mut next_cols.inner);
        // Sha256Air::generate_intermed_4::<Val<SC>>(&local_cols.inner, &mut next_cols.inner);

        // // Fill in intermed_8
        // for i in (0..height - 1).step_by(width) {
        //     let rows = &mut values[i * width..(i + 2) * width];
        //     let (local, next) = rows.split_at_mut(width);
        //     let local_cols: &mut Sha256VmRoundCols<Val<SC>> = local.borrow_mut();
        //     let next_cols: &mut Sha256VmRoundCols<Val<SC>> = next.borrow_mut();
        //     Sha256Air::generate_intermed_8::<Val<SC>>(&local_cols.inner, &mut next_cols.inner);
        // }

        // // Fill in intermed_8 for the last row
        // let (first, rest) = values.split_at_mut(width);
        // let rest_len = rest.len();
        // let last = &mut rest[rest_len - width..];
        // let local_cols: &mut Sha256VmRoundCols<Val<SC>> = first.borrow_mut();
        // let next_cols: &mut Sha256VmRoundCols<Val<SC>> = last.borrow_mut();
        // Sha256Air::generate_intermed_8::<Val<SC>>(&local_cols.inner, &mut next_cols.inner);

        // // Fill in intermed_12
        // for i in (0..height - 1).step_by(width) {
        //     let rows = &mut values[i * width..(i + 2) * width];
        //     let (local, next) = rows.split_at_mut(width);
        //     let local_cols: &mut Sha256VmRoundCols<Val<SC>> = local.borrow_mut();
        //     let next_cols: &mut Sha256VmRoundCols<Val<SC>> = next.borrow_mut();
        //     Sha256Air::generate_intermed_12::<Val<SC>>(&mut local_cols.inner, &next_cols.inner);
        // }

        // // Fill in intermed_12 for the last row
        // let (first, rest) = values.split_at_mut(width);
        // let rest_len = rest.len();
        // let last = &mut rest[rest_len - width..];
        // let local_cols: &mut Sha256VmRoundCols<Val<SC>> = first.borrow_mut();
        // let next_cols: &mut Sha256VmRoundCols<Val<SC>> = last.borrow_mut();
        // Sha256Air::generate_intermed_12::<Val<SC>>(&mut local_cols.inner, &mut next_cols.inner);

        for i in (0..values.len()).step_by(width) {
            let rows = &mut values[i..i + width];
            let local_cols: &mut Sha256VmDigestCols<Val<SC>> =
                rows[..SHA256VM_DIGEST_WIDTH].borrow_mut();
            // println!("local_cols: {:?}", local_cols.pad_flags);
            println!(
                "len: {:?}, cur_timestamp: {:?}, flags: {:?}, local_block_idx: {:?}, from_state: {:?}",
                local_cols.control.len, local_cols.control.cur_timestamp, local_cols.inner.flags, local_cols.inner.local_block_idx, local_cols.from_state
            );
            if local_cols.inner.flags.is_digest_row.as_canonical_u32() != 0 {
                println!(
                    "prev_hash: {:?}, final_hash: {:?}",
                    local_cols.inner.prev_hash, local_cols.inner.final_hash
                );
            }
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
