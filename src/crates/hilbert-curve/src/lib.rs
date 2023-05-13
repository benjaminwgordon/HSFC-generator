use brgc::Brgc;
use skilling_transform::skilling_transform;
use std::error::Error;
/*
 * The main feature of the library
 *
 * The Hilbert Curve constructor takes two parameters:
 *      n: the number of dimensions
 *          (can be either 2d for planar curves, or 3d for cubes)
 *      p: the number of binary bits used to represent the position of each vertex
 *          (your resulting curve/cube will have 2^n * 2^n vertices in each face)
 *
 * The resulting Vector of (u32,u32,u32) represents the ordered traversal of each
 * (x,y,z) vertex in a Hilbert space filling curve defined by n and p.
 */
#[derive(Debug)]
pub struct HilbertCurve {
    pub coordinates: Vec<(u32, u32, u32)>,
}

impl HilbertCurve {
    pub fn new(n: u32, p: u32) -> Result<Self, Box<dyn Error>> {
        if !(n == 2 || n == 3) {
            return Err("invalid value for n | allowed values for n are 2 and 3".into());
        }

        if !(p <= 8) {
            return Err("invalid value for p | allowed values for p are [0,8]".into());
        }

        let brgc = Brgc { index: 0 };
        let num_vertices = u32::pow(u32::pow(2, p), n).try_into().unwrap();
        let brgc_vec = brgc.take(num_vertices).collect();

        let hilbert_indices = skilling_transform(brgc_vec, n, p);

        match n {
            2 => {
                let coordinates = hilbert_indices
                    .iter()
                    .map(|index| into_xyz_decimal_2d(*index, n, p))
                    .collect();
                Ok(Self { coordinates })
            }
            3 => {
                let coordinates = hilbert_indices
                    .iter()
                    .map(|index| into_xyz_decimal_3d(*index, n, p))
                    .collect();
                Ok(Self { coordinates })
            }
            _ => Err("invalid value for n | allowed values for n are 2 and 3".into()),
        }
    }
}

pub fn into_xyz_binary_2d(hilbert_coordinates: u32, n: u32, p: u32) -> (u32, u32) {
    let binary_hilbert_encoded_coordinates = format!("{:b}", hilbert_coordinates);
    // insert any necessary leading zeros so that the string is N * P characters
    // this is necessary to ensure each bit is correctly
    //      assigned to its x,y, or z axis
    let mut padding = String::new();
    while binary_hilbert_encoded_coordinates.len() + padding.len() < (n * p) as usize {
        padding.push_str("0");
    }
    padding.push_str(&binary_hilbert_encoded_coordinates);
    let padded_hilbert_coordinate_string = padding;
    // take every nth character and assign it to its axis
    // two axis decoding, only x and y coordinates
    let x = padded_hilbert_coordinate_string
        .chars()
        .step_by(2)
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    let y = padded_hilbert_coordinate_string
        .chars()
        .skip(1)
        .step_by(2)
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    (x, y)
}

pub fn into_xyz_binary_3d(hilbert_coordinates: u32, n: u32, p: u32) -> (u32, u32, u32) {
    let binary_hilbert_encoded_coordinates = format!("{:b}", hilbert_coordinates);
    // insert any necessary leading zeros so that the string is N * P characters
    // this is necessary to ensure each bit is correctly
    // assigned to its x,y, or z axis
    let mut padding = String::new();
    while binary_hilbert_encoded_coordinates.len() + padding.len() < (n * p) as usize {
        padding.push_str("0");
    }
    padding.push_str(&binary_hilbert_encoded_coordinates);
    let padded_hilbert_coordinate_string = padding;

    // take every nth character and assign it to its axis
    // three axis decoding, x,y,z coordinates
    let x = padded_hilbert_coordinate_string
        .chars()
        .step_by(3)
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    let y = padded_hilbert_coordinate_string
        .chars()
        .skip(1)
        .step_by(3)
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    let z = padded_hilbert_coordinate_string
        .chars()
        .skip(2)
        .step_by(3)
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    (x, y, z)
}

pub fn into_xyz_decimal_2d(hilbert_coordinates: u32, n: u32, p: u32) -> (u32, u32, u32) {
    let (x_bin, y_bin) = into_xyz_binary_2d(hilbert_coordinates, n, p);
    let x_dec = u32::from_str_radix(&x_bin.to_string(), 2).unwrap();
    let y_dec = u32::from_str_radix(&y_bin.to_string(), 2).unwrap();
    (x_dec, y_dec, 0)
}

pub fn into_xyz_decimal_3d(hilbert_coordinates: u32, n: u32, p: u32) -> (u32, u32, u32) {
    let (x_bin, y_bin, z_bin) = into_xyz_binary_3d(hilbert_coordinates, n, p);
    let x_dec = u32::from_str_radix(&x_bin.to_string(), 2).unwrap();
    let y_dec = u32::from_str_radix(&y_bin.to_string(), 2).unwrap();
    let z_dec = u32::from_str_radix(&z_bin.to_string(), 2).unwrap();
    (x_dec, y_dec, z_dec)
}
