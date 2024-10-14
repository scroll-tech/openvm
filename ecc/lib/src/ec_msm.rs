use elliptic_curve::group::cofactor::CofactorCurveAffine;
use itertools::Itertools;
use snark_verifier_sdk::snark_verifier::halo2_base::{
    halo2_proofs::halo2curves::secp256k1::{Fq, Secp256k1Affine},
    utils::{log2_ceil, ScalarField},
};

// Note that the sqrt is at most 16, so this is a very dumb implementation.
pub fn sqrt_ceil(n: usize) -> usize {
    assert!(n <= 256);
    for i in 1..16 {
        if i * i >= n {
            return i;
        }
    }
    16
}

pub fn multi_scalar_multiply(g: Vec<Secp256k1Affine>, scalars: Vec<Fq>) -> Secp256k1Affine {
    let n = g.len();
    let lambda = 256;
    let s = sqrt_ceil(lambda / n);
    let t = (lambda + s - 1) / s;
    let b = log2_ceil(t as u64) - log2_ceil(log2_ceil(t as u64) as u64);
    println!("s: {}, t: {}, b: {}", s, t, b);
    let e = scalars.iter().map(|s| scalar_to_bits(*s)).collect_vec();
    let (g, e) = multi_scalar_multiply_internal0(g, e, s, t, lambda);
    let g = multi_scalar_multiply_internal1(g, e, b);
    multi_scalar_multiply_internal2(g, s)
}

pub fn scalar_to_bits(scalar: Fq) -> Vec<bool> {
    let scalar = scalar.to_bytes_le();
    let mut bits = Vec::with_capacity(256);
    for b in scalar {
        let mut i = b;
        for _ in 0..8 {
            bits.push(i & 1 == 1);
            i /= 2;
        }
    }
    bits
}

fn multi_scalar_multiply_internal0(
    g: Vec<Secp256k1Affine>,
    e: Vec<Vec<bool>>,
    s: usize,
    t: usize,
    lambda: usize,
) -> (Vec<Secp256k1Affine>, Vec<Vec<bool>>) {
    let n = g.len();
    let mut new_g = Vec::with_capacity(n * s);
    let mut new_e = vec![Vec::with_capacity(n * s); t];
    for (k, ne) in new_e.iter_mut().enumerate() {
        for (i, g) in g.iter().enumerate() {
            if k == 0 {
                new_g.push(*g);
                for _ in 1..s {
                    let last = new_g[new_g.len() - 1];
                    let double = ec_add(last, last);
                    new_g.push(double);
                }
            }
            let fill = if k == t - 1 { lambda % s } else { s };
            for j in 0..fill {
                ne.push(e[i][k * s + j]);
            }
            for _ in fill..s {
                ne.push(false);
            }
        }
    }
    (new_g, new_e)
}

fn multi_scalar_multiply_internal1(
    g: Vec<Secp256k1Affine>,
    e: Vec<Vec<bool>>,
    b: usize,
) -> Vec<Secp256k1Affine> {
    // let m = g.len();
    let s_chunks = g.chunks(b).collect::<Vec<_>>();
    let e_chunks = e.iter().map(|e| e.chunks(b).collect_vec()).collect_vec();
    let t = s_chunks
        .iter()
        .map(|s| {
            let iters = 1 << s.len();
            let mut prods = vec![];
            let mut cur_pow = 1;
            let mut cur_idx = 0;
            for i in 1..iters {
                if i == cur_pow * 2 {
                    cur_pow *= 2;
                    cur_idx += 1;
                }
                if i == cur_pow {
                    prods.push(s[cur_idx])
                } else {
                    prods.push((s[cur_idx] + prods[i - cur_pow - 1]).into());
                }
            }
            prods
        })
        .collect_vec();
    let g = e_chunks
        .iter()
        .map(|e| {
            let id = Secp256k1Affine::identity();
            e.iter().zip(t.iter()).fold(id, |acc, (e, t)| {
                let mut idx = 0;
                let l = e.len();
                for i in 0..l {
                    if i > 0 {
                        idx *= 2;
                    }
                    if e[l - i - 1] {
                        idx += 1;
                    }
                }
                if idx == 0 {
                    acc
                } else {
                    ec_add(acc, t[idx - 1])
                }
            })
        })
        .collect_vec();
    g
}

fn multi_scalar_multiply_internal2(g: Vec<Secp256k1Affine>, s: usize) -> Secp256k1Affine {
    let l = g.len();
    let mut acc = g[l - 1];
    for i in 0..l - 1 {
        for _ in 0..s {
            acc = ec_add(acc, acc);
        }
        acc = ec_add(acc, g[l - i - 2]);
    }
    acc
}

pub fn ec_add(g: Secp256k1Affine, h: Secp256k1Affine) -> Secp256k1Affine {
    let r = g + h;
    r.into()
}
