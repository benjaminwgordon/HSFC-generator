mod brgc;
mod linear_path;
use std::{error::Error, path::PathBuf};

use brgc::Brgc;
use linear_path::LinearPath;

fn main() -> Result<(), Box<dyn Error>> {
    let brgc = Brgc { index: 0 };
    let n = 4;

    let linear_path = LinearPath::from_brgc(brgc, 2_usize.pow(n));
    let pipes_and_boxes = linear_path.to_2d_square_and_circles(0.5);

    pipes_and_boxes.write_to_obj(PathBuf::from("out.obj"))?;
    Ok(())
}
