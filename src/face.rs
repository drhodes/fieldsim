use crate::types::*;

impl Face {
    // Ray Triangle Intersection
    // https://stackoverflow.com/questions/53962225/
    // pg 237, Computational Geometry in C
    pub fn intersects_segment(&self, seg: &Segment) -> bool {
        let [a, b, c] = &self.verts;
        let (d, e) = (&seg.p1, &seg.p2);

        let vol1 = Face::volume_sign(d, a, b, e);
        let vol2 = Face::volume_sign(d, b, c, e);
        let vol3 = Face::volume_sign(d, c, a, e);

        ((vol1 == Neg && vol2 == Neg && vol3 == Neg) || (vol1 == Pos && vol2 == Pos && vol3 == Pos))

        // ignores degenerate forms: segment in plane of triangle, segment intersects points.
    }

    // pg 131, Computational Geometry in C
    fn volume_sign(v0: &Point3, v1: &Point3, v2: &Point3, p: &Point3) -> Sign {
        let ax = v0.x - p.x;
        let ay = v0.y - p.y;
        let az = v0.z - p.z;
        let bx = v1.x - p.x;
        let by = v1.y - p.y;
        let bz = v1.z - p.z;
        let cx = v2.x - p.x;
        let cy = v2.y - p.y;
        let cz = v2.z - p.z;
        let vol = ax * (by * cz - bz * cy) + ay * (bz * cx - bx * cz) + az * (bx * cy - by * cx);
        if vol > 0.5 {
            Pos
        } else if vol < -0.5 {
            Neg
        } else {
            Zero
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vol1() {
        let o = Obj { faces: vec![] };
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let p2 = Point3::new(9.0, 0.0, 0.0);
        let p3 = Point3::new(0.0, 9.0, 0.0);
        let face = Face {
            verts: [p1, p2, p3],
        };
        let seg = Segment {
            p1: Point3::new(2.0, 2.0, -1.0),
            p2: Point3::new(2.0, 2.0, 1.0),
        };
        assert!(face.intersects_segment(&seg));
    }

    #[test]
    fn vol2() {
        let o = Obj { faces: vec![] };
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
        let b = face.intersects_segment(&seg);
        println!("{:?}", b);
        assert!(b);
    }
}
