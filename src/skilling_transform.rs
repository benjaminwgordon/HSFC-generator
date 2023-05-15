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
            // swap the lower-order x bits with x,y, or z bits depending on current r
            let offset = r % n as usize;
            let mut s = (p * n - n) as usize + offset;
            let mut q = (p * n - n) as usize;
            while q > r {
                hilbert_index_bitvec.swap(q, s);
                q -= n as usize;
                s -= n as usize;
            }
        } else {
            // flip all lower-order bits of the corresponding dimension (x, y, or z)
            let mut i = n as usize * p as usize - n as usize;
            while i > r && i >= n as usize {
                hilbert_index_bitvec[i] = !hilbert_index_bitvec[i];
                i -= n as usize;
            }
        }
    }
    into_u32(hilbert_index_bitvec)
}

fn into_bit_vec(int: u32, length: usize) -> Vec<bool> {
    let mut bitvec = Vec::<bool>::new();
    // for each bit of the input u32, push a boolean into the bitvec
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
