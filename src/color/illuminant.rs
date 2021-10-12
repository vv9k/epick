#![allow(dead_code)]

use crate::color::Xyz;

#[derive(Debug)]
pub enum Illuminant {
    A,
    B,
    C,
    D50,
    D55,
    D65,
    D75,
    E,
    F2,
    F7,
    F11,
}

impl Illuminant {
    #[rustfmt::skip]
    pub fn xyz(&self) -> Xyz {
        match self {
            Illuminant::A   => Xyz::new(1.0985 , 1., 0.35585),
            Illuminant::B   => Xyz::new(0.99072, 1., 0.85223),
            Illuminant::C   => Xyz::new(0.98074, 1., 1.18232),
            Illuminant::D50 => Xyz::new(0.96422, 1., 0.82521),
            Illuminant::D55 => Xyz::new(0.95682, 1., 0.92149),
            Illuminant::D65 => Xyz::new(0.95047, 1., 1.08883),
            Illuminant::D75 => Xyz::new(0.94972, 1., 1.22638),
            Illuminant::E   => Xyz::new(1.     , 1., 1.     ),
            Illuminant::F2  => Xyz::new(0.99186, 1., 0.67393),
            Illuminant::F7  => Xyz::new(0.95041, 1., 1.08747),
            Illuminant::F11 => Xyz::new(1.00962, 1., 0.64350),
        }
    }
    pub fn reference_u(&self) -> f32 {
        self.xyz().u()
    }
    pub fn reference_v(&self) -> f32 {
        self.xyz().v()
    }
}
