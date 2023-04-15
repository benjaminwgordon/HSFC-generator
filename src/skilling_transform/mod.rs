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
    let mut hilbert_index_bitvec = into_bit_vec(*hilbert_index, 32);

    // there will be leading 0's in the bitvec, so calculate how many valid
    // data bits exist
    let hilbert_index_bits: usize = (p * n) as usize;

    let data_start_index = hilbert_index_bitvec.len() - hilbert_index_bits;

    // walk over each meaningful data bit, skipping the first two
    for r in (data_start_index..(&hilbert_index_bitvec.len() - 2)).rev() {
        if !hilbert_index_bitvec[r as usize] {
            // swap the lower-order x bits with y-bits
            // TODO, work out how exchanging works in 3 dimensions (only support 2 dim for now)

            let mut i = hilbert_index_bitvec.len() - n as usize;
            let bitvec_len = hilbert_index_bitvec.len();
            while i > r {
                hilbert_index_bitvec.swap(i, i + (bitvec_len - r) % n as usize);
                i -= n as usize;
            }
        } else {
            // flip all lower-order bits of the corresponding dimension (x, y, or z)
            let mut i = hilbert_index_bitvec.len() - n as usize;
            while i > r {
                let temp = hilbert_index_bitvec[i];
                let _ = std::mem::replace(&mut hilbert_index_bitvec[i], !temp);
                i -= n as usize;
            }
        }
        //println!("{:b}", into_u32(hilbert_index_bitvec.clone()));
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

#[cfg(test)]
mod tests {
    use crate::skilling_transform::hilbert_index_to_hilbert_coordinates;

    #[test]
    fn example_4_bit() {
        let input = 0b1101 as u32;
        let expected = 0b1101 as u32;
        assert_eq!(hilbert_index_to_hilbert_coordinates(&input, 2), expected);
    }

    #[test]
    fn example_8_bit() {
        let input = 0b11001101 as u32;
        let expected = 0b11001110 as u32;
        assert_eq!(hilbert_index_to_hilbert_coordinates(&input, 2), expected);
    }
}
