// This is the Skilling Transform, as proposed by John Skilling at the AIP conference
// This is an algorithm for taking a series of Binary Reflected Gray Code indices,
// and converting them into an encoding of cartesian coordinates of the vertices
// of a Hilbert Space Filling Curve
pub fn skilling_transform(brgc: Vec<u32>, n: u32, p: u32) -> Vec<u32> {
    brgc.iter()
        .map(|hilbert_index| hilbert_index_to_hilbert_coordinates(hilbert_index, n, p))
        .collect()
}

fn hilbert_index_to_hilbert_coordinates(hilbert_index: &u32, n: u32, p: u32) -> u32 {
    let mut hilbert_index_bitvec = into_bit_vec(*hilbert_index, (n * p) as usize);

    // walk over each data bit, skipping the first two
    for r in (0..(&hilbert_index_bitvec.len() - n as usize)).rev() {
        if !hilbert_index_bitvec[r as usize] {
            // swap the lower-order x bits with y-bits
            let mut i = hilbert_index_bitvec.len() - n as usize;
            let bitvec_len = hilbert_index_bitvec.len();
            match n {
                2 => {
                    // 2 dimensional swap
                    while i > r {
                        hilbert_index_bitvec.swap(i, i + (bitvec_len - r) % n as usize);
                        i -= n as usize;
                    }
                }
                3 => {
                    // 3 dimensional rotate
                    while i > r {
                        hilbert_index_bitvec.swap(i, i + 2);
                        hilbert_index_bitvec.swap(i + 1, i + 2);
                        i -= n as usize;
                    }
                }
                _ => !unreachable!(),
            }
        } else {
            // flip all lower-order bits of the corresponding dimension (x, y, or z)
            let mut i = n as usize * p as usize - n as usize;
            while i > r && i >= n as usize {
                let temp = hilbert_index_bitvec[i];
                let _ = std::mem::replace(&mut hilbert_index_bitvec[i], !temp);
                i -= n as usize;
            }
        }
    }
    into_u32(hilbert_index_bitvec)
}

fn into_bit_vec(int: u32, length: usize) -> Vec<bool> {
    let mut bitvec = Vec::<bool>::new();
    format!("{int:b}")
        .chars()
        .for_each(|bit| bitvec.push(bit == '1'));
    // prepend leading 0's to prevent out of bounds issues later
    let leading_false_count = length - bitvec.len();
    let mut leading = Vec::<bool>::new();
    for _ in 0..leading_false_count {
        leading.push(false);
    }
    leading.extend(bitvec);
    leading
}

fn into_u32(bitvec: Vec<bool>) -> u32 {
    let bitstring: String = bitvec
        .iter()
        .map(|bit| if *bit { '1' } else { '0' })
        .collect();
    u32::from_str_radix(&bitstring, 2).unwrap()
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
    let z = padded_hilbert_coordinate_string
        .chars()
        .skip(2)
        .step_by(2)
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    (x, y)
}

pub fn into_xyz_binary_3d(hilbert_coordinates: u32, n: u32, p: u32) -> (u32, u32, u32) {
    let binary_hilbert_encoded_coordinates = format!("{:b}", hilbert_coordinates);
    println!("{binary_hilbert_encoded_coordinates}");
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

pub fn into_xyz_decimal_2d(hilbert_coordinates: u32, n: u32, p: u32) -> (u32, u32) {
    let (x_bin, y_bin) = into_xyz_binary_2d(hilbert_coordinates, n, p);
    let x_dec = u32::from_str_radix(&x_bin.to_string(), 2).unwrap();
    let y_dec = u32::from_str_radix(&y_bin.to_string(), 2).unwrap();
    (x_dec, y_dec)
}

pub fn into_xyz_decimal_3d(hilbert_coordinates: u32, n: u32, p: u32) -> (u32, u32, u32) {
    let (x_bin, y_bin, z_bin) = into_xyz_binary_3d(hilbert_coordinates, n, p);
    let x_dec = u32::from_str_radix(&x_bin.to_string(), 2).unwrap();
    let y_dec = u32::from_str_radix(&y_bin.to_string(), 2).unwrap();
    let z_dec = u32::from_str_radix(&z_bin.to_string(), 2).unwrap();
    (x_dec, y_dec, z_dec)
}

#[cfg(test)]
mod tests {
    use crate::skilling_transform::{
        hilbert_index_to_hilbert_coordinates, into_xyz_binary_2d, into_xyz_binary_3d,
        into_xyz_decimal_2d, into_xyz_decimal_3d,
    };

    #[test]
    fn example_4_bit() {
        let input = 0b1101 as u32;
        let expected = 0b1101 as u32;
        assert_eq!(hilbert_index_to_hilbert_coordinates(&input, 2, 2), expected);
    }

    #[test]
    fn example_8_bit() {
        let input = 0b11001101 as u32;
        let expected = 0b11001110 as u32;
        assert_eq!(hilbert_index_to_hilbert_coordinates(&input, 2, 4), expected);
    }

    /*
     * binary numbers that have leading 0's when represented with (n*p) bits
     * are a potential failure case
     */
    #[test]
    fn example_leading_zeros() {
        let input = 0b01100000 as u32;
        let expected = 0b01000000 as u32;
        assert_eq!(hilbert_index_to_hilbert_coordinates(&input, 2, 4), expected);
    }

    // regression test for case that caused failures for unknown reasons
    #[test]
    pub fn test_unknown() {
        let input = 0b000101000 as u32;
        let expected = 0b000101110 as u32;
        assert_eq!(hilbert_index_to_hilbert_coordinates(&input, 3, 3), expected);
    }

    #[test]
    fn decode_hilbert_coordinate_to_xyz_binary_2d() {
        let n = 2;
        let p = 4;
        let hilbert_coordinate = 0b11000101 as u32;
        let expected = (1000, 1011);
        assert_eq!(into_xyz_binary_2d(hilbert_coordinate, n, p), expected);
    }

    #[test]
    fn decode_hilbert_coordinate_to_xyz_binary_3d() {
        let n = 3;
        let p = 3;
        let hilbert_coordinate = 0b101110100 as u32;
        let expected = (111, 010, 100);
        assert_eq!(into_xyz_binary_3d(hilbert_coordinate, n, p), expected);
    }

    #[test]
    fn decode_hilbert_coordinate_to_xyz_decimal_2d() {
        let n = 2;
        let p = 3;
        let hilbert_coordinate = 0b11000101 as u32;
        let expected = (8, 11);
        assert_eq!(into_xyz_decimal_2d(hilbert_coordinate, n, p), expected);
    }

    #[test]
    fn decode_hilbert_coordinate_to_xyz_decimal_3d() {
        let n = 3;
        let p = 3;
        let hilbert_coordinate = 0b101110100 as u32;
        let expected = (7, 2, 4);
        assert_eq!(into_xyz_decimal_3d(hilbert_coordinate, n, p), expected);
    }
}
