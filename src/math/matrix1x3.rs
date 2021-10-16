use std::ops::Index;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix1x3(pub [f32; 3]);

impl From<[f32; 3]> for Matrix1x3 {
    fn from(arr: [f32; 3]) -> Self {
        Self(arr)
    }
}

impl Index<usize> for Matrix1x3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
