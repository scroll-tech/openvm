use std::{array, borrow::Borrow, cmp::min};

use ax_circuit_primitives::{
    bitwise_op_lookup::BitwiseOperationLookupBus, encoder::Encoder, utils::not, SubAir,
};
use ax_hashes::sha256::{
    compose, contains_flag, contains_flag_range, flag_with_val, Sha256Air, SHA256_HASH_WORDS,
    SHA256_WIDTH, SHA256_WORD_BITS, SHA256_WORD_U16S, SHA256_WORD_U8S,
};
use ax_stark_backend::{
    air_builders::sub::SubAirBuilder,
    interaction::InteractionBuilder,
    rap::{BaseAirWithPublicValues, PartitionedBaseAir},
};
use axvm_instructions::{
    riscv::{RV32_CELL_BITS, RV32_REGISTER_NUM_LIMBS},
    Rv32Sha256Opcode,
};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use super::{
    Sha256VmDigestCols, Sha256VmRoundCols, SHA256VM_CONTROL_WIDTH, SHA256VM_DIGEST_WIDTH,
    SHA256VM_ROUND_WIDTH, SHA256VM_WIDTH, SHA256_READ_SIZE,
};
use crate::{
    arch::ExecutionBridge,
    system::memory::{offline_checker::MemoryBridge, MemoryAddress},
};

#[derive(Clone, Debug, derive_new::new)]
pub struct Sha256VmAir {
    pub execution_bridge: ExecutionBridge,
    pub memory_bridge: MemoryBridge,
    /// Bus to send byte checks to
    pub bitwise_lookup_bus: BitwiseOperationLookupBus,
    /// Maximum number of bits allowed for an address pointer
    pub ptr_max_bits: usize,
    pub(super) offset: usize,
    pub(super) sha256_subair: Sha256Air,
    pub(super) padding_encoder: Encoder,
}

impl<F: Field> BaseAirWithPublicValues<F> for Sha256VmAir {}
impl<F: Field> PartitionedBaseAir<F> for Sha256VmAir {}
impl<F: Field> BaseAir<F> for Sha256VmAir {
    fn width(&self) -> usize {
        SHA256VM_WIDTH
    }
}

impl<AB: InteractionBuilder> Air<AB> for Sha256VmAir {
    fn eval(&self, builder: &mut AB) {
        // self.eval_padding(builder);
        self.eval_transitions(builder);
        self.eval_reads(builder);
        self.eval_last_row(builder);

        self.sha256_subair.eval(builder, SHA256VM_CONTROL_WIDTH);
    }
}

/// Note: 4th row versions need to be seperated to keep the degree
#[allow(dead_code)]
pub(super) enum PaddingFlags {
    /// Not considered for padding
    NotConsidered,
    /// Not padding - should be equal to the message
    NotPadding,
    /// FIRST_PADDING_i: it is the first row with padding and there are i cells of non-padding
    FirstPadding0,
    FirstPadding1,
    FirstPadding2,
    FirstPadding3,
    FirstPadding4,
    FirstPadding5,
    FirstPadding6,
    FirstPadding7,
    FirstPadding8,
    FirstPadding9,
    FirstPadding10,
    FirstPadding11,
    FirstPadding12,
    FirstPadding13,
    FirstPadding14,
    FirstPadding15,
    /// FIRST_PADDING_i: it is the first row with padding and there are i cells of non-padding and it is the 4th row in the block
    FirstPadding0_4thRow,
    FirstPadding1_4thRow,
    FirstPadding2_4thRow,
    FirstPadding3_4thRow,
    FirstPadding4_4thRow,
    FirstPadding5_4thRow,
    FirstPadding6_4thRow,
    FirstPadding7_4thRow,
    /// NOTE: if the 4th row has padding it has to be at least 9 cells
    /// The entire row is padding and it is not the first row with padding and it is the 4th row in the block
    EntirePadding4thRow,
    /// The entire row is padding and it is not the first row with padding
    EntirePadding,
}

impl PaddingFlags {
    /// The number of padding flags (not including NotConsidered)
    pub const COUNT: usize = EntirePadding as usize;
}

use PaddingFlags::*;
impl Sha256VmAir {
    fn eval_padding<AB: InteractionBuilder>(&self, builder: &mut AB) {
        let main = builder.main();
        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local_cols: &Sha256VmRoundCols<AB::Var> = local[..SHA256VM_ROUND_WIDTH].borrow();
        let next_cols: &Sha256VmRoundCols<AB::Var> = next[..SHA256VM_ROUND_WIDTH].borrow();

        let message: [AB::Var; SHA256_READ_SIZE] = array::from_fn(|i| {
            local_cols.inner.message_schedule.carry_or_buffer[i / (SHA256_WORD_U16S * 2)]
                [i % (SHA256_WORD_U16S * 2)]
        });

        let get_ith_byte = |i: usize| {
            let word_idx = i / SHA256_WORD_BITS;
            let word = local_cols.inner.message_schedule.w[word_idx].map(|x| x.into());
            // Need to reverse the byte order to match the endianness of the memory
            let byte_idx = 4 - i % 4 - 1;
            compose::<AB>(&word[byte_idx * 8..(byte_idx + 1) * 8], 1)
        };

        builder.assert_bool(local_cols.control.is_padding);
        // When we are not in the last block or the first 4 rows, there should be no padding
        builder
            .when(not::<AB::Expr>(local_cols.inner.flags.is_last_block))
            .assert_zero(local_cols.control.is_padding);
        builder
            .when(not::<AB::Expr>(local_cols.inner.flags.is_first_4_rows))
            .assert_zero(local_cols.control.is_padding);
        // When current row is padding, the next row should also be padding (in the first 4 rows)
        builder
            .when(local_cols.control.is_padding)
            .when(next_cols.inner.flags.is_first_4_rows)
            .assert_one(next_cols.control.is_padding);

        let is_row_idx_4 = contains_flag::<AB>(
            &self.sha256_subair.row_idx_encoder,
            &local_cols.inner.flags.row_idx,
            &[3],
        );

        // Assert that on the 4th row of the last block we have padding
        builder
            .when(local_cols.inner.flags.is_last_block)
            .assert_eq(is_row_idx_4.clone(), local_cols.control.is_padding);

        // indicates if the next row is the first row with padding
        let is_first_padding = next_cols.control.is_padding * not(local_cols.control.is_padding);
        self.padding_encoder
            .eval(builder, &local_cols.control.pad_flags);

        let is_first_paddding_flag = contains_flag_range::<AB>(
            &self.padding_encoder,
            &local_cols.control.pad_flags,
            FirstPadding0 as usize,
            FirstPadding7_4thRow as usize,
        );
        let is_4th_row_flag = contains_flag_range::<AB>(
            &self.padding_encoder,
            &local_cols.control.pad_flags,
            FirstPadding0_4thRow as usize,
            EntirePadding4thRow as usize,
        );
        let is_not_considered_flag = contains_flag::<AB>(
            &self.padding_encoder,
            &local_cols.control.pad_flags,
            &[NotConsidered as usize],
        );

        let is_not_padding_flag = contains_flag::<AB>(
            &self.padding_encoder,
            &local_cols.control.pad_flags,
            &[NotPadding as usize],
        );

        // Padding flags should be one of the options define in [PaddingFlags]
        builder.assert_one(contains_flag_range::<AB>(
            &self.padding_encoder,
            &local_cols.control.pad_flags,
            NotConsidered as usize,
            EntirePadding4thRow as usize,
        ));

        // When we are not in the first 4 rows it shouldn't be considered for padding
        builder
            .when(not::<AB::Expr>(local_cols.inner.flags.is_first_4_rows))
            .assert_one(is_not_considered_flag.clone());
        // When we are in the first 4 rows it should be considered for padding
        builder
            .when(local_cols.inner.flags.is_first_4_rows)
            .assert_zero(is_not_considered_flag.clone());

        // Constrains that `is_padding` and the padding flags match
        builder
            .when(not(local_cols.control.is_padding))
            .assert_one(is_not_padding_flag.clone() + is_not_considered_flag.clone());

        // Constrains that `is_padding` and the padding flags match
        builder
            .when(local_cols.control.is_padding)
            .assert_zero(is_not_padding_flag.clone() + is_not_considered_flag.clone());

        builder.assert_eq(is_first_padding, is_first_paddding_flag.clone());

        builder
            .when(local_cols.control.is_padding)
            .assert_eq(is_4th_row_flag.clone(), is_row_idx_4);

        // Check the `w`s on case by case basis
        for i in 0..SHA256_READ_SIZE {
            let w = get_ith_byte(i);
            let should_be_message = is_not_padding_flag.clone()
                + if i < 15 {
                    contains_flag_range::<AB>(
                        &self.padding_encoder,
                        &local_cols.control.pad_flags,
                        FirstPadding0 as usize + i + 1,
                        FirstPadding15 as usize,
                    )
                } else {
                    AB::Expr::ZERO
                }
                + if i < 7 {
                    contains_flag_range::<AB>(
                        &self.padding_encoder,
                        &local_cols.control.pad_flags,
                        FirstPadding0_4thRow as usize + i + 1,
                        FirstPadding7_4thRow as usize,
                    )
                } else {
                    AB::Expr::ZERO
                };
            builder
                .when(should_be_message)
                .assert_eq(w.clone(), message[i]);

            let should_be_zero = contains_flag::<AB>(
                &self.padding_encoder,
                &local_cols.control.pad_flags,
                &[EntirePadding as usize],
            ) + if i < 12 {
                contains_flag::<AB>(
                    &self.padding_encoder,
                    &local_cols.control.pad_flags,
                    &[EntirePadding4thRow as usize],
                ) + if i > 0 {
                    contains_flag_range::<AB>(
                        &self.padding_encoder,
                        &local_cols.control.pad_flags,
                        FirstPadding0_4thRow as usize,
                        min(
                            FirstPadding0_4thRow as usize + i - 1,
                            FirstPadding7_4thRow as usize,
                        ),
                    )
                } else {
                    AB::Expr::ZERO
                }
            } else {
                AB::Expr::ZERO
            } + if i > 0 {
                contains_flag_range::<AB>(
                    &self.padding_encoder,
                    &local_cols.control.pad_flags,
                    FirstPadding0 as usize,
                    FirstPadding0 as usize + i - 1,
                )
            } else {
                AB::Expr::ZERO
            };
            builder.when(should_be_zero).assert_zero(w.clone());

            let should_be_128 = contains_flag::<AB>(
                &self.padding_encoder,
                &local_cols.control.pad_flags,
                &[FirstPadding0 as usize + i],
            ) + if i < 8 {
                contains_flag::<AB>(
                    &self.padding_encoder,
                    &local_cols.control.pad_flags,
                    &[FirstPadding0_4thRow as usize + i],
                )
            } else {
                AB::Expr::ZERO
            };
            builder
                .when(should_be_128)
                .assert_eq(w.clone(), AB::Expr::from_canonical_u32(1 << 7));

            // should be len is handled outside of the loop
        }
        let appended_len = compose::<AB>(
            &[
                get_ith_byte(15),
                get_ith_byte(14),
                get_ith_byte(13),
                get_ith_byte(12),
            ],
            RV32_CELL_BITS,
        );

        let len_val = local_cols.control.len[0]
            + local_cols.control.len[1] * AB::Expr::from_canonical_u32(1 << 6);
        builder
            .when(is_4th_row_flag.clone())
            .assert_eq(appended_len, len_val);

        // Finally, check that the amount of padding with respect to the length of the message
        let row_idx_val = flag_with_val::<AB>(
            &self.sha256_subair.row_idx_encoder,
            &local_cols.inner.flags.row_idx,
            &(0..4).map(|x| (x, x)).collect::<Vec<_>>(),
        );

        let row_message_amount = flag_with_val::<AB>(
            &self.padding_encoder,
            &local_cols.control.pad_flags,
            &(0..16)
                .map(|i| (FirstPadding0 as usize + i, i))
                .collect::<Vec<_>>(),
        ) + flag_with_val::<AB>(
            &self.padding_encoder,
            &local_cols.control.pad_flags,
            &(0..8)
                .map(|i| (FirstPadding0_4thRow as usize + i, i))
                .collect::<Vec<_>>(),
        );
        let got_length =
            row_idx_val * AB::Expr::from_canonical_usize(SHA256_READ_SIZE) + row_message_amount;
        builder
            .when(is_4th_row_flag)
            .assert_eq(got_length, local_cols.control.len[0]);
    }

    /// Implement constraints on `len`, `read_ptr` and `cur_timestamp`
    fn eval_transitions<AB: InteractionBuilder>(&self, builder: &mut AB) {
        let main = builder.main();
        let (local, next) = (main.row_slice(0), main.row_slice(1));
        let local_cols: &Sha256VmRoundCols<AB::Var> = local[..SHA256VM_ROUND_WIDTH].borrow();
        let next_cols: &Sha256VmRoundCols<AB::Var> = next[..SHA256VM_ROUND_WIDTH].borrow();

        let is_last_row =
            local_cols.inner.flags.is_last_block * local_cols.inner.flags.is_digest_row;
        // Len should be the same for the entire message
        for (local_limb, next_limb) in local_cols
            .control
            .len
            .iter()
            .zip(next_cols.control.len.iter())
        {
            builder
                .when_transition()
                .when(not::<AB::Expr>(is_last_row.clone()))
                .assert_eq(local_limb.clone(), next_limb.clone());
        }

        // Read ptr should increment by [SHA256_READ_SIZE] for the first 4 rows and stay the same otherwise
        let read_ptr_delta = local_cols.inner.flags.is_first_4_rows
            * AB::Expr::from_canonical_usize(SHA256_READ_SIZE);
        builder
            .when_transition()
            .when(not::<AB::Expr>(is_last_row.clone()))
            .assert_eq(
                next_cols.control.read_ptr,
                local_cols.control.read_ptr + read_ptr_delta,
            );

        // Timestamp should increment by 1 for the first 4 rows and stay the same otherwise
        let timestamp_delta = local_cols.inner.flags.is_first_4_rows * AB::Expr::ONE;
        builder
            .when_transition()
            .when(not::<AB::Expr>(is_last_row.clone()))
            .assert_eq(
                next_cols.control.cur_timestamp,
                local_cols.control.cur_timestamp + timestamp_delta,
            );
    }

    /// Implement the reads for the first 4 rows of a block
    fn eval_reads<AB: InteractionBuilder>(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let local_cols: &Sha256VmRoundCols<AB::Var> = local[..SHA256VM_ROUND_WIDTH].borrow();

        let message: [AB::Var; SHA256_READ_SIZE] = array::from_fn(|i| {
            local_cols.inner.message_schedule.carry_or_buffer[i / (SHA256_WORD_U16S * 2)]
                [i % (SHA256_WORD_U16S * 2)]
        });

        self.memory_bridge
            .read(
                MemoryAddress::new(AB::Expr::TWO, local_cols.control.read_ptr),
                message,
                local_cols.control.cur_timestamp,
                &local_cols.read_aux,
            )
            .eval(builder, local_cols.inner.flags.is_first_4_rows);
    }
    /// Implement the constraints for the last row of a message
    fn eval_last_row<AB: InteractionBuilder>(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let local_cols: &Sha256VmDigestCols<AB::Var> = local[..SHA256VM_DIGEST_WIDTH].borrow();

        let timestamp: AB::Var = local_cols.from_state.timestamp;
        let mut timestamp_delta: usize = 0;
        let mut timestamp_pp = || {
            timestamp_delta += 1;
            timestamp + AB::Expr::from_canonical_usize(timestamp_delta - 1)
        };

        let is_last_row =
            local_cols.inner.flags.is_last_block * local_cols.inner.flags.is_digest_row;

        self.memory_bridge
            .read(
                MemoryAddress::new(AB::Expr::ONE, local_cols.rd_ptr),
                local_cols.dst_ptr,
                timestamp_pp(),
                &local_cols.register_reads_aux[0],
            )
            .eval(builder, is_last_row.clone());

        self.memory_bridge
            .read(
                MemoryAddress::new(AB::Expr::ONE, local_cols.rs1_ptr),
                local_cols.src_ptr,
                timestamp_pp(),
                &local_cols.register_reads_aux[1],
            )
            .eval(builder, is_last_row.clone());

        self.memory_bridge
            .read(
                MemoryAddress::new(AB::Expr::ONE, local_cols.rs2_ptr),
                local_cols.len_data,
                timestamp_pp(),
                &local_cols.register_reads_aux[2],
            )
            .eval(builder, is_last_row.clone());

        // range check that the memory pointers don't overflow
        // Note: no need to range check the length since we read from memory step by step and
        //       the memory bus will catch any memory accesses beyond ptr_max_bits
        let shift = AB::Expr::from_canonical_usize(
            1 << (RV32_REGISTER_NUM_LIMBS * RV32_CELL_BITS - self.ptr_max_bits),
        );
        self.bitwise_lookup_bus
            .send_range(
                // It is fine to shift like this since we already know that dst_ptr and src_ptr have [RV32_CELL_BITS] bits
                local_cols.dst_ptr[RV32_REGISTER_NUM_LIMBS - 1] * shift.clone(),
                local_cols.src_ptr[RV32_REGISTER_NUM_LIMBS - 1] * shift.clone(),
            )
            .eval(builder, is_last_row.clone());

        // the number of reads that happened to read the entire message: we do 4 reads per block
        let time_delta =
            (local_cols.inner.local_block_idx + AB::Expr::ONE) * AB::Expr::from_canonical_usize(4);
        // Every time we read the message we increment the read pointer by SHA256_READ_SIZE
        let read_ptr_delta = time_delta.clone() * AB::Expr::from_canonical_usize(SHA256_READ_SIZE);

        let result: [AB::Var; SHA256_WORD_U8S * SHA256_HASH_WORDS] = array::from_fn(|i| {
            // The limbs are written in big endian order to the memory so need to be reversed
            local_cols.inner.final_hash[i / SHA256_WORD_U8S]
                [SHA256_WORD_U8S - i % SHA256_WORD_U8S - 1]
        });

        let dst_ptr_val = compose::<AB>(&local_cols.dst_ptr.map(|x| x.into()), RV32_CELL_BITS);

        self.memory_bridge
            .write(
                MemoryAddress::new(AB::Expr::TWO, dst_ptr_val),
                result,
                timestamp_pp() + time_delta.clone(),
                &local_cols.writes_aux,
            )
            .eval(builder, is_last_row.clone());

        self.execution_bridge
            .execute_and_increment_pc(
                AB::Expr::from_canonical_usize(Rv32Sha256Opcode::SHA256 as usize + self.offset),
                [
                    local_cols.rd_ptr.into(),
                    local_cols.rs1_ptr.into(),
                    local_cols.rs2_ptr.into(),
                    AB::Expr::ONE,
                    AB::Expr::TWO,
                ],
                local_cols.from_state,
                AB::Expr::from_canonical_usize(timestamp_delta) + time_delta.clone(),
            )
            .eval(builder, is_last_row.clone());

        // Assert that we read the correct length of the message
        let len_val = compose::<AB>(&local_cols.len_data.map(|x| x.into()), RV32_CELL_BITS);
        builder.when(is_last_row.clone()).assert_eq(
            local_cols.control.len[0]
                + local_cols.control.len[1] * AB::Expr::from_canonical_u32(1 << 6),
            len_val,
        );
        // Assert that we started reading from the correct pointer initially
        let src_val = compose::<AB>(&local_cols.src_ptr.map(|x| x.into()), RV32_CELL_BITS);
        builder
            .when(is_last_row.clone())
            .assert_eq(local_cols.control.read_ptr, src_val + read_ptr_delta);
        // Assert that we started reading from the correct timestamp
        builder.when(is_last_row.clone()).assert_eq(
            local_cols.control.cur_timestamp,
            local_cols.from_state.timestamp + AB::Expr::from_canonical_u32(3) + time_delta,
        );
    }
}
