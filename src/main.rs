use fieldsim::types::*;
use std::path::Path;

fn main() {
    let mut obj = Obj::from_file(Path::new("models/pi-sphere.off")).unwrap();
    let mut hits = 0;
    let n = 1000;
    for _ in 0..n {
        // make a p inside the bounding box, not necessarily inside the object,
        let p = obj.bbox.make_point_inside();

        // well, is the point inside the object?
        if obj.is_point_inside(&p) {
            hits += 1;
        }
    }
    println!("π: {:?}", 6.0 * ((hits as f32) / (n as f32)));
}
