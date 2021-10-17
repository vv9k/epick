mod matrix1x3;
mod matrix3x3;

pub use matrix1x3::Matrix1x3;
pub use matrix3x3::Matrix3;

#[inline(always)]
pub fn wrap_f32(num: f32) -> f32 {
    (num + 1.).fract()
}

#[inline(always)]
pub fn eq_f32(lhs: f32, rhs: f32) -> bool {
    (lhs - rhs).abs() < f32::EPSILON
}

#[cfg(test)]
mod tests {
    use super::{eq_f32, wrap_f32};

    #[test]
    fn it_wraps() {
        assert!(eq_f32(wrap_f32(0.0f32), 0.0f32));
        assert!(eq_f32(wrap_f32(-1. / 8.), 7. / 8.));
        assert!(eq_f32(wrap_f32(-1. / 2.), 1. / 2.));
        assert!(eq_f32(wrap_f32(-1.), 0.));
        assert!(eq_f32(wrap_f32(1. / 8.), 1. / 8.));
        assert!(eq_f32(wrap_f32(1. / 2.), 1. / 2.));
        assert!(eq_f32(wrap_f32(1.), 0.));
    }

    #[test]
    fn it_eqs() {
        assert!(eq_f32(0., 0.));
        assert!(!eq_f32(0., f32::NAN));
        assert!(!eq_f32(0., 1.));
        assert!(eq_f32(1., 1.));
    }
}
