use std::{
    array,
    borrow::{Borrow, BorrowMut},
};

use ax_circuit_primitives::bitwise_op_lookup::BitwiseOperationLookupChip;
use p3_field::PrimeField32;

use super::{
    air::Sha256Air, columns::Sha256RoundCols, get_flag_pt_array, SHA256_BLOCK_WORDS,
    SHA256_HASH_WORDS, SHA256_ROUND_WIDTH, SHA256_WIDTH,
};
use crate::sha256::{
    big_sig0, big_sig1, ch, columns::Sha256DigestCols, limbs_into_u32, maj, small_sig0, small_sig1,
    u32_into_limbs, SHA256_H, SHA256_K, SHA256_ROUNDS_PER_ROW, SHA256_ROWS_PER_BLOCK,
    SHA256_WORD_BITS, SHA256_WORD_U16S, SHA256_WORD_U8S,
};

impl Sha256Air {
    /// This function takes the intput_massage (should be already padded), the previous hash,
    /// a flag indicating if it's the last block, the global block index, the local block index,
    /// and the buffer values that will be put in rows 0..4. Returns the trace of the block and the final hash
    pub fn generate_block_trace<F: PrimeField32>(
        &self,
        input: &[u32; SHA256_BLOCK_WORDS],
        prev_hash: &[u32; SHA256_HASH_WORDS],
        is_last_block: bool,
        global_block_idx: u32,
        local_block_idx: u32,
        buffer_vals: &[[u32; SHA256_WORD_U16S * SHA256_ROUNDS_PER_ROW * 2]; 4],
    ) -> ([[F; SHA256_WIDTH]; 17], [u32; SHA256_HASH_WORDS]) {
        let mut output = [[F::ZERO; SHA256_WIDTH]; SHA256_ROWS_PER_BLOCK];
        if local_block_idx == 0 {
            debug_assert!(*prev_hash == SHA256_H);
        }
        let bitwise_lookup_chip = BitwiseOperationLookupChip::<8>::new(self.bitwise_lookup_bus);
        let mut message_schedule: Vec<u32> = input.to_vec();
        let mut work_vars = *prev_hash;
        let mut final_hash = [0u32; SHA256_HASH_WORDS];
        for (i, row) in output.iter_mut().enumerate() {
            // doing the 64 rounds in 16 rows
            if i < 16 {
                let cols: &mut Sha256RoundCols<F> = row[0..SHA256_ROUND_WIDTH].borrow_mut();

                cols.flags.is_round_row = F::ONE;
                cols.flags.is_first_4_rows = if i < 4 { F::ONE } else { F::ZERO };
                cols.flags.is_digest_row = F::ZERO;
                cols.flags.is_last_block = F::from_bool(is_last_block);
                cols.flags.row_idx =
                    get_flag_pt_array(&self.row_idx_encoder, i).map(F::from_canonical_u32);
                cols.flags.global_block_idx = F::from_canonical_u32(global_block_idx);

                // W_idx = M_idx
                if i < SHA256_ROWS_PER_BLOCK / SHA256_ROUNDS_PER_ROW {
                    for j in 0..SHA256_ROUNDS_PER_ROW {
                        cols.message_schedule.w[j] = u32_into_limbs::<SHA256_WORD_BITS>(
                            input[i * SHA256_ROUNDS_PER_ROW + j],
                        )
                        .map(F::from_canonical_u32);
                        cols.message_schedule.carry_or_buffer[j] = array::from_fn(|k| {
                            F::from_canonical_u32(buffer_vals[i][j * SHA256_WORD_U16S * 2 + k])
                        });
                    }
                }
                // W_idx = SIG1(W_{idx-2}) + W_{idx-7} + SIG0(W_{idx-15}) + W_{idx-16}
                else {
                    for j in 0..SHA256_ROUNDS_PER_ROW {
                        let idx = i * SHA256_ROUNDS_PER_ROW + j;
                        let nums: [u32; 4] = [
                            small_sig1(message_schedule[idx - 2]),
                            message_schedule[idx - 7],
                            small_sig0(message_schedule[idx - 15]),
                            message_schedule[idx - 16],
                        ];
                        let w: u32 = nums.iter().fold(0, |acc, &num| acc.wrapping_add(num));
                        cols.message_schedule.w[j] =
                            u32_into_limbs::<SHA256_WORD_BITS>(w).map(F::from_canonical_u32);

                        // fill in the carrys
                        for k in 0..SHA256_WORD_U16S {
                            let mut sum = nums.iter().fold(0, |acc, &num| {
                                acc + u32_into_limbs::<SHA256_WORD_U16S>(num)[k]
                            });
                            if k > 0 {
                                sum += cols.message_schedule.carry_or_buffer[j][k * 2 - 2]
                                    .as_canonical_u32()
                                    + 2 * cols.message_schedule.carry_or_buffer[j][k * 2 - 1]
                                        .as_canonical_u32();
                            }
                            let carry = (sum - u32_into_limbs::<SHA256_WORD_U16S>(w)[k]) >> 16;
                            cols.message_schedule.carry_or_buffer[j][k * 2] =
                                F::from_canonical_u32(carry & 1);
                            cols.message_schedule.carry_or_buffer[j][k * 2 + 1] =
                                F::from_canonical_u32(carry >> 1);
                        }
                        // update the message schedule
                        message_schedule.push(w);
                    }
                }
                // fill in the work variables
                for j in 0..SHA256_ROUNDS_PER_ROW {
                    // t1 = h + SIG1(e) + ch(e, f, g) + K_idx + W_idx
                    let t1 = [
                        work_vars[7],
                        big_sig1(work_vars[4]),
                        ch(work_vars[4], work_vars[5], work_vars[6]),
                        SHA256_K[i * SHA256_ROUNDS_PER_ROW + j],
                        limbs_into_u32(cols.message_schedule.w[j].map(|f| f.as_canonical_u32())),
                    ];
                    let t1_sum: u32 = t1.iter().fold(0, |acc, &num| acc.wrapping_add(num));

                    // t2 = SIG0(a) + maj(a, b, c)
                    let t2 = [
                        big_sig0(work_vars[0]),
                        maj(work_vars[0], work_vars[1], work_vars[2]),
                    ];
                    let t2_sum: u32 = t2.iter().fold(0, |acc, &num| acc.wrapping_add(num));

                    // e = d + t1
                    let e = work_vars[3].wrapping_add(t1_sum);
                    // a = t1 + t2
                    let a = t1_sum.wrapping_add(t2_sum);

                    // fill in the carrys
                    for k in 0..SHA256_WORD_U16S {
                        let t1_limb = t1.iter().fold(0, |acc, &num| {
                            acc + u32_into_limbs::<SHA256_WORD_U16S>(num)[k]
                        });
                        let t2_limb = t2.iter().fold(0, |acc, &num| {
                            acc + u32_into_limbs::<SHA256_WORD_U16S>(num)[k]
                        });

                        let mut a_limb =
                            t1_limb + u32_into_limbs::<SHA256_WORD_U16S>(work_vars[3])[k];
                        let mut e_limb = t1_limb + t2_limb;
                        if k > 0 {
                            a_limb += cols.work_vars.carry_a[j][k - 1].as_canonical_u32();
                            e_limb += cols.work_vars.carry_e[j][k - 1].as_canonical_u32();
                        }
                        let carry_a = (a_limb - u32_into_limbs::<SHA256_WORD_U16S>(a)[k]) >> 16;
                        let carry_e = (e_limb - u32_into_limbs::<SHA256_WORD_U16S>(e)[k]) >> 16;
                        cols.work_vars.carry_a[j][k] = F::from_canonical_u32(carry_a);
                        cols.work_vars.carry_e[j][k] = F::from_canonical_u32(carry_e);
                        bitwise_lookup_chip.request_range(carry_a, carry_e);
                    }

                    // update working variables
                    work_vars[7] = work_vars[6];
                    work_vars[6] = work_vars[5];
                    work_vars[5] = work_vars[4];
                    work_vars[4] = e;
                    work_vars[3] = work_vars[2];
                    work_vars[2] = work_vars[1];
                    work_vars[1] = work_vars[0];
                    work_vars[0] = a;
                }
            }
            // computing the final hash
            else {
                let cols: &mut Sha256DigestCols<F> = row[..].borrow_mut();
                cols.flags.is_round_row = F::ZERO;
                cols.flags.is_first_4_rows = F::ZERO;
                cols.flags.is_digest_row = F::ONE;
                cols.flags.is_last_block = F::from_bool(is_last_block);
                // TODO: cols.flags.row_idx = 16
                cols.flags.global_block_idx = F::from_canonical_u32(global_block_idx);

                cols.local_block_idx = F::from_canonical_u32(local_block_idx);
                final_hash = array::from_fn(|i| work_vars[i].wrapping_add(prev_hash[i]));
                cols.final_hash = array::from_fn(|i| {
                    u32_into_limbs::<SHA256_WORD_U8S>(final_hash[i]).map(F::from_canonical_u32)
                });
                cols.prev_hash = prev_hash
                    .map(|f| u32_into_limbs::<SHA256_WORD_U16S>(f).map(F::from_canonical_u32));
                let hash = if is_last_block {
                    SHA256_H.map(|f| u32_into_limbs::<SHA256_WORD_BITS>(f))
                } else {
                    cols.final_hash
                        .map(|f| limbs_into_u32(f.map(|x| x.as_canonical_u32())))
                        .map(u32_into_limbs::<SHA256_WORD_BITS>)
                }
                .map(|x| x.map(F::from_canonical_u32));

                for i in 0..SHA256_ROUNDS_PER_ROW {
                    cols.hash.a[i] = hash[SHA256_ROUNDS_PER_ROW - i - 1];
                    cols.hash.e[i] = hash[SHA256_ROUNDS_PER_ROW - i + 3];
                }
            }
        }
        let (first16, last1) = output.split_at_mut(16);
        let local_cols: &Sha256RoundCols<F> = first16[15].as_slice().borrow();
        let next_cols: &mut Sha256RoundCols<F> = last1[16].as_mut_slice().borrow_mut();
        Self::generate_carry_ae(local_cols, next_cols, 16);
        (output, final_hash)
    }

    /// Puts the correct carrys in the `next_row`, the resulting carrys can be out of bound
    /// Here, row_idx is the index of the row in the block: 0..16 for the first 16 rows and
    /// some value outside of the 0..16 range for other rows (possibly invalid)
    fn generate_carry_ae<F: PrimeField32>(
        local_cols: &Sha256RoundCols<F>,
        next_cols: &mut Sha256RoundCols<F>,
        row_idx: usize,
    ) {
        let a = [local_cols.work_vars.a, next_cols.work_vars.a].concat();
        let e = [local_cols.work_vars.e, next_cols.work_vars.e].concat();
        for i in 0..SHA256_ROUNDS_PER_ROW {
            let cur_a = limbs_into_u32(a[i + 4].map(|x| x.as_canonical_u32()));
            let sig_a = big_sig0(limbs_into_u32(a[i + 3].map(|x| x.as_canonical_u32())));
            let maj_abc = maj(
                limbs_into_u32(a[i + 3].map(|x| x.as_canonical_u32())),
                limbs_into_u32(a[i + 2].map(|x| x.as_canonical_u32())),
                limbs_into_u32(a[i + 1].map(|x| x.as_canonical_u32())),
            );
            let d = limbs_into_u32(a[i].map(|x| x.as_canonical_u32()));
            let cur_e = limbs_into_u32(e[i + 4].map(|x| x.as_canonical_u32()));
            let h = limbs_into_u32(e[i].map(|x| x.as_canonical_u32()));
            let sig_e = big_sig1(limbs_into_u32(e[i + 3].map(|x| x.as_canonical_u32())));
            let ch_efg = ch(
                limbs_into_u32(e[i + 3].map(|x| x.as_canonical_u32())),
                limbs_into_u32(e[i + 2].map(|x| x.as_canonical_u32())),
                limbs_into_u32(e[i + 1].map(|x| x.as_canonical_u32())),
            );
            // TODO: update k
            let k = if (0..16).contains(&row_idx) {
                SHA256_K[row_idx * SHA256_ROUNDS_PER_ROW + i]
            } else {
                0
            };
            let w = limbs_into_u32(next_cols.message_schedule.w[i].map(|x| x.as_canonical_u32()));

            for j in 0..SHA256_WORD_U16S {
                let cur_a_limb = u32_into_limbs::<SHA256_WORD_U16S>(cur_a)[j];
                let sig_a_limb = u32_into_limbs::<SHA256_WORD_U16S>(sig_a)[j];
                let maj_abc_limb = u32_into_limbs::<SHA256_WORD_U16S>(maj_abc)[j];
                let d_limb = u32_into_limbs::<SHA256_WORD_U16S>(d)[j];
                let cur_e_limb = u32_into_limbs::<SHA256_WORD_U16S>(cur_e)[j];
                let h_limb = u32_into_limbs::<SHA256_WORD_U16S>(h)[j];
                let sig_e_limb = u32_into_limbs::<SHA256_WORD_U16S>(sig_e)[j];
                let ch_efg_limb = u32_into_limbs::<SHA256_WORD_U16S>(ch_efg)[j];
                let k_limb = u32_into_limbs::<SHA256_WORD_U16S>(k)[j];
                let w_limb = u32_into_limbs::<SHA256_WORD_U16S>(w)[j];

                let sum = d_limb + h_limb + sig_e_limb + ch_efg_limb + k_limb + w_limb - cur_e_limb
                    + if j == 0 {
                        0
                    } else {
                        next_cols.work_vars.carry_e[i][j - 1].as_canonical_u32()
                    };
                let carry_e = F::from_canonical_u32(sum) * F::from_canonical_u32(1 << 16).inverse();

                let sum =
                    h_limb + sig_e_limb + ch_efg_limb + k_limb + w_limb + sig_a_limb + maj_abc_limb
                        - cur_a_limb
                        + if j == 0 {
                            0
                        } else {
                            next_cols.work_vars.carry_a[i][j - 1].as_canonical_u32()
                        };
                let carry_a = F::from_canonical_u32(sum) * F::from_canonical_u32(1 << 16).inverse();
                next_cols.work_vars.carry_e[i][j] = carry_e;
                next_cols.work_vars.carry_a[i][j] = carry_a;
            }
        }
    }
    /// Puts the correct count correction in the `next_row`
    /// Here, row_idx is the index of the row in the block: 0..16 for the first 16 rows and
    /// some value outside of the 0..16 range for other rows (possibly invalid)
    pub fn generate_count_correction<F: PrimeField32>(
        local_cols: &Sha256RoundCols<F>,
        next_cols: &mut Sha256RoundCols<F>,
        row_idx: usize,
    ) {
        let mut count_correction = F::ZERO;
        // Doesn't matter which columns struct we use here
        let w = [local_cols.message_schedule.w, next_cols.message_schedule.w].concat();
        for i in 0..SHA256_ROUNDS_PER_ROW {
            let cur = limbs_into_u32(w[i + 4].map(|x| x.as_canonical_u32()));
            let cur_sig = small_sig1(cur);
            let cur_1 = limbs_into_u32(w[i + 3].map(|x| x.as_canonical_u32()));
            let cur_2 = limbs_into_u32(w[i + 2].map(|x| x.as_canonical_u32()));
            let cur_2_sig = small_sig1(cur_2);
            for j in 0..SHA256_WORD_U16S {
                let round_idx = if (0..16).contains(&row_idx) {
                    row_idx * SHA256_ROUNDS_PER_ROW + i
                } else {
                    // some invalid index
                    280
                };
                let cur_limb = u32_into_limbs::<SHA256_WORD_U16S>(cur)[j];
                let cur_sig_limb = u32_into_limbs::<SHA256_WORD_U16S>(cur_sig)[j];
                let cur_1_limb = u32_into_limbs::<SHA256_WORD_U16S>(cur_1)[j];
                let cur_2_sig_limb = u32_into_limbs::<SHA256_WORD_U16S>(cur_2_sig)[j];
                let carry = next_cols.message_schedule.carry_or_buffer[i][j * 2].as_canonical_u32()
                    + 2 * next_cols.message_schedule.carry_or_buffer[i][j * 2 + 1]
                        .as_canonical_u32();
                let mut count = cur_2_sig_limb - (carry << 16) - cur_limb;
                if j > 0 {
                    count += next_cols.message_schedule.carry_or_buffer[i][2 * j - 2]
                        .as_canonical_u32()
                        + 2 * next_cols.message_schedule.carry_or_buffer[i][2 * j - 1]
                            .as_canonical_u32();
                }
                if !(16..64).contains(&round_idx) {
                    count_correction += F::from_canonical_u32(count);
                }
                if !(1..48).contains(&round_idx) {
                    count_correction += F::from_canonical_u32(cur_1_limb);
                }
                if !(9..56).contains(&round_idx) {
                    count_correction +=
                        F::from_canonical_u32(cur_1_limb) + F::from_canonical_u32(cur_sig_limb);
                }
            }
        }
        // TODO: need -count_correction?
        next_cols.message_schedule.count_correction = count_correction;
    }

    /// Fills the `next_row` as a padding row
    /// Note: we still need to correctly fill in the hash values, carries and count corrections
    pub fn default_row<F: PrimeField32>(
        local_cols: &Sha256RoundCols<F>,
        next_cols: &mut Sha256RoundCols<F>,
    ) {
        next_cols.flags.is_round_row = F::ZERO;
        next_cols.flags.is_first_4_rows = F::ZERO;
        next_cols.flags.is_digest_row = F::ZERO;

        // TODO: revisit this
        next_cols.flags.is_last_block = F::ZERO;
        next_cols.flags.row_idx = [F::ZERO; 5];
        next_cols.flags.global_block_idx = F::ZERO;

        next_cols.message_schedule.w = [[F::ZERO; SHA256_WORD_BITS]; SHA256_ROUNDS_PER_ROW];
        next_cols.message_schedule.carry_or_buffer =
            [[F::ZERO; SHA256_WORD_U16S * 2]; SHA256_ROUNDS_PER_ROW];

        let hash = SHA256_H
            .map(|f| u32_into_limbs::<SHA256_WORD_BITS>(f))
            .map(|x| x.map(F::from_canonical_u32));

        for i in 0..SHA256_ROUNDS_PER_ROW {
            next_cols.work_vars.a[i] = hash[SHA256_ROUNDS_PER_ROW - i - 1];
            next_cols.work_vars.e[i] = hash[SHA256_ROUNDS_PER_ROW - i + 3];
        }
        Self::generate_carry_ae(local_cols, next_cols, 28);
    }
}
