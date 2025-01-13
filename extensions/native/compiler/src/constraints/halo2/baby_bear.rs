use std::{cmp::Ordering, sync::Arc};

use itertools::Itertools;
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use openvm_stark_backend::p3_field::{
    extension::{BinomialExtensionField, BinomiallyExtendable},
    Field, FieldAlgebra, FieldExtensionAlgebra, PrimeField32, PrimeField64,
};
use openvm_stark_sdk::p3_baby_bear::BabyBear;
use snark_verifier_sdk::snark_verifier::{
    halo2_base::{
        gates::{GateChip, GateInstructions, RangeChip, RangeInstructions},
        halo2_proofs::halo2curves::bn256::Fr,
        utils::{
            bigint_to_fe, biguint_to_fe, bit_length, decompose_fe_to_u64_limbs, fe_to_bigint,
            log2_ceil, BigPrimeField,
        },
        AssignedValue, Context, QuantumCell,
    },
    util::arithmetic::{Field as _, PrimeField as _},
};

pub(crate) const BABYBEAR_MAX_BITS: usize = 31;

#[derive(Copy, Clone, Debug)]
pub struct AssignedBabyBear {
    pub value: AssignedValue<Fr>,
    /// The value is guaranteed to be less than 2^max_bits.
    pub max_bits: usize,
}

impl AssignedBabyBear {
    pub fn to_baby_bear(&self) -> BabyBear {
        let mut b_int = fe_to_bigint(self.value.value()) % BabyBear::ORDER_U32;
        if b_int < BigInt::from(0) {
            b_int += BabyBear::ORDER_U32;
        }
        BabyBear::from_canonical_u32(b_int.try_into().unwrap())
    }
}

pub struct BabyBearChip {
    pub range: Arc<RangeChip<Fr>>,
    pub cached_limbs_r: Vec<QuantumCell<Fr>>,
    pub extra_bits: usize,
}

impl BabyBearChip {
    pub fn new(range_chip: Arc<RangeChip<Fr>>) -> Self {
        let lookup_bits = range_chip.lookup_bits();
        let mut v = 1;
        let mult = 1 << lookup_bits;
        let mut s = 0;
        let mut r = Vec::new();
        for _ in 0..range_chip.limb_bases.len() {
            r.push(QuantumCell::Constant(Fr::from(v)));
            s += v.min(BabyBear::ORDER_U64 - v);
            v = (mult * v) % BabyBear::ORDER_U64;
        }
        BabyBearChip {
            range: range_chip,
            cached_limbs_r: r,
            extra_bits: log2_ceil(s),
        }
    }

    pub fn gate(&self) -> &GateChip<Fr> {
        self.range.gate()
    }

    pub fn load_witness(&self, ctx: &mut Context<Fr>, value: BabyBear) -> AssignedBabyBear {
        let value = ctx.load_witness(Fr::from(PrimeField64::as_canonical_u64(&value)));
        self.range.range_check(ctx, value, BABYBEAR_MAX_BITS);
        AssignedBabyBear {
            value,
            max_bits: BABYBEAR_MAX_BITS,
        }
    }

    pub fn load_constant(&self, ctx: &mut Context<Fr>, value: BabyBear) -> AssignedBabyBear {
        let max_bits = bit_length(value.as_canonical_u64());
        let value = ctx.load_constant(Fr::from(PrimeField64::as_canonical_u64(&value)));
        AssignedBabyBear { value, max_bits }
    }

    pub fn full_reduce(&self, ctx: &mut Context<Fr>, a: AssignedBabyBear) -> AssignedBabyBear {
        debug_assert!(fe_to_bigint(a.value.value()).bits() as usize <= a.max_bits);
        let (_, r) = signed_div_mod(&self.range, ctx, a.value, BabyBear::ORDER_U32, a.max_bits);
        let r = AssignedBabyBear {
            value: r,
            max_bits: BABYBEAR_MAX_BITS,
        };
        debug_assert_eq!(a.to_baby_bear(), r.to_baby_bear());
        r
    }

    pub fn reduce(&self, ctx: &mut Context<Fr>, b: AssignedBabyBear) -> AssignedBabyBear {
        self.full_reduce(ctx, b)
        // if b.max_bits <= BABYBEAR_MAX_BITS {
        //     return b;
        // }
        // let range_bits = b.max_bits;
        // let a = b.value;
        // let lookup_bits = self.range.lookup_bits();
        // if range_bits == 0 {
        //     self.gate().assert_is_const(ctx, &a, &Fr::ZERO);
        //     return AssignedBabyBear {
        //         value: a,
        //         max_bits: 0,
        //     };
        // }
        // // the number of limbs
        // let num_limbs = range_bits.div_ceil(lookup_bits);
        // // println!("range check {} bits {} len", range_bits, k);
        // let rem_bits = range_bits % lookup_bits;

        // debug_assert!(self.range.limb_bases.len() >= num_limbs);

        // let (last_limb, r, new_max_bits) = if num_limbs == 1 {
        //     self.add_cell_to_lookup(ctx, a);
        //     (a, a, range_bits)
        // } else {
        //     let mut negative = false;
        //     let mut v = *a.value();
        //     if fe_to_bigint(&v) < 0.into() {
        //         negative = true;
        //         v = -v;
        //     }
        //     let sign = if negative {
        //         QuantumCell::Witness(-Fr::ONE)
        //     } else {
        //         QuantumCell::Witness(Fr::ONE)
        //     };
        //     ctx.assign_region(
        //         [
        //             sign,
        //             sign,
        //             QuantumCell::Constant(-Fr::ONE),
        //             QuantumCell::Constant(Fr::ZERO),
        //         ],
        //         [0],
        //     );
        //     let (s1, s2) = (ctx.get(-4), ctx.get(-3));
        //     ctx.constrain_equal(&s1, &s2);
        //     let sign = s1;
        //     let limbs = decompose_fe_to_u64_limbs(&v, num_limbs, lookup_bits)
        //         .into_iter()
        //         .map(|x| QuantumCell::Witness(Fr::from(x)));
        //     let row_offset = ctx.advice.len() as isize;
        //     let acc = self.gate().inner_product(
        //         ctx,
        //         limbs.clone(),
        //         self.range.limb_bases[..num_limbs].to_vec(),
        //     );
        //     // the inner product above must equal `a`

        //     let mut limbs = Vec::new();
        //     // we fetch the cells to lookup by getting the indices where `limbs` were assigned in `inner_product`. Because `limb_bases[0]` is 1, the progression of indices is 0,1,4,...,4+3*i
        //     limbs.push(ctx.get(row_offset));
        //     self.add_cell_to_lookup(ctx, ctx.get(row_offset));
        //     for i in 0..num_limbs - 1 {
        //         limbs.push(ctx.get(row_offset + 1 + 3 * i as isize));
        //         self.add_cell_to_lookup(ctx, ctx.get(row_offset + 1 + 3 * i as isize));
        //     }
        //     let acc = self.gate().mul(ctx, acc, sign);
        //     ctx.constrain_equal(&a, &acc);

        //     let last_limb = *limbs.last().unwrap();
        //     let r =
        //         self.gate()
        //             .inner_product(ctx, limbs, self.cached_limbs_r[..num_limbs].to_vec());
        //     // let (_, r) =
        //     //     self.range
        //     //         .div_mod(ctx, r, BabyBear::ORDER_U32, self.extra_bits + lookup_bits);
        //     let r = self.gate().mul(ctx, r, sign);
        //     // debug_assert!(fe_to_bigint(r.value()).bits() as usize <= self.extra_bits + lookup_bits);
        //     (last_limb, r, self.extra_bits + lookup_bits)
        // };

        // // additional constraints for the last limb if rem_bits != 0
        // match rem_bits.cmp(&1) {
        //     // we want to check x := limbs[num_limbs-1] is boolean
        //     // we constrain x*(x-1) = 0 + x * x - x == 0
        //     // | 0 | x | x | x |
        //     Ordering::Equal => {
        //         self.gate().assert_bit(ctx, last_limb);
        //     }
        //     Ordering::Greater => {
        //         let mult_val = self.gate().pow_of_two[lookup_bits - rem_bits];
        //         let check = self
        //             .gate()
        //             .mul(ctx, last_limb, QuantumCell::Constant(mult_val));
        //         self.add_cell_to_lookup(ctx, check);
        //     }
        //     _ => {}
        // }
        // let r = AssignedBabyBear {
        //     value: r,
        //     max_bits: new_max_bits,
        // };
        // debug_assert_eq!(b.to_baby_bear(), r.to_baby_bear());
        // r
    }

    pub fn add(
        &self,
        ctx: &mut Context<Fr>,
        mut a: AssignedBabyBear,
        mut b: AssignedBabyBear,
    ) -> AssignedBabyBear {
        if a.max_bits.max(b.max_bits) + 1 >= Fr::CAPACITY as usize - 1 {
            a = self.reduce(ctx, a);
            if a.max_bits.max(b.max_bits) + 1 >= Fr::CAPACITY as usize - 1 {
                b = self.reduce(ctx, b);
            }
        }
        let value = self.gate().add(ctx, a.value, b.value);
        let max_bits = a.max_bits.max(b.max_bits) + 1;
        let mut c = AssignedBabyBear { value, max_bits };
        debug_assert_eq!(c.to_baby_bear(), a.to_baby_bear() + b.to_baby_bear());
        if c.max_bits >= Fr::CAPACITY as usize - 1 {
            c = self.reduce(ctx, c);
        }
        c
    }

    pub fn neg(&self, ctx: &mut Context<Fr>, a: AssignedBabyBear) -> AssignedBabyBear {
        let value = self.gate().neg(ctx, a.value);
        let b = AssignedBabyBear {
            value,
            max_bits: a.max_bits,
        };
        debug_assert_eq!(b.to_baby_bear(), -a.to_baby_bear());
        b
    }

    pub fn sub(
        &self,
        ctx: &mut Context<Fr>,
        mut a: AssignedBabyBear,
        mut b: AssignedBabyBear,
    ) -> AssignedBabyBear {
        if a.max_bits.max(b.max_bits) + 1 >= Fr::CAPACITY as usize - 1 {
            a = self.reduce(ctx, a);
            if a.max_bits.max(b.max_bits) + 1 >= Fr::CAPACITY as usize - 1 {
                b = self.reduce(ctx, b);
            }
        }
        let value = self.gate().sub(ctx, a.value, b.value);
        let max_bits = a.max_bits.max(b.max_bits) + 1;
        let mut c = AssignedBabyBear { value, max_bits };
        debug_assert_eq!(c.to_baby_bear(), a.to_baby_bear() - b.to_baby_bear());
        if c.max_bits >= Fr::CAPACITY as usize - 1 {
            c = self.reduce(ctx, c);
        }
        c
    }

    pub fn mul(
        &self,
        ctx: &mut Context<Fr>,
        mut a: AssignedBabyBear,
        mut b: AssignedBabyBear,
    ) -> AssignedBabyBear {
        if a.max_bits < b.max_bits {
            std::mem::swap(&mut a, &mut b);
        }
        if a.max_bits + b.max_bits >= Fr::CAPACITY as usize - 1 {
            a = self.reduce(ctx, a);
            if a.max_bits + b.max_bits >= Fr::CAPACITY as usize - 1 {
                b = self.reduce(ctx, b);
            }
        }
        let value = self.gate().mul(ctx, a.value, b.value);
        let max_bits = a.max_bits + b.max_bits;

        let mut c = AssignedBabyBear { value, max_bits };
        if c.max_bits >= Fr::CAPACITY as usize - 1 {
            c = self.reduce(ctx, c);
        }
        debug_assert_eq!(c.to_baby_bear(), a.to_baby_bear() * b.to_baby_bear());
        c
    }

    pub fn mul_add(
        &self,
        ctx: &mut Context<Fr>,
        mut a: AssignedBabyBear,
        mut b: AssignedBabyBear,
        mut c: AssignedBabyBear,
    ) -> AssignedBabyBear {
        if a.max_bits < b.max_bits {
            std::mem::swap(&mut a, &mut b);
        }
        if a.max_bits + b.max_bits + 1 >= Fr::CAPACITY as usize - 1 {
            a = self.reduce(ctx, a);
            if a.max_bits + b.max_bits + 1 >= Fr::CAPACITY as usize - 1 {
                b = self.reduce(ctx, b);
            }
        }
        if c.max_bits + 1 >= Fr::CAPACITY as usize - 1 {
            c = self.reduce(ctx, c)
        }
        let value = self.gate().mul_add(ctx, a.value, b.value, c.value);
        let max_bits = c.max_bits.max(a.max_bits + b.max_bits) + 1;

        let mut d = AssignedBabyBear { value, max_bits };
        if d.max_bits >= Fr::CAPACITY as usize - 1 {
            d = self.reduce(ctx, d);
        }
        debug_assert_eq!(
            d.to_baby_bear(),
            a.to_baby_bear() * b.to_baby_bear() + c.to_baby_bear()
        );
        d
    }

    pub fn div(
        &self,
        ctx: &mut Context<Fr>,
        mut a: AssignedBabyBear,
        mut b: AssignedBabyBear,
    ) -> AssignedBabyBear {
        let b_val = b.to_baby_bear();
        let b_inv = b_val.try_inverse().unwrap();

        let mut c = self.load_witness(ctx, a.to_baby_bear() * b_inv);
        // constraint a = b * c (mod p)
        if a.max_bits >= Fr::CAPACITY as usize - 1 {
            a = self.reduce(ctx, a);
        }
        if b.max_bits + c.max_bits >= Fr::CAPACITY as usize - 1 {
            b = self.reduce(ctx, b);
        }
        if b.max_bits + c.max_bits >= Fr::CAPACITY as usize - 1 {
            c = self.reduce(ctx, c);
        }
        let diff = self.gate().sub_mul(ctx, a.value, b.value, c.value);
        let max_bits = a.max_bits.max(b.max_bits + c.max_bits) + 1;
        let (_, r) = signed_div_mod(&self.range, ctx, diff, BabyBear::ORDER_U32, max_bits);
        self.gate().assert_is_const(ctx, &r, &Fr::ZERO);
        debug_assert_eq!(c.to_baby_bear(), a.to_baby_bear() / b.to_baby_bear());
        c
    }

    pub fn select(
        &self,
        ctx: &mut Context<Fr>,
        cond: AssignedValue<Fr>,
        a: AssignedBabyBear,
        b: AssignedBabyBear,
    ) -> AssignedBabyBear {
        let value = self.gate().select(ctx, a.value, b.value, cond);
        let max_bits = a.max_bits.max(b.max_bits);
        AssignedBabyBear { value, max_bits }
    }

    pub fn assert_zero(&self, ctx: &mut Context<Fr>, a: AssignedBabyBear) {
        debug_assert_eq!(a.to_baby_bear(), BabyBear::ZERO);
        assert!(a.max_bits < Fr::CAPACITY as usize);
        let (_, r) = signed_div_mod(&self.range, ctx, a.value, BabyBear::ORDER_U32, a.max_bits);
        self.gate().assert_is_const(ctx, &r, &Fr::ZERO);
    }

    pub fn assert_equal(&self, ctx: &mut Context<Fr>, a: AssignedBabyBear, b: AssignedBabyBear) {
        debug_assert_eq!(a.to_baby_bear(), b.to_baby_bear());
        let diff = self.sub(ctx, a, b);
        self.assert_zero(ctx, diff);
    }

    fn add_cell_to_lookup(&self, ctx: &Context<Fr>, a: AssignedValue<Fr>) {
        let phase = ctx.phase();
        let manager = &self.range.lookup_manager()[phase];
        manager.add_lookup(ctx.tag(), [a]);
    }
}

/// Constrains and returns `(c, r)` such that `a = b * c + r`.
///
/// * a: [QuantumCell] value to divide
/// * b: [BigUint] value to divide by
/// * a_num_bits: number of bits needed to represent the absolute value of `a`
///
/// ## Assumptions
/// * `b != 0` and that `abs(a) < 2^a_max_bits`
/// * `a_max_bits < F::CAPACITY = F::NUM_BITS - 1`
///   * Unsafe behavior if `a_max_bits >= F::CAPACITY`
fn signed_div_mod<F>(
    range: &RangeChip<F>,
    ctx: &mut Context<F>,
    a: impl Into<QuantumCell<F>>,
    b: impl Into<BigUint>,
    a_num_bits: usize,
) -> (AssignedValue<F>, AssignedValue<F>)
where
    F: BigPrimeField,
{
    let a = a.into();
    let b = b.into();
    let a_val = fe_to_bigint(a.value());
    assert!(a_val.bits() <= a_num_bits as u64);
    let (div, rem) = a_val.div_mod_floor(&b.clone().into());
    let [div, rem] = [div, rem].map(|v| bigint_to_fe(&v));
    ctx.assign_region(
        [
            QuantumCell::Witness(rem),
            QuantumCell::Constant(biguint_to_fe(&b)),
            QuantumCell::Witness(div),
            a,
        ],
        [0],
    );
    let rem = ctx.get(-4);
    let div = ctx.get(-2);
    // Constrain that `abs(div) <= 2 ** a_num_bits / b`.
    let bound = (BigUint::from(1u32) << (a_num_bits as u32)) / &b;
    let shifted_div = range
        .gate()
        .add(ctx, div, QuantumCell::Constant(biguint_to_fe(&bound)));
    debug_assert!(*shifted_div.value() < biguint_to_fe(&(&bound * 2u32 + 1u32)));
    range.check_big_less_than_safe(ctx, shifted_div, bound * 2u32 + 1u32);
    // Constrain that remainder is less than divisor (i.e. `r < b`).
    debug_assert!(*rem.value() < biguint_to_fe(&b));
    range.check_big_less_than_safe(ctx, rem, b);
    (div, rem)
}

// irred poly is x^4 - 11
pub struct BabyBearExt4Chip {
    pub base: Arc<BabyBearChip>,
}

#[derive(Copy, Clone, Debug)]
pub struct AssignedBabyBearExt4(pub [AssignedBabyBear; 4]);
pub type BabyBearExt4 = BinomialExtensionField<BabyBear, 4>;

impl AssignedBabyBearExt4 {
    pub fn to_extension_field(&self) -> BabyBearExt4 {
        let b_val = (0..4).map(|i| self.0[i].to_baby_bear()).collect_vec();
        BabyBearExt4::from_base_slice(&b_val)
    }
}

impl BabyBearExt4Chip {
    pub fn new(base_chip: Arc<BabyBearChip>) -> Self {
        BabyBearExt4Chip { base: base_chip }
    }
    pub fn load_witness(&self, ctx: &mut Context<Fr>, value: BabyBearExt4) -> AssignedBabyBearExt4 {
        AssignedBabyBearExt4(
            value
                .as_base_slice()
                .iter()
                .map(|x| self.base.load_witness(ctx, *x))
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }
    pub fn load_constant(
        &self,
        ctx: &mut Context<Fr>,
        value: BabyBearExt4,
    ) -> AssignedBabyBearExt4 {
        AssignedBabyBearExt4(
            value
                .as_base_slice()
                .iter()
                .map(|x| self.base.load_constant(ctx, *x))
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }
    pub fn add(
        &self,
        ctx: &mut Context<Fr>,
        a: AssignedBabyBearExt4,
        b: AssignedBabyBearExt4,
    ) -> AssignedBabyBearExt4 {
        AssignedBabyBearExt4(
            a.0.iter()
                .zip(b.0.iter())
                .map(|(a, b)| self.base.add(ctx, *a, *b))
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }

    pub fn neg(&self, ctx: &mut Context<Fr>, a: AssignedBabyBearExt4) -> AssignedBabyBearExt4 {
        AssignedBabyBearExt4(
            a.0.iter()
                .map(|x| self.base.neg(ctx, *x))
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }

    pub fn sub(
        &self,
        ctx: &mut Context<Fr>,
        a: AssignedBabyBearExt4,
        b: AssignedBabyBearExt4,
    ) -> AssignedBabyBearExt4 {
        AssignedBabyBearExt4(
            a.0.iter()
                .zip(b.0.iter())
                .map(|(a, b)| self.base.sub(ctx, *a, *b))
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }

    pub fn scalar_mul(
        &self,
        ctx: &mut Context<Fr>,
        a: AssignedBabyBearExt4,
        b: AssignedBabyBear,
    ) -> AssignedBabyBearExt4 {
        AssignedBabyBearExt4(
            a.0.iter()
                .map(|x| self.base.mul(ctx, *x, b))
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }

    pub fn select(
        &self,
        ctx: &mut Context<Fr>,
        cond: AssignedValue<Fr>,
        a: AssignedBabyBearExt4,
        b: AssignedBabyBearExt4,
    ) -> AssignedBabyBearExt4 {
        AssignedBabyBearExt4(
            a.0.iter()
                .zip(b.0.iter())
                .map(|(a, b)| self.base.select(ctx, cond, *a, *b))
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }

    pub fn assert_zero(&self, ctx: &mut Context<Fr>, a: AssignedBabyBearExt4) {
        for x in a.0.iter() {
            self.base.assert_zero(ctx, *x);
        }
    }

    pub fn assert_equal(
        &self,
        ctx: &mut Context<Fr>,
        a: AssignedBabyBearExt4,
        b: AssignedBabyBearExt4,
    ) {
        for (a, b) in a.0.iter().zip(b.0.iter()) {
            self.base.assert_equal(ctx, *a, *b);
        }
    }

    pub fn mul(
        &self,
        ctx: &mut Context<Fr>,
        a: AssignedBabyBearExt4,
        b: AssignedBabyBearExt4,
    ) -> AssignedBabyBearExt4 {
        let mut coeffs = Vec::with_capacity(7);
        for i in 0..4 {
            for j in 0..4 {
                if i + j < coeffs.len() {
                    coeffs[i + j] = self.base.mul_add(ctx, a.0[i], b.0[j], coeffs[i + j]);
                } else {
                    coeffs.push(self.base.mul(ctx, a.0[i], b.0[j]));
                }
            }
        }
        let w = self
            .base
            .load_constant(ctx, <BabyBear as BinomiallyExtendable<4>>::W);
        for i in 4..7 {
            coeffs[i - 4] = self.base.mul_add(ctx, coeffs[i], w, coeffs[i - 4]);
        }
        coeffs.truncate(4);
        let c = AssignedBabyBearExt4(coeffs.try_into().unwrap());
        debug_assert_eq!(
            c.to_extension_field(),
            a.to_extension_field() * b.to_extension_field()
        );
        c
    }

    pub fn div(
        &self,
        ctx: &mut Context<Fr>,
        a: AssignedBabyBearExt4,
        b: AssignedBabyBearExt4,
    ) -> AssignedBabyBearExt4 {
        let b_val = b.to_extension_field();
        let b_inv = b_val.try_inverse().unwrap();

        let c = self.load_witness(ctx, a.to_extension_field() * b_inv);
        // constraint a = b * c
        let prod = self.mul(ctx, b, c);
        self.assert_equal(ctx, a, prod);

        debug_assert_eq!(
            c.to_extension_field(),
            a.to_extension_field() / b.to_extension_field()
        );
        c
    }
}
