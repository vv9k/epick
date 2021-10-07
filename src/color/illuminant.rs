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
            Illuminant::A   => Xyz { x: 1.0985 , y: 1., z: 0.35585},
            Illuminant::B   => Xyz { x: 0.99072, y: 1., z: 0.85223},
            Illuminant::C   => Xyz { x: 0.98074, y: 1., z: 1.18232},
            Illuminant::D50 => Xyz { x: 0.96422, y: 1., z: 0.82521},
            Illuminant::D55 => Xyz { x: 0.95682, y: 1., z: 0.92149},
            Illuminant::D65 => Xyz { x: 0.95047, y: 1., z: 1.08883},
            Illuminant::D75 => Xyz { x: 0.94972, y: 1., z: 1.22638},
            Illuminant::E   => Xyz { x: 1.     , y: 1., z: 1.     },
            Illuminant::F2  => Xyz { x: 0.99186, y: 1., z: 0.67393},
            Illuminant::F7  => Xyz { x: 0.95041, y: 1., z: 1.08747},
            Illuminant::F11 => Xyz { x: 1.00962, y: 1., z: 0.64350},
        }
    }
}
