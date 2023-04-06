mod brgc;
mod linear_path;
mod obj;
use std::{error::Error, path::PathBuf};

use brgc::Brgc;
use linear_path::LinearPath;

fn main() -> Result<(), Box<dyn Error>> {
    let brgc = Brgc { index: 0 };
    let n = 14;

    let linear_path = LinearPath::from_brgc(brgc, 2_usize.pow(n));
    let pipes_and_boxes = linear_path.to_2d_edges_and_vertices_obj(0.25);

    pipes_and_boxes.write(PathBuf::from("out.obj"))?;
    Ok(())
}
