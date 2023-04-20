use std::error::Error;

use glam::{Quat, Vec3};

use crate::obj::Obj;

#[derive(Debug)]
pub struct LinearPath {
    vertices: Vec<(f32, f32, f32)>,
    path: Vec<(usize, usize)>,
}

impl LinearPath {
    // given a Vec of u32, generate a LinearPath representing
    // X,Y,Z coordinates in a 3d rectangular prism of vertices with a single
    // non-cyclic traversal path

    // This can be done by taking every other bit of the BRGC, and assigning it to a
    // binary encoded whole number representing one coordinate in space

    // e.g. BRGC 011010
    //           0 -> x : 0
    //           1 -> y : 1
    //           1 -> x : 01
    //           0 -> y : 10
    //           1 -> x : 011
    //           0 -> y : 010
    //           x coordinate: 011 -> 3 in decimal
    //           y coordinate: 010 -> 2 in decimal
    //           full coordinate: (3,2) in cartesian space
    pub fn from_brgc_vec(
        brgc: Vec<u32>,
        length: usize,
        n: u32,
        p: u32,
    ) -> Result<Self, Box<dyn Error>> {
        if !(n == 2 || n == 3) {
            return Err(format!(
                "Cannot generate Hilbert geometry for {n} dimesions. Supported dimensions are 2 and 3"
            )
            .into());
        }
        let vertices: Vec<(f32, f32, f32)> = brgc
            .iter()
            .map(|gray_code| {
                let gray_code = format!("{:064b}", gray_code);

                match n {
                    2 => {
                        // map bits of BRGC to X and Y space
                        let x_bin: String = gray_code
                            .chars()
                            .step_by(2.try_into().unwrap())
                            .collect::<String>();
                        let y_bin: String =
                            gray_code.chars().skip(1).step_by(2).collect::<String>();

                        let x_dec = u32::from_str_radix(x_bin.as_ref(), 2).unwrap();
                        let y_dec = u32::from_str_radix(y_bin.as_ref(), 2).unwrap();

                        (x_dec as f32, y_dec as f32, 0.0)
                    }
                    3 => {
                        // map bits of BRGC to X,Y,Z space
                        let x_bin: String = gray_code
                            .chars()
                            .step_by(3.try_into().unwrap())
                            .collect::<String>();
                        let y_bin: String =
                            gray_code.chars().skip(1).step_by(3).collect::<String>();
                        let z_bin: String =
                            gray_code.chars().skip(2).step_by(3).collect::<String>();

                        let x_dec = u32::from_str_radix(x_bin.as_ref(), 2).unwrap();
                        let y_dec = u32::from_str_radix(y_bin.as_ref(), 2).unwrap();
                        let z_dec = u32::from_str_radix(z_bin.as_ref(), 2).unwrap();

                        (x_dec as f32, y_dec as f32, z_dec as f32)
                    }
                    _ => !unreachable!(),
                }
            })
            .collect();

        let mut path = Vec::new();
        for i in 0..(vertices.len() - 1) {
            path.push((i, i + 1));
        }

        Ok(Self { vertices, path })
    }

    // creates a new linearPath that replaces simple connections with triangle geometry
    // squares represent vertices
    // rectangles represent edges
    pub fn to_2d_edges_and_vertices_obj(&self, thickness: f32) -> Obj {
        let mut squares = Vec::<(f32, f32, f32)>::new(); // represent vertex coordinates
        let mut rectangles = Vec::<(f32, f32, f32)>::new(); // represent edge coordinates
        let mut paths = Vec::<(usize, usize, usize)>::new(); // draw path between vertices

        // create a square with radius 'thickness' / 4 per vertex
        for vertex in &self.vertices {
            let (vertices, polypaths) =
                LinearPath::center_square(squares.len(), *vertex, thickness / 2.0);
            squares.extend(vertices);
            paths.extend(polypaths);
        }

        // create a rectangle with width 'thickness' / 8 per edge (sequential pair of vertices)
        for i in 0..(self.vertices.len() - 1) {
            let v1 = self.vertices.get(i).unwrap();
            let v2 = self.vertices.get(i + 1).unwrap();
            let (vertices, polypaths) = Self::midpoint_to_midpoint_rect(
                (v1.0, v1.1, v1.2),
                (v2.0, v2.1, v2.2),
                thickness / 8.0,
                squares.len() + rectangles.len(),
            )
            .unwrap();
            rectangles.extend(vertices);
            paths.extend(polypaths);
        }

        squares.extend(rectangles);

        // polypaths are written using vertex indices in base 1, not base 0
        paths = paths
            .iter_mut()
            .map(|f| (f.0 + 1, f.1 + 1, f.2 + 1))
            .collect();

        // replace this dummy object
        Obj {
            vertices: squares,
            path: paths,
        }
    }

    // given two coordinates (x1,y1, _), (x2, y2, _) presuming z coordinates are always 0.0
    // return the coordinates of a rectangle so that (x1,y1, _) is the midpoint of one edge
    // and (x2, y2, _) is the midpoints of the edge on the opposing side of the rectangle
    pub fn midpoint_to_midpoint_rect(
        first_midpoint: (f32, f32, f32),
        second_midpoint: (f32, f32, f32),
        width: f32,
        starting_index: usize,
    ) -> Result<(Vec<(f32, f32, f32)>, Vec<(usize, usize, usize)>), Box<dyn Error>> {
        let mut vertices: Vec<(f32, f32, f32)> = Vec::new();
        let mut path: Vec<(usize, usize, usize)> = Vec::new();

        let v1 = Vec3::from(first_midpoint);
        let v2 = Vec3::from(second_midpoint);

        let midpoint_to_midpoint_vec = v2 - v1;
        let midpoint_to_midpoint_dist = midpoint_to_midpoint_vec.length();

        if midpoint_to_midpoint_dist.abs() != 1.0 {
            return Err(format!("v1-v2 distance is not 1 v2:{v2} v1:{v1} v2-v1{}", v2 - v1).into());
        }

        let is_horizontal = midpoint_to_midpoint_vec.normalize() == Vec3::X
            || midpoint_to_midpoint_vec.normalize() == Vec3::NEG_X;
        let is_vertical = midpoint_to_midpoint_vec.normalize() == Vec3::Y
            || midpoint_to_midpoint_vec.normalize() == Vec3::NEG_Y;
        let is_inward = midpoint_to_midpoint_vec.normalize() == Vec3::Z
            || midpoint_to_midpoint_vec.normalize() == Vec3::NEG_Z;

        if is_horizontal || is_vertical || is_inward {
            let rotation = midpoint_to_midpoint_vec.angle_between(Vec3::new(0.0, 1.0, 0.0));

            // generate 4 points of rectangle edges
            let quat = Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, rotation);

            let v1_left = v1 + quat.mul_vec3(Vec3::NEG_X * width);
            let v1_right = v1 + quat.mul_vec3(Vec3::X * width);
            let v2_left = v2 + quat.mul_vec3(Vec3::NEG_X * width);
            let v2_right = v2 + quat.mul_vec3(Vec3::X * width);

            for v in [v1_left, v1_right, v2_left, v2_right] {
                vertices.push((v.x, v.y, v.z));
            }

            path.push((starting_index, starting_index + 1, starting_index + 2));
            path.push((starting_index + 1, starting_index + 2, starting_index + 3));
        } else {
            println!(
                "ERROR: not a vertical or horizontal v2-v1:\nv1: {}\nv2: {}\nv2-v1: {}",
                v1, v2, midpoint_to_midpoint_vec
            );
            return Err(
                "sequential pairs of vertices in path should be vertical or horizontal".into(),
            );
        }

        Ok((vertices, path))
    }

    // returns vertices and triangulated poly-paths for a square centered on a vertex
    fn center_square(
        starting_index: usize,
        center: (f32, f32, f32),
        radius: f32,
    ) -> (Vec<(f32, f32, f32)>, Vec<(usize, usize, usize)>) {
        let mut vertices: Vec<(f32, f32, f32)> = Vec::new();
        let offset = 0.7071 * radius;

        vertices.push((center.0 - offset, center.1 - offset, center.2 + offset));
        vertices.push((center.0 + offset, center.1 - offset, center.2 + offset));
        vertices.push((center.0 - offset, center.1 + offset, center.2 + offset));
        vertices.push((center.0 + offset, center.1 + offset, center.2 + offset));

        vertices.push((center.0 - offset, center.1 - offset, center.2 - offset));
        vertices.push((center.0 + offset, center.1 - offset, center.2 - offset));
        vertices.push((center.0 - offset, center.1 + offset, center.2 - offset));
        vertices.push((center.0 + offset, center.1 + offset, center.2 - offset));

        let mut polypaths = Vec::<(usize, usize, usize)>::new();
        let cube_paths = [
            [0, 1, 2],
            [3, 2, 1],
            [0, 2, 4],
            [6, 4, 2],
            [2, 3, 6],
            [7, 6, 3],
            [1, 3, 5],
            [7, 5, 3],
            [0, 1, 4],
            [5, 4, 1],
            [4, 5, 6],
            [7, 6, 5],
        ];

        let cube_vertex_index_path = cube_paths
            .iter()
            .map(|path| {
                path.iter()
                    .map(|index| index + starting_index)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        cube_vertex_index_path.iter().for_each(|path| {
            polypaths.push((
                *path.get(0).unwrap(),
                *path.get(1).unwrap(),
                *path.get(2).unwrap(),
            ));
        });

        (vertices, polypaths)
    }
}
