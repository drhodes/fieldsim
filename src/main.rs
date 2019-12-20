use bvh::nalgebra::{Point3, Vector3};
use fieldsim::types::*;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() {
    let mut obj = Obj::from_file(Path::new("models/pi-sphere.off")).unwrap();
    let mut hits = 0;
    let n = 1000;
    for _ in 0..n {
        let p = obj.bbox.make_point_inside();
        if obj.is_point_inside(&p) {
            hits += 1;
        }
    }
    println!("π: {:?}", 6.0 * ((hits as f32) / (n as f32)));
}
