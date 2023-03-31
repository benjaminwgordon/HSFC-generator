mod brgc;

use brgc::Brgc;

fn main() {
    let mut brgc = Brgc { index: 0 };
    let n = 4;

    for i in 0..(2_i32.pow(n)) {
        let gray = brgc.next().unwrap();
        let gray_as_b_string = format!("{:04b}", gray);
        let x_coord: String = gray_as_b_string.chars().step_by(2).collect();
        let y_coord: String = gray_as_b_string.chars().skip(1).step_by(2).collect();
        // println!(
        //     "gray_code: {:04b}\n   (x,y): ({}, {})",
        //     gray, x_coord, y_coord
        // );
        println!("{}, {}", x_coord, y_coord);
    }
}
