mod matrix1x3;
mod matrix3x3;

pub use matrix1x3::Matrix1x3;
pub use matrix3x3::Matrix3;

#[inline(always)]
pub fn wrap_f32(num: f32) -> f32 {
    (num + 1.).fract()
}

#[cfg(test)]
mod tests {
    use super::wrap_f32;

    #[test]
    fn it_wraps() {
        assert_eq!(wrap_f32(0.), 0.);
        assert_eq!(wrap_f32(-1. / 8.), 7. / 8.);
        assert_eq!(wrap_f32(-1. / 2.), 1. / 2.);
        assert_eq!(wrap_f32(-1.), 0.);
        assert_eq!(wrap_f32(1. / 8.), 1. / 8.);
        assert_eq!(wrap_f32(1. / 2.), 1. / 2.);
        assert_eq!(wrap_f32(1.), 0.);
    }
}
