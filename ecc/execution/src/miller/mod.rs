use halo2curves_axiom::ff::Field;

use crate::ec_point::EcPoint;

#[allow(non_snake_case)]
pub fn miller_double_step<Fp2>(S: EcPoint<Fp2>) -> (EcPoint<Fp2>, [Fp2; 2])
where
    Fp2: Field,
{
    let one = Fp2::ONE;
    let two = one + one;
    let three = one + two;

    // Calculate 2S
    let x = S.x;
    let y = S.y;
    // λ = (3x^2) / (2y)
    let lambda = (three * x.square()) * (two * y).invert().unwrap();
    // x_2S = λ^2 - 2x
    let x_2S = lambda.square() - two * x;
    // y_2S = λ(x - x_2S) - y
    let y_2S = lambda * (x - x_2S) - y;
    let res = EcPoint { x: x_2S, y: y_2S };

    // Tangent line
    //   1 + b' (x_P / y_P) w + c' (1 / y_P) w^3
    // where
    //   l_{\Psi(S),\Psi(S)}(P) = 1 - λ (x_P / y_P) w + (λ * x_S - y_S) (1 / y_P) w^3
    // b = - λ
    // c = λ * x_S - y_S
    let b = lambda.neg();
    let c = lambda * x - y;

    (res, [b, c])
}

#[allow(non_snake_case)]
pub fn miller_double_and_add<Fp2>(
    S: EcPoint<Fp2>,
    Q: EcPoint<Fp2>,
) -> (EcPoint<Fp2>, [Fp2; 2], [Fp2; 2])
where
    Fp2: Field,
{
    let one = Fp2::ONE;
    let two = one + one;

    let x_s = S.x;
    let y_s = S.y;
    let x_q = Q.x;
    let y_q = Q.y;

    let lambda1 = (y_s - y_q) * (x_s - x_q).invert().unwrap();
    let x_s_plus_q = lambda1.square() - x_s - x_q;
    let lambda2 = lambda1.neg() - two * y_s * (x_s_plus_q - x_s).invert().unwrap();
    let x_s_plus_q_plus_s = lambda2.square() - x_s - x_s_plus_q;
    let y_s_plus_q_plus_s = lambda2 * (x_s - x_s_plus_q_plus_s) - y_s;

    let res = EcPoint {
        x: x_s_plus_q_plus_s,
        y: y_s_plus_q_plus_s,
    };

    // l_{\Psi(S),\Psi(Q)}(P) = 1 - λ_1 (x_P / y_P) w + (λ_1 * x_S - y_S) (1 / y_P) w^3
    let b_1 = -lambda1;
    let c_1 = lambda1 * x_s - y_s;

    // l_{\Psi(S+Q),\Psi(Q)}(P) = 1 - λ_2 (x_P / y_P) w + (λ_2 * x_S - y_S) (1 / y_P) w^3
    let b_2 = -lambda2;
    let c_2 = lambda2 * x_s - y_s;

    (res, [b_1, c_1], [b_2, c_2])
}
