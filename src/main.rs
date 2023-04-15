mod brgc;
mod linear_path;
mod obj;
mod skilling_transform;
use std::{env, error::Error, path::PathBuf};

use brgc::Brgc;
use linear_path::LinearPath;
use skilling_transform::skilling_transform;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // construct an iterator of Binary Reflected Gray Codes
    let brgc = Brgc { index: 0 };

    // let n be the number of dimensions to generate (2-3 are ideal for graphing)
    let n = &args[1].parse::<u32>().unwrap();

    // let p be the number of bits to assign to each dimension
    // each side of the constructed rectangle/rectangular prism will have length 2^n
    let p = &args[2].parse::<u32>().unwrap();

    let skilling = &args[3].parse::<bool>().unwrap();

    let vertices_per_axis = 2_usize.pow(*p);
    let total_vertices = vertices_per_axis.pow(*n);

    println!("STATUS: generating {total_vertices} vertices of Binary Reflected Gray Code");
    let brgc_vec: Vec<u32> = brgc.take(total_vertices).collect();
    println!(
        "STATUS: succeeded generating {total_vertices} vertices of Binary Reflected Gray Code"
    );

    // optionally apply the skilling transform
    let skilling_transformed = match *skilling {
        true => {
            println!("STATUS: Applying the skilling transform to {total_vertices} vertices");
            println!("{:?}", brgc_vec);
            let transformed_brgc = skilling_transform(brgc_vec.clone(), *n, *p);
            println!("Hilbert  -> Skilling -> ( Xbin, Ybin )  -> (X,Y)");
            println!("- - - - - - - - - - - - - - - - - - - - - - - - ");
            for i in 0..transformed_brgc.len() {
                println!(
                    "{:08b} -> {:08b} -> ({:?},{:?}) -> ({},{})",
                    brgc_vec[i],
                    transformed_brgc[i],
                    format!("{:08b}", transformed_brgc[i])
                        .chars()
                        .step_by(2)
                        .collect::<String>(),
                    format!("{:08b}", transformed_brgc[i])
                        .chars()
                        .skip(1)
                        .step_by(2)
                        .collect::<String>(),
                    u32::from_str_radix(
                        &format!("{:032b}", transformed_brgc[i])
                            .chars()
                            .step_by(2)
                            .collect::<String>(),
                        2
                    )
                    .unwrap(),
                    u32::from_str_radix(
                        &format!("{:032b}", transformed_brgc[i])
                            .chars()
                            .skip(1)
                            .step_by(2)
                            .collect::<String>(),
                        2
                    )
                    .unwrap()
                );
            }
            println!(
                "STATUS: succeeded applying the skilling transform to {total_vertices} vertices"
            );
            transformed_brgc
        }
        false => brgc_vec,
    };

    println!("STATUS: generating {total_vertices} vertices of Binary Reflected Gray Code");
    let linear_path =
        LinearPath::from_brgc_vec(skilling_transformed, total_vertices, *n, *p).unwrap();
    println!(
        "STATUS: succeeded generating {total_vertices} vertices of Binary Reflected Gray Code"
    );

    println!("STATUS: generating geometry data for render of {total_vertices} vertices");
    let pipes_and_boxes = linear_path.to_2d_edges_and_vertices_obj(0.25);
    println!("STATUS: succeeded generating geometry data for render of {total_vertices} vertices");

    let total_obj_lines = pipes_and_boxes.vertices.len() + pipes_and_boxes.path.len();

    println!("STATUS: writing {total_obj_lines} lines of OBJ data to disc");
    pipes_and_boxes.write(PathBuf::from("out.obj"))?;
    println!("STATUS: succeeded writing {total_obj_lines} lines of OBJ data to disc");

    Ok(())
}
