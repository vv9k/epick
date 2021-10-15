use std::ops::Neg;

#[derive(Debug)]
pub struct Matrix3(pub [[f32; 3]; 3]);

impl From<[[f32; 3]; 3]> for Matrix3 {
    fn from(arr: [[f32; 3]; 3]) -> Self {
        Self(arr)
    }
}

impl Matrix3 {
    pub fn determinant(&self) -> f32 {
        let arr = &self.0;
        arr[0][0] * (arr[1][1] * arr[2][2] - arr[1][2] * arr[2][1])
            - arr[0][1] * (arr[1][0] * arr[2][2] - arr[1][2] * arr[2][0])
            + arr[0][2] * (arr[1][0] * arr[2][1] - arr[1][1] * arr[2][0])
    }

    pub fn mul_by(&mut self, n: f32) {
        self.0[0][0] *= n;
        self.0[0][1] *= n;
        self.0[0][2] *= n;
        self.0[1][0] *= n;
        self.0[1][1] *= n;
        self.0[1][2] *= n;
        self.0[2][0] *= n;
        self.0[2][1] *= n;
        self.0[2][2] *= n;
    }

    pub fn inverse(&self) -> Matrix3 {
        let arr = &self.0;
        let mut n = Matrix3::from([
            [
                arr[1][1] * arr[2][2] - arr[1][2] * arr[2][1],
                (arr[0][1] * arr[2][2] - arr[0][2] * arr[2][1]).neg(),
                arr[0][1] * arr[1][2] - arr[0][2] * arr[1][1],
            ],
            [
                (arr[1][0] * arr[2][2] - arr[1][2] * arr[2][0]).neg(),
                arr[0][0] * arr[2][2] - arr[0][2] * arr[2][0],
                (arr[0][0] * arr[1][2] - arr[0][2] * arr[1][0]).neg(),
            ],
            [
                arr[1][0] * arr[2][1] - arr[2][0] * arr[1][1],
                (arr[0][0] * arr[2][1] - arr[0][1] * arr[2][0]).neg(),
                arr[0][0] * arr[1][1] - arr[0][1] * arr[1][0],
            ],
        ]);

        n.mul_by(1. / self.determinant());

        n
    }

    pub fn mul_by_3x1(&self, other: [f32; 3]) -> [f32; 3] {
        let arr = &self.0;
        let a = other[0] * arr[0][0] * arr[0][1] * arr[0][2];
        let b = other[0] * arr[1][0] * arr[1][1] * arr[1][2];
        let c = other[0] * arr[2][0] * arr[2][1] * arr[2][2];
        [a, b, c]
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Matrix3;

    #[test]
    fn matrix3_determinant() {
        let got = Matrix3::from([[1., 2., 3.], [4., 5., 6.], [7., 2., 9.]]).determinant();
        let want = -36.0f32;

        assert!((got - want).abs() < f32::EPSILON);
    }

    #[test]
    fn matrix_inverse() {
        let got = Matrix3::from([[1., 2., 3.], [4., 5., 6.], [7., 2., 9.]]).inverse();
        let want = Matrix3::from([
            [-11. / 12., 1. / 3., 1. / 12.],
            [-1. / 6., 1. / 3., -1. / 6.],
            [3. / 4., -1. / 3., 1. / 12.],
        ]);
        assert_eq!(got.0, want.0);
    }
}
