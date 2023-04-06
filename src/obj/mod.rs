use std::{error::Error, fs::File, io::Write, path::PathBuf};

pub struct Obj {
    pub vertices: Vec<(f32, f32, f32)>,
    pub path: Vec<(usize, usize, usize)>,
}

impl Obj {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            path: Vec::new(),
        }
    }

    pub fn write(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(&path)?;
        file.write_all(format!("# {}\n# \n", path.display()).as_bytes())?;
        file.write_all(format!("# {} vertices\n", self.vertices.len()).as_bytes())?;
        for vertex in &self.vertices {
            file.write_all(
                format!("v {:.5} {:.5} {:.5}\n", vertex.0, vertex.1, vertex.2).as_bytes(),
            )?;
        }
        file.write_all(format!("\n\n# {} edges\n", self.path.len()).as_bytes())?;
        for edge in &self.path {
            file.write_all(format!("f {} {} {}\n", edge.0, edge.1, edge.2).as_bytes())?;
        }
        Ok(())
    }
}
