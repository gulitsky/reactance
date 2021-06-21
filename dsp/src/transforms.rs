use core::default::default;

use crate::{
    consts::{FRAC_1_3, FRAC_1_SQRT_3, FRAC_SQRT_3_2},
    types::{Abc, AlphaBetaGamma},
};

#[inline]
pub fn clarke((a, b, c): Abc) -> AlphaBetaGamma {
    let mut alpha_beta_gamma: AlphaBetaGamma = default();

    alpha_beta_gamma.0 = (2.0 * a - b - c) * FRAC_1_3;
    alpha_beta_gamma.1 = (b - c) * FRAC_1_SQRT_3;
    alpha_beta_gamma.2 = (a + b + c) * FRAC_1_3;

    alpha_beta_gamma
}

#[inline]
pub fn inverse_clarke((alpha, beta, gamma): AlphaBetaGamma) -> Abc {
    let mut abc: Abc = default();

    abc.0 = gamma;
    abc.1 = abc.0 - alpha * 0.5;
    abc.2 = abc.1 - beta * FRAC_SQRT_3_2;
    abc.0 += alpha;
    abc.1 += beta * FRAC_SQRT_3_2;

    abc
}

#[cfg(test)]
mod tests {
    use std::default::default;

    use float_cmp::{approx_eq, F32Margin};

    use super::*;

    #[test]
    pub fn zero() {
        let abc = default();
        let (a0, b0, c0) = abc;

        let alpha_beta_gamma = clarke(abc);
        let (alpha, beta, gamma) = alpha_beta_gamma;
        assert_eq!(alpha, 0.0);
        assert_eq!(beta, 0.0);
        assert_eq!(gamma, 0.0);

        let (a1, b1, c1) = inverse_clarke(alpha_beta_gamma);
        assert_eq!(a1, a0);
        assert_eq!(b1, b0);
        assert_eq!(c1, c0);
    }

    #[test]
    pub fn unbalanced() {
        let abc = (-1.91, 0.68, 0.04);
        let (a0, b0, c0) = abc;

        let alpha_beta_gamma = clarke(abc);
        let (alpha, beta, gamma) = alpha_beta_gamma;
        assert_eq!(alpha, -1.5133333);
        assert_eq!(beta, 0.36950415);
        assert!(approx_eq!(f32, gamma, -0.3966666, F32Margin::default()));

        let (a1, b1, c1) = inverse_clarke(alpha_beta_gamma);
        assert!(approx_eq!(f32, a1, a0, F32Margin::default()));
        assert!(approx_eq!(f32, b1, b0, F32Margin::default()));
        assert!(approx_eq!(f32, c1, c0, F32Margin::default()));
    }

    #[test]
    pub fn balanced() {
        let abc = (0.15, 1.13, -1.28);
        let (a0, b0, c0) = abc;

        let alpha_beta_gamma = clarke(abc);
        let (alpha, beta, gamma) = alpha_beta_gamma;
        assert_eq!(alpha, a0);
        assert_eq!(beta, 1.391414);
        assert_eq!(gamma, 0.0);

        let (a1, b1, c1) = inverse_clarke(alpha_beta_gamma);
        assert!(approx_eq!(f32, a1, a0, F32Margin::default()));
        assert!(approx_eq!(f32, b1, b0, F32Margin::default()));
        assert!(approx_eq!(f32, c1, c0, F32Margin::default()));
    }
}
