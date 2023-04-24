use std::error::Error;

use crate::brgc::Brgc;
use crate::skilling_transform::{into_xyz_decimal_2d, into_xyz_decimal_3d, skilling_transform};
pub struct HilbertCurve {
    pub coordinates: Vec<(u32, u32, u32)>,
}

impl HilbertCurve {
    pub fn new(n: u32, p: u32) -> Result<Self, Box<dyn Error>> {
        if !(n == 2 || n == 3) {
            return Err("invalid value for n | allowed values for n are 2 and 3".into());
        }

        if !(p >= 0 && p <= 6) {
            return Err("invalid value for p | allowed values for p are [0,6]".into());
        }

        let brgc = Brgc { index: 0 };
        let brgc_vec = brgc
            .take(u32::pow(u32::pow(2, p), n).try_into().unwrap())
            .collect();

        let hilbert_indices = skilling_transform(brgc_vec, n, p);

        let coordinates = match n {
            2 => hilbert_indices
                .iter()
                .map(|index| into_xyz_decimal_2d(*index, n, p))
                .collect(),
            3 => hilbert_indices
                .iter()
                .map(|index| into_xyz_decimal_3d(*index, n, p))
                .collect(),
            _ => !unreachable!(),
        };
        Ok(Self { coordinates })
    }
}
