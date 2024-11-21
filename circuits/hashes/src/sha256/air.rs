use std::{array, borrow::Borrow, cmp::max, iter::once};

use ax_circuit_primitives::{
    bitwise_op_lookup::BitwiseOperationLookupBus,
    encoder::Encoder,
    utils::{not, select},
    SubAir,
};
use ax_stark_backend::interaction::InteractionBuilder;
use p3_air::{AirBuilder, BaseAir};
use p3_field::{AbstractField, Field};
use p3_matrix::Matrix;

use super::{
    compose, contains_flag, flag_with_val, u32_into_limbs, Sha256DigestCols, Sha256RoundCols,
    SHA256_H, SHA256_HASH_WORDS, SHA256_K, SHA256_ROUNDS_PER_ROW, SHA256_WORD_BITS,
    SHA256_WORD_U16S, SHA256_WORD_U8S,
};

#[derive(Clone, Debug)]
pub struct Sha256Air {
    pub bitwise_lookup_bus: BitwiseOperationLookupBus,
    pub row_idx_encoder: Encoder,
    bus_idx: usize,
}

impl Sha256Air {
    pub fn new(bitwise_lookup_bus: BitwiseOperationLookupBus, self_bus_idx: usize) -> Self {
        Self {
            bitwise_lookup_bus,
            row_idx_encoder: Encoder::new(17, 2),
            bus_idx: self_bus_idx,
        }
    }
}

impl<F: Field> BaseAir<F> for Sha256Air {
    fn width(&self) -> usize {
        max(
            Sha256RoundCols::<F>::width(),
            Sha256DigestCols::<F>::width(),
        )
    }
}

impl<AB: InteractionBuilder> SubAir<AB> for Sha256Air {
    type AirContext<'a>
        = ()
    where
        Self: 'a,
        AB: 'a,
        <AB as AirBuilder>::Var: 'a,
        <AB as AirBuilder>::Expr: 'a;

    fn eval<'a>(&'a self, builder: &'a mut AB, _ctx: Self::AirContext<'a>)
    where
        <AB as AirBuilder>::Var: 'a,
        <AB as AirBuilder>::Expr: 'a,
    {
        self.eval_row(builder);
        self.eval_transitions(builder);
    }
}

impl Sha256Air {
    /// Rotates `bits` right by `n` bits, assumes `bits` is in little-endian
    #[inline]
    fn rotr<AB: InteractionBuilder>(
        bits: &[AB::Var; SHA256_WORD_BITS],
        n: usize,
    ) -> [AB::Expr; SHA256_WORD_BITS] {
        array::from_fn(|i| bits[(i + n) % SHA256_WORD_BITS].into())
    }

    /// Shifts `bits` right by `n` bits, assumes `bits` is in little-endian
    #[inline]
    fn shr<AB: InteractionBuilder>(
        bits: &[AB::Var; SHA256_WORD_BITS],
        n: usize,
    ) -> [AB::Expr; SHA256_WORD_BITS] {
        array::from_fn(|i| {
            if i + n < SHA256_WORD_BITS {
                bits[i + n].into()
            } else {
                AB::Expr::ZERO
            }
        })
    }

    /// Computes x ^ y ^ z, where x, y, z are single bits
    #[inline]
    fn xor_single<AB: InteractionBuilder>(x: AB::Expr, y: AB::Expr, z: AB::Expr) -> AB::Expr {
        (x.clone() * y.clone() * z.clone())
            + (x.clone() * y.clone() * not::<AB::Expr>(z.clone()))
            + (x.clone() * not::<AB::Expr>(y.clone()) * z.clone())
            + (not::<AB::Expr>(x.clone()) * y.clone() * z.clone())
    }

    /// Computes x ^ y ^ z, where x, y, z are [SHA256_WORD_BITS] bit numbers
    #[inline]
    fn xor<AB: InteractionBuilder>(
        x: &[AB::Expr; SHA256_WORD_BITS],
        y: &[AB::Expr; SHA256_WORD_BITS],
        z: &[AB::Expr; SHA256_WORD_BITS],
    ) -> [AB::Expr; SHA256_WORD_BITS] {
        array::from_fn(|i| Self::xor_single::<AB>(x[i].clone(), y[i].clone(), z[i].clone()))
    }

    /// Computes Ch(x,y,z), where x, y, z are [SHA256_WORD_BITS] bit numbers
    #[inline]
    fn ch<AB: InteractionBuilder>(
        x: &[AB::Var; SHA256_WORD_BITS],
        y: &[AB::Var; SHA256_WORD_BITS],
        z: &[AB::Var; SHA256_WORD_BITS],
    ) -> [AB::Expr; SHA256_WORD_BITS] {
        array::from_fn(|i| select(x[i], y[i], z[i]))
    }

    /// Computes Maj(x,y,z), where x, y, z are [SHA256_WORD_BITS] bit numbers
    #[inline]
    fn maj<AB: InteractionBuilder>(
        x: &[AB::Var; SHA256_WORD_BITS],
        y: &[AB::Var; SHA256_WORD_BITS],
        z: &[AB::Var; SHA256_WORD_BITS],
    ) -> [AB::Expr; SHA256_WORD_BITS] {
        array::from_fn(|i| {
            x[i] * y[i] + x[i] * z[i] + y[i] * z[i] - AB::Expr::TWO * x[i] * y[i] * z[i]
        })
    }

    /// Computes SmallSigma0(x), where x is a [SHA256_WORD_BITS] bit number in little-endian
    #[inline]
    fn small_sig0<AB: InteractionBuilder>(
        x: &[AB::Var; SHA256_WORD_BITS],
    ) -> [AB::Expr; SHA256_WORD_BITS] {
        Self::xor::<AB>(
            &Self::rotr::<AB>(x, 2),
            &Self::rotr::<AB>(x, 13),
            &Self::shr::<AB>(x, 22),
        )
    }

    /// Computes SmallSigma1(x), where x is a [SHA256_WORD_BITS] bit number in little-endian
    #[inline]
    fn small_sig1<AB: InteractionBuilder>(
        x: &[AB::Var; SHA256_WORD_BITS],
    ) -> [AB::Expr; SHA256_WORD_BITS] {
        Self::xor::<AB>(
            &Self::rotr::<AB>(x, 6),
            &Self::rotr::<AB>(x, 11),
            &Self::shr::<AB>(x, 25),
        )
    }

    /// Computes BigSigma0(x), where x is a [SHA256_WORD_BITS] bit number in little-endian
    #[inline]
    fn big_sig0<AB: InteractionBuilder>(
        x: &[AB::Var; SHA256_WORD_BITS],
    ) -> [AB::Expr; SHA256_WORD_BITS] {
        Self::xor::<AB>(
            &Self::rotr::<AB>(x, 2),
            &Self::rotr::<AB>(x, 13),
            &Self::rotr::<AB>(x, 22),
        )
    }

    /// Computes BigSigma1(x), where x is a [SHA256_WORD_BITS] bit number in little-endian
    #[inline]
    fn big_sig1<AB: InteractionBuilder>(
        x: &[AB::Var; SHA256_WORD_BITS],
    ) -> [AB::Expr; SHA256_WORD_BITS] {
        Self::xor::<AB>(
            &Self::rotr::<AB>(x, 6),
            &Self::rotr::<AB>(x, 11),
            &Self::rotr::<AB>(x, 25),
        )
    }

    /// Implement constraints that need to be true for all rows
    fn eval_row<AB: InteractionBuilder>(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);

        let local_cols: &Sha256RoundCols<AB::Var> = (*local).borrow();
        let flags = &local_cols.flags;
        builder.assert_bool(flags.is_round_row);
        builder.assert_bool(flags.is_first_4_rows);
        builder.assert_bool(flags.is_digest_row);
        builder.assert_bool(flags.is_round_row + flags.is_digest_row);
        builder.assert_bool(flags.is_last_block);
        builder.when_first_row().assert_zero(flags.global_block_idx);
        self.row_idx_encoder
            .eval(builder, &local_cols.flags.row_idx);
        builder.assert_one(contains_flag::<AB>(
            &self.row_idx_encoder,
            &local_cols.flags.row_idx,
            &(0..18).collect::<Vec<_>>(),
        ));
        builder.assert_eq(
            contains_flag::<AB>(
                &self.row_idx_encoder,
                &local_cols.flags.row_idx,
                &[0, 1, 2, 3],
            ),
            flags.is_first_4_rows,
        );
        builder.assert_eq(
            contains_flag::<AB>(
                &self.row_idx_encoder,
                &local_cols.flags.row_idx,
                &(0..16).collect::<Vec<_>>(),
            ),
            flags.is_round_row,
        );
        builder.assert_eq(
            contains_flag::<AB>(&self.row_idx_encoder, &local_cols.flags.row_idx, &[16]),
            flags.is_digest_row,
        );
        // If invalid row we want the row_idx to be 17
        builder.assert_eq(
            contains_flag::<AB>(&self.row_idx_encoder, &local_cols.flags.row_idx, &[17]),
            not::<AB::Expr>(flags.is_digest_row + flags.is_round_row),
        );

        // Constrain a, e, being composed of bits: we make sure a and e are always in the same place in the trace matrix
        // Note: this has to be true for every row, even invalid rows
        for i in 0..SHA256_ROUNDS_PER_ROW {
            for j in 0..SHA256_WORD_BITS {
                builder.assert_bool(local_cols.work_vars.a[i][j]);
                builder.assert_bool(local_cols.work_vars.e[i][j]);
            }
        }
        self.eval_round_row(builder, local_cols);
        let local_cols: &Sha256DigestCols<AB::Var> = (*local).borrow();
        self.eval_digest_row(builder, local_cols);
    }

    /// Implement constraints for a row as if it is a round row
    fn eval_round_row<AB: InteractionBuilder>(
        &self,
        builder: &mut AB,
        local: &Sha256RoundCols<AB::Var>,
    ) {
        for i in 0..SHA256_ROUNDS_PER_ROW {
            // Constrain w being composed of bits
            for j in 0..SHA256_WORD_BITS {
                builder
                    .when(local.flags.is_round_row)
                    .assert_bool(local.message_schedule.w[i][j]);
            }
            for j in 0..SHA256_WORD_U16S {
                // Although we need carry_a <= 6 and carry_e <= 5, constraining carry_a, carry_e in [0, 2^8) is enough
                // to prevent overflow and ensure the soundness of the addition we want to check
                self.bitwise_lookup_bus
                    .send_range(local.work_vars.carry_a[i][j], local.work_vars.carry_e[i][j])
                    .eval(builder, local.flags.is_round_row);

                // When on rows 4..16 carries shouuld be 0 or 1
                let is_row_4_15 = local.flags.is_round_row - local.flags.is_first_4_rows;
                builder
                    .when(is_row_4_15.clone())
                    .assert_bool(local.message_schedule.carry_or_buffer[i][j * 2]);
                builder
                    .when(is_row_4_15)
                    .assert_bool(local.message_schedule.carry_or_buffer[i][j * 2 + 1]);
            }
        }
    }

    /// Implement constraints for a digest row
    fn eval_digest_row<AB: InteractionBuilder>(
        &self,
        builder: &mut AB,
        local: &Sha256DigestCols<AB::Var>,
    ) {
        // Check that if this is the last row of a message or an invalid row, the hash should be the [SHA256_H]
        for i in 0..SHA256_ROUNDS_PER_ROW {
            let a = local.hash.a[i].map(|x| x.into());
            let e = local.hash.e[i].map(|x| x.into());
            for j in 0..SHA256_WORD_U16S {
                let a_limb = compose::<AB>(&a[j * 16..(j + 1) * 16], 1);
                let e_limb = compose::<AB>(&e[j * 16..(j + 1) * 16], 1);

                // If it is a padding row or the last row of a message, the `hash` should be the [SHA256_H]
                builder
                    .when(
                        not::<AB::Expr>(local.flags.is_round_row + local.flags.is_digest_row)
                            + local.flags.is_last_block * local.flags.is_digest_row,
                    )
                    .assert_eq(
                        a_limb,
                        AB::Expr::from_canonical_u32(
                            u32_into_limbs::<2>(SHA256_H[SHA256_ROUNDS_PER_ROW - i - 1])[j],
                        ),
                    );

                builder
                    .when(
                        not::<AB::Expr>(local.flags.is_round_row + local.flags.is_digest_row)
                            + local.flags.is_last_block * local.flags.is_digest_row,
                    )
                    .assert_eq(
                        e_limb,
                        AB::Expr::from_canonical_u32(
                            u32_into_limbs::<2>(SHA256_H[SHA256_ROUNDS_PER_ROW - i + 3])[j],
                        ),
                    );
            }
        }

        // Check if last row of a non-last block, the `hash` should be equal to the final hash of the current block
        for i in 0..SHA256_ROUNDS_PER_ROW {
            let prev_a = local.hash.a[i].map(|x| x.into());
            let prev_e = local.hash.e[i].map(|x| x.into());
            let cur_a = local.final_hash[SHA256_ROUNDS_PER_ROW - i - 1].map(|x| x.into());
            let cur_e = local.final_hash[SHA256_ROUNDS_PER_ROW - i + 3].map(|x| x.into());
            for j in 0..SHA256_WORD_U8S {
                let prev_a_limb = compose::<AB>(&prev_a[j * 8..(j + 1) * 8], 1);
                let prev_e_limb = compose::<AB>(&prev_e[j * 8..(j + 1) * 8], 1);

                builder
                    .when(not(local.flags.is_last_block) * local.flags.is_digest_row)
                    .assert_eq(prev_a_limb, cur_a[j].clone());

                builder
                    .when(not(local.flags.is_last_block) * local.flags.is_digest_row)
                    .assert_eq(prev_e_limb, cur_e[j].clone());
            }
        }

        // Need u16 limbs instead of u8 limbs before pushing
        let composed_final_hash: [[<AB as AirBuilder>::Expr; SHA256_WORD_U16S]; SHA256_HASH_WORDS] =
            local.final_hash.map(|x| {
                let x = x.map(|f| f.into());
                array::from_fn(|i| compose::<AB>(&x[i * 2..(i + 1) * 2], 8))
            });

        let next_local_block_id = select(
            local.flags.is_last_block,
            AB::Expr::ZERO,
            local.local_block_idx + AB::Expr::ONE,
        );
        // The following interactions constrain certain values from block to block
        builder.push_send(
            self.bus_idx,
            composed_final_hash
                .into_iter()
                .flatten()
                .chain(once(next_local_block_id))
                .chain(once(AB::Expr::ONE + local.flags.global_block_idx)),
            local.flags.is_digest_row,
        );

        builder.push_receive(
            self.bus_idx,
            local
                .prev_hash
                .into_iter()
                .flatten()
                .map(|x| x.into())
                .chain(once(local.local_block_idx.into()))
                .chain(once(local.flags.global_block_idx.into())),
            local.flags.is_digest_row,
        );
    }

    fn eval_transitions<AB: InteractionBuilder>(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let next = main.row_slice(1);

        // Doesn't matter what column structs we use here
        let local_cols: &Sha256RoundCols<AB::Var> = (*local).borrow();
        let next_cols: &Sha256RoundCols<AB::Var> = (*next).borrow();

        let local_is_padding_row =
            not::<AB::Expr>(local_cols.flags.is_round_row + local_cols.flags.is_digest_row);
        let next_is_padding_row =
            not::<AB::Expr>(next_cols.flags.is_round_row + next_cols.flags.is_digest_row);
        // Checking the very last block has `is_last_block` -> at least one block is a `is_last_block`
        // the rest of the constraining of `is_last_block` should be done by the wrapper chip
        builder
            .when(next_is_padding_row.clone())
            .when(local_cols.flags.is_digest_row)
            .assert_one(local_cols.flags.is_last_block);
        builder
            .when_last_row()
            .when(local_cols.flags.is_digest_row)
            .assert_one(local_cols.flags.is_last_block);
        // If we are in a round row, the next row cannot be a padding row
        builder
            .when(local_cols.flags.is_round_row)
            .assert_zero(next_is_padding_row.clone());
        // The first row must be a round row
        builder
            .when_first_row()
            .assert_one(local_cols.flags.is_round_row);
        // If we are in a padding row, the next row must also be a padding row
        builder
            .when_transition()
            .when(local_is_padding_row.clone())
            .assert_one(next_is_padding_row.clone());
        // If we are in a digest row, the next row cannot be a digest row
        builder
            .when(local_cols.flags.is_digest_row)
            .assert_zero(next_cols.flags.is_digest_row);
        // Constrin how much the row index changes by
        // round->round: 1
        // round->digest: 1
        // digest->round: -16
        // digest->padding: 0
        // padding->padding: 0
        // Other transitions are not allowed by the above
        let delta = local_cols.flags.is_round_row
            * (next_cols.flags.is_digest_row + next_cols.flags.is_round_row)
            * AB::Expr::ONE
            + local_cols.flags.is_digest_row
                * next_cols.flags.is_round_row
                * AB::Expr::from_canonical_u32(16)
                * AB::Expr::NEG_ONE
            + local_cols.flags.is_digest_row * next_is_padding_row * AB::Expr::ONE
            + local_is_padding_row * AB::Expr::ZERO;

        let local_row_idx = flag_with_val::<AB>(
            &self.row_idx_encoder,
            &local_cols.flags.row_idx,
            &(0..18).map(|i| (i, i)).collect::<Vec<_>>(),
        );
        let next_row_idx = flag_with_val::<AB>(
            &self.row_idx_encoder,
            &next_cols.flags.row_idx,
            &(0..18).map(|i| (i, i)).collect::<Vec<_>>(),
        );

        builder
            .when_transition()
            .assert_eq(local_row_idx.clone() + delta, next_row_idx.clone());
        builder.when_first_row().assert_zero(local_row_idx);

        builder
            .when_first_row()
            .assert_zero(local_cols.flags.global_block_idx);
        builder
            .when_transition()
            .when(next_cols.flags.is_digest_row)
            .assert_eq(
                local_cols.flags.global_block_idx + AB::Expr::ONE,
                next_cols.flags.global_block_idx,
            );

        self.eval_message_schedule::<AB>(builder, local_cols, next_cols);
        self.eval_work_vars::<AB>(builder, local_cols, next_cols);
    }

    /// Constrain the message schedule additions
    /// Note: For every addition we need to constrain the following for each of [SHA256_WORD_U16S] limbs
    /// sig_1(w_{t-2})[i] + w_{t-7}[i] + sig_0(w_{t-15})[i] + w_{t-16}[i] + carry_w[t][i-1] - carry_w[t][i] * 2^16 - w_t[i] == 0
    fn eval_message_schedule<AB: InteractionBuilder>(
        &self,
        builder: &mut AB,
        local: &Sha256RoundCols<AB::Var>,
        next: &Sha256RoundCols<AB::Var>,
    ) {
        let w = [local.message_schedule.w, next.message_schedule.w].concat();

        for i in 0..SHA256_ROUNDS_PER_ROW {
            let cur = w[i + 4].map(|x| x.into());
            let cur_sig = Self::small_sig0::<AB>(&w[i + 4]);
            let cur_1 = w[i + 3].map(|x| x.into());
            let cur_2 = w[i + 2];
            let cur_2_sig = Self::small_sig1::<AB>(&cur_2);
            for j in 0..SHA256_WORD_U16S {
                // the current round index or 0 if it is not a valid round to do a message schedule addition
                let round_idx = flag_with_val::<AB>(
                    &self.row_idx_encoder,
                    &next.flags.row_idx,
                    &(4..16)
                        .map(|rw_idx| (rw_idx, rw_idx * SHA256_ROUNDS_PER_ROW + i))
                        .collect::<Vec<_>>(),
                );
                // 1 if the round index is in [16, 63] and 0 otherwise (ie is it a valid round to send an interaction)
                let is_in_16_63 = contains_flag::<AB>(
                    &self.row_idx_encoder,
                    &next.flags.row_idx,
                    &(4..16).collect::<Vec<_>>(),
                );
                // the round index + 7 or 0 if it is not a valid round for w_{t-7}
                let round_idx_7 = flag_with_val::<AB>(
                    &self.row_idx_encoder,
                    &next.flags.row_idx,
                    &if i == 0 {
                        (3..15)
                            .map(|rw_idx| (rw_idx, rw_idx * SHA256_ROUNDS_PER_ROW + i + 7))
                            .collect::<Vec<_>>()
                    } else {
                        (2..14)
                            .map(|rw_idx| (rw_idx, rw_idx * SHA256_ROUNDS_PER_ROW + i + 7))
                            .collect::<Vec<_>>()
                    },
                );
                // 1 if the round index is in [9, 56] and 0 otherwise
                let is_in_9_56 = contains_flag::<AB>(
                    &self.row_idx_encoder,
                    &next.flags.row_idx,
                    &if i == 0 {
                        (3..15).collect::<Vec<_>>()
                    } else {
                        (2..14).collect::<Vec<_>>()
                    },
                );
                // the round index + 15 or 0 if it is not a valid round for sig_0(w_{t-15}) + w_{t-16}
                let round_idx_15 = flag_with_val::<AB>(
                    &self.row_idx_encoder,
                    &next.flags.row_idx,
                    &if i == 0 {
                        (1..13)
                            .map(|rw_idx| (rw_idx, rw_idx * SHA256_ROUNDS_PER_ROW + i + 15))
                            .collect::<Vec<_>>()
                    } else {
                        (0..12)
                            .map(|rw_idx| (rw_idx, rw_idx * SHA256_ROUNDS_PER_ROW + i + 7))
                            .collect::<Vec<_>>()
                    },
                );
                // 1 if the round index is in [1, 48] and 0 otherwise
                let is_in_1_48 = contains_flag::<AB>(
                    &self.row_idx_encoder,
                    &next.flags.row_idx,
                    &if i == 0 {
                        (1..13).collect::<Vec<_>>()
                    } else {
                        (0..12).collect::<Vec<_>>()
                    },
                );

                let cur_limb = compose::<AB>(&cur[j * 16..(j + 1) * 16], 1);
                let cur_sig_limb = compose::<AB>(&cur_sig[j * 16..(j + 1) * 16], 1);
                let cur_1_limb = compose::<AB>(&cur_1[j * 16..(j + 1) * 16], 1);
                let cur_2_sig_limb = compose::<AB>(&cur_2_sig[j * 16..(j + 1) * 16], 1);
                let carry = next.message_schedule.carry_or_buffer[i][j * 2]
                    + AB::Expr::TWO * next.message_schedule.carry_or_buffer[i][j * 2 + 1];
                let count = cur_2_sig_limb
                    - carry * AB::Expr::from_canonical_u32(1 << 16)
                    - cur_limb.clone();
                let count = if j == 0 {
                    count
                } else {
                    count
                        + next.message_schedule.carry_or_buffer[i][2 * j - 2]
                        + AB::Expr::TWO * next.message_schedule.carry_or_buffer[i][j * 2 - 1]
                };
                // Pushing the sig_1(w_{t-2})[i] + carry_w[t][i-1] - carry_w[t][i] * 2^16 - w_t[i] part
                builder.push_send(
                    self.bus_idx,
                    [
                        local.flags.global_block_idx.into(),
                        round_idx.clone(),
                        is_in_16_63 * AB::Expr::from_canonical_usize(j),
                    ],
                    count,
                );
                // Pushing the w_{t-7}[i] part
                builder.push_send(
                    self.bus_idx,
                    [
                        local.flags.global_block_idx.into(),
                        round_idx_7.clone(),
                        is_in_9_56 * AB::Expr::from_canonical_usize(j),
                    ],
                    cur_limb.clone(),
                );
                // Pushing the sig_0(w_{t-15})[i] + w_{t-16}[i] part
                builder.push_send(
                    self.bus_idx,
                    [
                        local.flags.global_block_idx.into(),
                        round_idx_15.clone(),
                        is_in_1_48 * AB::Expr::from_canonical_usize(j),
                    ],
                    cur_1_limb + cur_sig_limb,
                );
            }
        }
        builder.push_send(
            self.bus_idx,
            [
                local.flags.global_block_idx.into(),
                AB::Expr::ZERO,
                AB::Expr::ZERO,
            ],
            local.message_schedule.count_correction,
        );
    }

    /// Constrain the work vars according to the sha256 documentation
    fn eval_work_vars<AB: InteractionBuilder>(
        &self,
        builder: &mut AB,
        local: &Sha256RoundCols<AB::Var>,
        next: &Sha256RoundCols<AB::Var>,
    ) {
        let a = [local.work_vars.a, next.work_vars.a].concat();
        let e = [local.work_vars.e, next.work_vars.e].concat();
        for i in 0..SHA256_ROUNDS_PER_ROW {
            let cur_a = a[i + 4].map(|x| x.into());
            let sig_a = Self::big_sig0::<AB>(&a[i + 3]);
            let maj_abc = Self::maj::<AB>(&a[i + 3], &a[i + 2], &a[i + 1]);
            let d = a[i].map(|x| x.into());
            let cur_e = e[i + 4].map(|x| x.into());
            let h = e[i].map(|x| x.into());
            let sig_e = Self::big_sig1::<AB>(&e[i + 3]);
            let ch_efg = Self::ch::<AB>(&e[i + 3], &e[i + 2], &e[i + 1]);
            let w = next.message_schedule.w[i].map(|x| x.into());

            for j in 0..SHA256_WORD_U16S {
                let cur_a_limb = compose::<AB>(&cur_a[j * 16..(j + 1) * 16], 1);
                let sig_a_limb = compose::<AB>(&sig_a[j * 16..(j + 1) * 16], 1);
                let maj_abc_limb = compose::<AB>(&maj_abc[j * 16..(j + 1) * 16], 1);
                let d_limb = compose::<AB>(&d[j * 16..(j + 1) * 16], 1);
                let cur_e_limb = compose::<AB>(&cur_e[j * 16..(j + 1) * 16], 1);
                let h_limb = compose::<AB>(&h[j * 16..(j + 1) * 16], 1);
                let sig_e_limb = compose::<AB>(&sig_e[j * 16..(j + 1) * 16], 1);
                let ch_efg_limb = compose::<AB>(&ch_efg[j * 16..(j + 1) * 16], 1);
                let k_limb = flag_with_val::<AB>(
                    &self.row_idx_encoder,
                    &next.flags.row_idx,
                    &(4..16)
                        .map(|rw_idx| {
                            (
                                rw_idx,
                                u32_into_limbs::<2>(SHA256_K[rw_idx * SHA256_ROUNDS_PER_ROW + i])[j]
                                    as usize,
                            )
                        })
                        .collect::<Vec<_>>(),
                );
                let w_limb = compose::<AB>(&w[j * 16..(j + 1) * 16], 1);

                // Constrain `e`
                builder.assert_zero(
                    d_limb
                        + h_limb.clone()
                        + sig_e_limb.clone()
                        + ch_efg_limb.clone()
                        + k_limb.clone()
                        + w_limb.clone()
                        - cur_e_limb.clone()
                        - next.work_vars.carry_e[i][j] * AB::Expr::from_canonical_u32(1 << 16)
                        + if j == 0 {
                            AB::Expr::ZERO
                        } else {
                            next.work_vars.carry_e[i][j - 1].into()
                        },
                );

                // Constrain `a`
                builder.assert_zero(
                    h_limb + sig_e_limb + ch_efg_limb + k_limb + w_limb + sig_a_limb + maj_abc_limb
                        - cur_a_limb
                        - next.work_vars.carry_a[i][j] * AB::Expr::from_canonical_u32(1 << 16)
                        + if j == 0 {
                            AB::Expr::ZERO
                        } else {
                            next.work_vars.carry_a[i][j - 1].into()
                        },
                );
            }
        }
    }
}
