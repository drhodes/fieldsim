use crate::types::*;
use bvh::nalgebra::{Point3, Vector3};
use std::error::Error;
use std::fs;
use std::path::Path;

impl Obj {
    pub fn from_file(filename: &Path) -> FieldResult<Obj> {
        let loader = OffLoader::from_file(filename)?;

        Ok(Obj {
            bbox: BoundingBox::from_faces(&loader.faces),
            faces: loader.faces,
        })
    }

    /// does a point p fall inside the object?
    pub fn is_point_inside(&mut self, p: &Point3<f32>) -> bool {
        let out = self.bbox.make_point_outside();
        let seg = Segment {
            p1: p.clone(),
            p2: out.clone(),
        };
        let mut num_intersections = 0;

        // consider spacehash to reduce the number of checked faces.
        // it's easier to implement than OctTree with similar performance.
        for face in &self.faces {
            if face.intersect(&seg) {
                num_intersections += 1;
            }
        }

        // println!("intersections: {:?}", num_intersections);
        return num_intersections % 2 == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cube_1() {
        let mut obj = Obj::from_file(Path::new("models/cube.off")).unwrap();
        for _ in 0..100 {
            let p = obj.bbox.make_point_inside();
            println!("point: {:?}", p);
            assert!(obj.is_point_inside(&p));
        }
    }

    #[test]
    fn sphere_1() {
        let mut obj = Obj::from_file(Path::new("models/sphere.off")).unwrap();
        let mut hits = 0;
        let n = 10;
        for _ in 0..n {
            let p = obj.bbox.make_point_inside();
            println!("point: {:?}", p);
            if obj.is_point_inside(&p) {
                hits += 1;
            }
            println!("{:?}/{:?}", hits, n);
        }
    }
}
