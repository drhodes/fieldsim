use crate::types::*;
use bvh::aabb::AABB;
use bvh::nalgebra::{Point3, Vector3};
use bvh::ray::Ray;
use std::cmp::{max, min};

impl Face {
    pub fn new(x: Point3<f32>, y: Point3<f32>, z: Point3<f32>) -> Face {
        Face { verts: [x, y, z] }
    }
    pub fn maxs(&self) -> (f32, f32, f32) {
        let xmax = f32::max(f32::max(self.verts[0].x, self.verts[1].x), self.verts[2].x);
        let ymax = f32::max(f32::max(self.verts[0].y, self.verts[1].y), self.verts[2].y);
        let zmax = f32::max(f32::max(self.verts[0].z, self.verts[1].z), self.verts[2].z);
        (xmax, ymax, zmax)
    }

    pub fn mins(&self) -> (f32, f32, f32) {
        let xmin = f32::min(f32::min(self.verts[0].x, self.verts[1].x), self.verts[2].x);
        let ymin = f32::min(f32::min(self.verts[0].y, self.verts[1].y), self.verts[2].y);
        let zmin = f32::min(f32::min(self.verts[0].z, self.verts[1].z), self.verts[2].z);
        (xmin, ymin, zmin)
    }

    pub fn intersect(&self, seg: &Segment) -> bool {
        return self.jimenez(seg);
        // let dir = seg.p2 - seg.p1;
        // let ray = Ray::new(seg.p1, dir);
        // let intersection = ray.intersects_triangle(&self.verts[0], &self.verts[1], &self.verts[2]);
        // intersection.distance != std::f32::INFINITY
    }

    pub fn jimenez(&self, seg: &Segment) -> bool {
        let [v1, v2, v3] = &self.verts;
        let (q1, q2) = (&seg.p1, &seg.p2);

        let eps = 1e-6;

        let a = q1 - v3;
        let b = v1 - v3;
        let c = v2 - v3;
        let w1 = b.cross(&c);
        let w = a.dot(&w1);
        let d = q2 - v3;
        let s = d.dot(&w1);

        if w > eps {
            if s > eps {
                return false;
            }
            let w2 = a.cross(&d);
            let t = w2.dot(&c);
            if t < -eps {
                return false;
            }
            let u = -(w2.dot(&b));
            if u < -eps {
                return false;
            }
            if w < (s + t + u) {
                return false;
            }
        } else if w < -eps {
            if s < -eps {
                return false;
            }
            let w2 = a.cross(&d);
            let t = w2.dot(&c);
            if t < eps {
                return false;
            }
            let u = -w2.dot(&b);
            if u > eps {
                return false;
            }
            if w > (s + t + u) {
                return false;
            }
        } else {
            if s > eps {
                let w2 = d.cross(&a);
                let t = w2.dot(&c);
                if t < -eps {
                    return false;
                }
                let u = -w2.dot(&b);
                if u < -eps {
                    return false;
                }
                if -s < (t + u) {
                    return false;
                }
            } else if s < -eps {
                let w2 = d.cross(&a);
                let t = w2.dot(&c);
                if t > eps {
                    return false;
                }
                let u = -w2.dot(&b);
                if u > eps {
                    return false;
                }
                if -s > (t + u) {
                    return false;
                }
            } else {
                return false;
            }
        }
        return true;
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
        let p1 = Point3::new(9.0, 0.0, 0.0);
        let p2 = Point3::new(0.0, 0.0, 0.0);
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

    #[test]
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
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
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
        let p1 = Point3::new(-6.0, 0.0, 0.0);
        let p2 = Point3::new(0.0, -6.0, 0.0);
        let p3 = Point3::new(0.0, 0.0, -6.0);
        let face = Face::new(p1, p2, p3);
        let seg = Segment {
            p1: Point3::new(0.0, 0.0, 0.0),
            p2: Point3::new(1.0, 1.0, 1.0),
        };
        let b = face.intersect(&seg);
        assert!(!b);
    }
}
