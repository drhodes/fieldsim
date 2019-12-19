//use std::error::Error;
//use std::fs;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

use crate::types::*;

impl OffLoader {
    pub fn from_file(filename: &Path) -> FieldResult<OffLoader> {
        let f = File::open(filename)?;
        let f = BufReader::new(f);

        let mut loader = OffLoader {
            verts: vec![],
            faces: vec![],
        };

        let mut fiter = f.lines();
        loader.consume_header(&mut fiter);
        let (num_verts, num_faces) = loader.consume_stats(&mut fiter)?;
        loader.consume_verts_and_faces(&mut fiter);

        assert_eq!(num_verts, loader.verts.len());
        assert_eq!(num_faces, loader.faces.len());
        Ok(loader)
    }

    fn add_vert(&mut self, floats: Vec<&str>) -> FieldResult<()> {
        let x: f64 = floats[0].parse()?;
        let y: f64 = floats[1].parse()?;
        let z: f64 = floats[2].parse()?;
        self.verts.push(Point3 { x, y, z });
        Ok(())
    }

    fn add_face(&mut self, floats: Vec<&str>) -> FieldResult<()> {
        // this loader assume that each face is a triangle.  A more
        // robust program will load quads too then split them into
        // triangles.
        assert_eq!(floats[0], "3");
        let i1: usize = floats[1].parse()?;
        let i2: usize = floats[2].parse()?;
        let i3: usize = floats[3].parse()?;
        let v1 = self.verts[i1].clone();
        let v2 = self.verts[i2].clone();
        let v3 = self.verts[i3].clone();

        self.faces.push(Face {
            verts: [v1, v2, v3],
        });

        Ok(())
    }

    fn consume_verts_and_faces(&mut self, lines: &mut Lines<BufReader<File>>) -> FieldResult<()> {
        // consume the lines that tells how many verts and faces are in the file.
        for line in lines {
            let line = line.expect("unable to read stats");
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.len() {
                3 => self.add_vert(parts),
                4 => self.add_face(parts),
                _ => continue,
            };
        }
        Ok(())
    }

    fn consume_stats(&mut self, lines: &mut Lines<BufReader<File>>) -> FieldResult<(usize, usize)> {
        // consume the lines that tells how many verts and faces are in the file.
        let pat = Regex::new(r"^([0-9]+)\s([0-9]+)\s([0-9]+)$")?;

        for line in lines {
            let line = line.expect("unable to read stats");
            if pat.is_match(line.as_str()) {
                let caps = pat.captures(line.as_str()).unwrap();
                let num_verts: usize = *&caps[1].parse()?;
                let num_faces: usize = *&caps[2].parse()?;
                return Ok((num_verts, num_faces));
            }
        }
        bail!("something");
    }

    fn consume_header(&mut self, lines: &mut Lines<BufReader<File>>) -> FieldResult<()> {
        // consume the OFF header
        for line in lines {
            let line = line.expect("unable to read header");
            if line == "OFF" {
                return Ok(());
            } else {
                continue;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere() {
        OffLoader::from_file(Path::new("models/sphere.off"));
    }

    #[test]
    fn cube() {
        OffLoader::from_file(Path::new("models/cube.off"));
    }
}
