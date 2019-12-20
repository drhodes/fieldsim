use fieldsim::types::*;

fn main() {
    let face = Face::new(
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, 1.0),
    );
    let seg = Segment {
        p1: Point3::new(0.0, 0.0, 0.0),
        p2: Point3::new(1.0, 1.0, 1.0),
    };
    let b = face.intersect(&seg);
    assert!(b)
}
