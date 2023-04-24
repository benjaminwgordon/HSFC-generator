// Binary Reflected Gray Code
/* a BRGC is an ordering of the binary numeral system that holds the property that
   sequential numbers differ by exactly 1 bit.

   In a standard binary numeral ordering, moving from 7 to 8 requires 4 bits to change:
        (0111 -> 1000)
   whereas in brgc only a single bit is changed, due to a different ordering of
   decimal -> binary numbers

   BRGC's are stable, meaning once a number has appeared in a smaller order Gray code,
   it will also be in the same position in all higher order Gray codes.

   This means we do not need to know the bit count to generate any BRGC, making it safe
   to implement an iterator over each number in a BRGC without knowing our bit count
   ahead of time
*/
#[derive(Debug)]
pub struct Brgc {
    pub index: u32,
}

impl Iterator for Brgc {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let gray = self.index ^ (self.index >> 1);
        self.index += 1;
        Some(gray)
    }
}

impl Brgc {
    pub fn binary_from_gray(gray_code: u32) -> u32 {
        let mut mask: u32 = gray_code;
        let mut out = gray_code;
        while mask != 0 {
            mask >>= 1;
            out ^= mask;
        }
        out
    }
}

#[cfg(test)]
mod tests {

    use crate::Brgc;
    #[test]
    fn take_16_gray_codes() {
        let brgc = Brgc { index: 0 };
        let result: Vec<u32> = brgc.take(16).collect();
        let first_16_brgc_expected = [
            0b0000u32, 0b0001u32, 0b0011u32, 0b0010u32, 0b0110u32, 0b0111u32, 0b0101u32, 0b0100u32,
            0b1100u32, 0b1101u32, 0b1111u32, 0b1110u32, 0b1010u32, 0b1011u32, 0b1001u32, 0b1000u32,
        ];
        assert_eq!(result, Vec::from(first_16_brgc_expected));
    }

    #[test]
    fn convert_first_16_graycodes_to_binary() {
        let brgc = Brgc { index: 0 };
        let result: Vec<u32> = brgc
            .take(16)
            .map(|gray| Brgc::binary_from_gray(gray))
            .collect();
        let first_16_standard_binary = [
            0b0000u32, 0b0001u32, 0b0010u32, 0b0011u32, 0b0100u32, 0b0101u32, 0b0110u32, 0b0111u32,
            0b1000u32, 0b1001u32, 0b1010u32, 0b1011u32, 0b1100u32, 0b1101u32, 0b1110u32, 0b1111u32,
        ];
        assert_eq!(result, first_16_standard_binary);
    }
}
