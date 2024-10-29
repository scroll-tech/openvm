use super::Fp2;

/// Mulitplies two 013 line representations (where a is 1) by each other resulting in a 01234 Fp12 element representation
pub fn mul_013_by_013(
    b0: &mut Fp2,
    c0: &mut Fp2,
    b1: &mut Fp2,
    c1: &mut Fp2,
    xi: [isize; 2],
) -> [Fp2; 5] {
    // where w⁶ = xi
    // l0 * l1 = 1 + (b0 + b1)w + (b0b1)w² + (c0 + c1)w³ + (b0c1 + b1c0)w⁴ + (c0c1)w⁶
    //         = (1 + c0c1 * xi) + (b0 + b1)w + (b0b1)w² + (c0 + c1)w³ + (b0c1 + b1c0)w⁴
    let l0 = c0.mul(c1).int_mul(xi).int_add([1, 0]);
    let l1 = b0.add(b1);
    let l2 = b0.mul(b1);
    let l3 = c0.add(c1);
    let l4 = b0.mul(c1).add(&mut b1.mul(c0));

    [l0, l1, l2, l3, l4]
}
