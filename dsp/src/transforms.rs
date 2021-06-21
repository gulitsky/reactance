use super::{
    consts::FRAC_1_SQRT_3,
    types::{Abc, AlphaBeta},
};

#[inline]
pub fn clarke(abc: Abc) -> AlphaBeta {
    let mut alpha_beta: AlphaBeta = Default::default();

    alpha_beta.0 = abc.0;
    alpha_beta.1 = FRAC_1_SQRT_3 * abc.0 + 2.0 * FRAC_1_SQRT_3 * abc.1;

    alpha_beta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn c() {
        let abc = (1.25, -1.361121595, 0.1111215949);
        let alpha_beta = clarke(abc);
        assert_eq!(abc.0, alpha_beta.0);
    }
}
