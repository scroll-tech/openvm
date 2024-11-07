use axvm::intrinsics::{Fp2, Fp2Bn254};

pub trait MillerStepBn254 {
    fn miller_double_step(s: [Fp2Bn254; 2]) -> ([Fp2Bn254; 2], [Fp2Bn254; 2]);

    fn miller_double_and_add_step(
        s: [Fp2Bn254; 2],
        q: [Fp2Bn254; 2],
    ) -> ([Fp2Bn254; 2], [Fp2Bn254; 2], [Fp2Bn254; 2]);
}

impl MillerStepBn254 for Fp2Bn254 {
    fn miller_double_step(s: [Fp2Bn254; 2]) -> ([Fp2Bn254; 2], [Fp2Bn254; 2]) {
        #[cfg(not(target_os = "zkvm"))]
        {
            let two = &Fp2Bn254::from_u32((2, 0));
            let three = &Fp2Bn254::from_u32((3, 0));

            let x = &s[0];
            let y = &s[1];

            // λ = (3x^2) / (2y)
            let lambda = (three * x * x) / (two * y);

            // x_2S = λ^2 - 2x
            let x_2s = &lambda * &lambda - two * x;
            // y_2S = λ(x - x_2S) - y
            let y_2s = &lambda * &(x - &x_2s) - y;
            let two_s = [x_2s, y_2s];

            let b = Fp2Bn254::ZERO - &lambda;
            let c = &lambda * x - y;

            (two_s, [b, c])
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }

    fn miller_double_and_add_step(
        s: [Fp2Bn254; 2],
        q: [Fp2Bn254; 2],
    ) -> ([Fp2Bn254; 2], [Fp2Bn254; 2], [Fp2Bn254; 2]) {
        #[cfg(not(target_os = "zkvm"))]
        {
            let two = &Fp2Bn254::from_u32((2, 0));
            let x_s = &s[0];
            let y_s = &s[1];
            let x_q = &q[0];
            let y_q = &q[1];

            // λ1 = (y_s - y_q) / (x_s - x_q)
            let lambda1 = (y_s - y_q) / (x_s - x_q);
            let x_s_plus_q = &lambda1 * &lambda1 - x_s - x_q;

            // λ2 = -λ1 - 2y_s / (x_{s+q} - x_s)
            let lambda2 = (Fp2Bn254::ZERO - &lambda1) - two * y_s / (&x_s_plus_q - x_s);

            let x_s_plus_q_plus_s = &lambda2 * &lambda2 - x_s - &x_s_plus_q;
            let y_s_plus_q_plus_s = &lambda2 * &(x_s - &x_s_plus_q_plus_s) - y_s;

            let s_plus_q_plus_s = [x_s_plus_q_plus_s, y_s_plus_q_plus_s];

            // l_{\Psi(S),\Psi(Q)}(P) = (λ_1 * x_S - y_S) (1 / y_P) - λ_1 (x_P / y_P) w^2 + w^3
            let b0 = Fp2Bn254::ZERO - &lambda1;
            let c0 = &lambda1 * x_s - y_s;

            // l_{\Psi(S+Q),\Psi(S)}(P) = (λ_2 * x_S - y_S) (1 / y_P) - λ_2 (x_P / y_P) w^2 + w^3
            let b1 = Fp2Bn254::ZERO - &lambda2;
            let c1 = &lambda2 * x_s - y_s;

            (s_plus_q_plus_s, [b0, c0], [b1, c1])
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }
}
