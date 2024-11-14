#![allow(non_snake_case)]

use ax_ecc_execution::axvm_ecc::{
    field::FieldExtension,
    pairing::{EvaluatedLine, FinalExp, LineMulDType, MultiMillerLoop},
    point::AffinePoint,
};
use ax_stark_sdk::ax_stark_backend::p3_field::AbstractField;
use axvm_circuit::arch::{VmConfig, VmExecutor};
use eyre::Result;
use p3_baby_bear::BabyBear;

use crate::utils::build_example_program;

type F = BabyBear;

mod bn254 {
    use ax_ecc_execution::{
        axvm_ecc::{
            curve::{
                bn254::{Fq, Fq2, G1Affine, G2Affine},
                halo2curves_axiom::Coordinates,
            },
            point::AffineCoords,
        },
        curves::bn254::Bn254,
    };

    use super::*;

    #[test]
    fn test_line_function() -> Result<()> {
        let elf = build_example_program("line_function")?;
        let executor = VmExecutor::<F>::new(
            VmConfig::rv32im()
                .add_canonical_pairing_curves()
                .add_canonical_ec_curves()
                .add_canonical_modulus(),
        );

        let a = G2Affine::generator();
        let l0 = EvaluatedLine::<Fq, Fq2> { b: a.x(), c: a.y() };
        let b = G2Affine::generator();
        let l1 = EvaluatedLine::<Fq, Fq2> { b: b.x(), c: b.y() };
        let x = Bn254::mul_013_by_013(l0, l1);
        let io = [l0, l1]
            .into_iter()
            .flat_map(|fp2| fp2.into_iter())
            .chain(x)
            .flat_map(|fp2| fp2.to_coeffs())
            .flat_map(|fp| fp.to_bytes())
            .map(AbstractField::from_canonical_u8)
            .collect::<Vec<_>>();
        executor.execute(elf, vec![io])?;
        Ok(())
    }
}

mod bls12_381 {
    use ax_ecc_execution::{
        axvm_ecc::curve::bls12381::{Fq, Fq2, G1Affine, G2Affine},
        curves::bls12_381::Bls12_381,
    };

    use super::*;

    #[test]
    fn test_bls12_381_final_exp_hint() -> Result<()> {
        let elf = build_example_program("final_exp_hint")?;
        let executor = VmExecutor::<F>::new(VmConfig::rv32im());

        let bls12_381 = Bls12_381;
        let P = G1Affine::generator();
        let Q = G2Affine::generator();
        let ps = vec![AffinePoint::new(P.x, P.y), AffinePoint::new(P.x, -P.y)];
        let qs = vec![AffinePoint::new(Q.x, Q.y), AffinePoint::new(Q.x, Q.y)];
        let f = bls12_381.multi_miller_loop(&ps, &qs);
        let (c, s) = bls12_381.final_exp_hint(f);
        let io = [f, c, s]
            .into_iter()
            .flat_map(|fp12| fp12.to_coeffs())
            .flat_map(|fp2| fp2.to_coeffs())
            .flat_map(|fp| fp.to_bytes())
            .map(AbstractField::from_canonical_u8)
            .collect::<Vec<_>>();
        executor.execute(elf, vec![io])?;
        Ok(())
    }
}
