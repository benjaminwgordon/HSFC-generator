use hilbert_in_cartesian::{HilbertCurve, LinearPath};
use std::{env, error::Error, path::PathBuf};

/**
 * Example usage of the library
 *
 * Runs as a CLI executable with 3 arguments:
 *  n: the number of dimensions to generate (2D vs 3D)
 *      allowed values: [2,3]
 *      
 *  p: the number of bits to allocate for each coordinate
 *      allowed values: [0,8]
 *      (each axis of your resulting curve will have 2^p vertices)
 *          
 *  target: the relative path to write the resulting OBJ file contents to
 *
 */
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let n = &args[1].parse::<u32>().unwrap();
    let p = &args[2].parse::<u32>().unwrap();
    let out_dir = PathBuf::from(&args[3]);

    let hilbert_curve = HilbertCurve::new(*n, *p).unwrap();
    let linear_path = LinearPath::from_vec(hilbert_curve.coordinates, *n).unwrap();
    let pipes_and_boxes = linear_path.to_2d_edges_and_vertices_obj(0.25);
    pipes_and_boxes.write(out_dir)?;

    Ok(())
}
