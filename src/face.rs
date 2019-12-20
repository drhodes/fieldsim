use crate::types::*;
use nalgebra;
use std::cmp::{max, min};

impl Face {
    pub fn new(x: Point3, y: Point3, z: Point3) -> Face {
        Face { verts: [x, y, z] }
    }
    pub fn maxs(&self) -> (f64, f64, f64) {
        let xmax = f64::max(f64::max(self.verts[0].x, self.verts[1].x), self.verts[2].x);
        let ymax = f64::max(f64::max(self.verts[0].y, self.verts[1].y), self.verts[2].y);
        let zmax = f64::max(f64::max(self.verts[0].z, self.verts[1].z), self.verts[2].z);
        (xmax, ymax, zmax)
    }

    pub fn mins(&self) -> (f64, f64, f64) {
        let xmin = f64::min(f64::min(self.verts[0].x, self.verts[1].x), self.verts[2].x);
        let ymin = f64::min(f64::min(self.verts[0].y, self.verts[1].y), self.verts[2].y);
        let zmin = f64::min(f64::min(self.verts[0].z, self.verts[1].z), self.verts[2].z);
        (xmin, ymin, zmin)
    }

    pub fn intersect(&self, seg: &Segment) -> bool {
        self.moller_trumbore(seg)
    }

    pub fn moller_trumbore(&self, seg: &Segment) -> bool {
        const EPSILON: f64 = 1e-8;
        let [v0, v1, v2] = &self.verts;
        let (ray_origin, ray_end) = (&seg.p1, &seg.p2);
        let mut ray = ray_end - ray_origin;

        ray = ray.scalar_mul(1.0 / ray.len());

        let edge1 = v1 - v2;
        let edge2 = v2 - v0;
        let h = ray.cross(&edge2);
        let a = edge1.dot(&h);

        if a > -EPSILON || a < EPSILON {
            return false;
        }

        let f = 1.0 / a;
        let s = ray_origin - v0;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = s.cross(&edge1);
        let v = f * ray.dot(&q);
        let t = f * edge2.dot(&q);

        return t > EPSILON && t < 1.0 / EPSILON;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vol0() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let p2 = Point3::new(9.0, 0.0, 0.0);
        let p3 = Point3::new(0.0, 9.0, 0.0);
        let face = Face::new(p3, p1, p2);
        let seg = Segment {
            p1: Point3::new(2.0, 2.0, 1.0),
            p2: Point3::new(2.0, 2.0, -1.0),
        };
        assert!(face.intersect(&seg));
    }

    #[test]
    fn vol1() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let p2 = Point3::new(9.0, 0.0, 0.0);
        let p3 = Point3::new(0.0, 9.0, 0.0);
        let face = Face {
            verts: [p1, p2, p3],
        };
        let seg = Segment {
            p1: Point3::new(1.0, 1.0, -1.0),
            p2: Point3::new(1.0, 1.0, 1.0),
        };
        let b = face.intersect(&seg);
        assert!(b);
    }

    //#[test]
    fn face1() {
        let face = Face::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(5.0, 0.0, 0.0),
            Point3::new(5.0, 0.0, 5.0),
        );
        let seg = Segment {
            p1: Point3::new(2.2787395838278246, 1.473816575700314, 1.869434601797007),
            p2: Point3::new(10.1, 10.1, 10.1),
        };
        let b = face.intersect(&seg);
        assert!(!b);
    }

    #[test]
    fn face2() {
        let face = Face::new(
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
        );
        let seg = Segment {
            p1: Point3::new(0.0, 0.0, 0.0),
            p2: Point3::new(6.0, 6.0, 6.0),
        };
        let b = face.intersect(&seg);
        assert!(b);
    }

    #[test]
    fn face3() {
        let face = Face::new(
            Point3::new(0.0, -6.0, 0.0),
            Point3::new(0.0, 0.0, -6.0),
            Point3::new(-6.0, 0.0, 0.0),
        );
        let seg = Segment {
            p1: Point3::new(0.0, 0.0, 0.0),
            p2: Point3::new(1.0, 1.0, 1.0),
        };
        let b = face.intersect(&seg);
        assert!(!b);
    }
}
