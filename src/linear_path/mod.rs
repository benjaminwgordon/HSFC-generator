use std::{error::Error, fs::File, io::Write, path::PathBuf};

use crate::brgc::Brgc;

#[derive(Debug)]
pub struct LinearPath {
    vertices: Vec<(f32, f32)>,
    path: Vec<(usize, usize)>,
}

impl LinearPath {
    pub fn from_brgc(iterator: impl Iterator<Item = u32>, length: usize) -> Self {
        // we can convert gray_code to x,y coordinates by taking alternate bits
        let vertices: Vec<(f32, f32)> = iterator
            .take(length)
            .map(|gray_code| {
                let gray_code = format!("{:04b}", gray_code);
                let x_bin: String = gray_code.chars().step_by(2).collect::<String>();
                let y_bin: String = gray_code.chars().skip(1).step_by(2).collect::<String>();

                // let x_bin = Brgc::binary_from_gray(x_gray);
                // let y_bin = Brgc::binary_from_gray(y_gray);

                let x_dec = u32::from_str_radix(x_bin.to_string().as_ref(), 2).unwrap();
                let y_dec = u32::from_str_radix(y_bin.to_string().as_ref(), 2).unwrap();

                println!("({},{})", x_dec, y_dec);

                (x_dec as f32, y_dec as f32)
            })
            .collect();

        let mut path = Vec::new();
        for i in 0..(vertices.len() - 1) {
            path.push((i, i + 1));
        }

        Self { vertices, path }
    }

    // creates a new linearPath that replaces simple connections with triangle geometry
    // squares represent vertices
    // rectangles represent edges
    pub fn to_2d_square_and_circles(&self, thickness: f32) -> Self {
        let mut squares = Vec::<(f32, f32)>::new();
        let mut rectangles = Vec::<(f32, f32)>::new();
        let mut paths = Vec::<(usize, usize, usize)>::new();

        // create a square with radius 'thickness' / 4 per vertex
        for vertex in &self.vertices {
            let (vertices, polypaths) =
                LinearPath::center_square(squares.len(), *vertex, thickness);
            squares.extend(vertices);
            paths.extend(polypaths);
        }
        // create a rectangle with width 'thickness' / 8 per edge
        todo!()
    }

    // returns vertices and triangulated poly-paths for a square centered on a vertex
    fn center_square(
        starting_index: usize,
        center: (f32, f32),
        radius: f32,
    ) -> (Vec<(f32, f32)>, Vec<(usize, usize, usize)>) {
        let mut vertices: Vec<(f32, f32)> = Vec::new();
        let offset = 0.7071 * radius;
        println!("plotting vertex centered at: {:?}", center);
        vertices.push((center.0 - offset, center.1 - offset));
        vertices.push((center.0 + offset, center.1 - offset));
        vertices.push((center.0 - offset, center.1 + offset));
        vertices.push((center.0 + offset, center.1 + offset));

        //printing
        for vertex in &vertices {
            println!("{}, {}", vertex.0, vertex.1);
        }

        let mut polypaths = Vec::<(usize, usize, usize)>::new();
        let t1: Vec<usize> = [0, 1, 2]
            .iter()
            .map(|index| index + starting_index)
            .collect();
        let t2: Vec<usize> = [1, 2, 3]
            .iter()
            .map(|index| index + starting_index)
            .collect();
        polypaths.push((t1[0], t1[1], t1[2]));
        polypaths.push((t2[0], t2[1], t2[2]));
        (vertices, polypaths)
    }

    pub fn write_to_obj(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(&path)?;
        file.write_all(format!("# {}\n# \n", path.display()).as_bytes())?;
        file.write_all(format!("# {} vertices\n", self.vertices.len()).as_bytes())?;
        for vertex in &self.vertices {
            file.write_all(format!("v {} {} 0.0\n", vertex.0, vertex.1).as_bytes())?;
        }
        file.write_all(format!("\n\n# {} edges\n", self.path.len()).as_bytes())?;
        for edge in &self.path {
            file.write_all(format!("f {} {}\n", edge.0, edge.1).as_bytes())?;
        }
        Ok(())
    }
}
