use crate::types::*;

impl Face {
    // Ray Triangle Intersection
    // https://stackoverflow.com/questions/53962225/
    pub fn intersects_segment(&self, _seg: &Segment) -> bool {
        // let mut intersects = false;
        // for face in self.faces {
        //     let vol1 = self.tetrahedron_volume(&[face[0], face[2], face[3], seg[0]]);
        //     if vol1 == 0.0 {
        //         continue;
        //     }
        //     let vol2 = self.tetrahedron_volume(&[face[0], face[2], face[3], seg[1]]);
        //     if vol2 == 0.0 {
        //         continue;
        //     }
        //     let vol3 = self.tetrahedron_volume(&[face[0], face[2], face[3], seg[1]]);
        //     if vol3 == 0.0 {
        //         continue;
        //     }
        //     if vol1 < 0.0 && vol2 < 0.0 && vol3 < 0.0 {
        //         intersects = !intersects;
        //         continue;
        //     }
        //     if vol1 > 0.0 && vol2 > 0.0 && vol3 > 0.0 {
        //         intersects = !intersects;
        //         continue;
        //     }
        unimplemented!()
    }

    // the math
    // https://stackoverflow.com/questions/53962225/
    fn tetrahedron_volume(&self, ps: &[Point3; 4]) -> f64 {
        // 4x4 determinant.
        // can this be vectorized?
        let (a, b, c, d) = (ps[0].x, ps[0].y, ps[0].z, 1.0);
        let (e, f, g, h) = (ps[1].x, ps[1].y, ps[1].z, 1.0);
        let (i, j, k, l) = (ps[2].x, ps[2].y, ps[2].z, 1.0);
        let (m, n, o, p) = (ps[3].x, ps[3].y, ps[3].z, 1.0);

        let s1 = a * (f * k * p - f * l * o - g * j * p + g * l * n + h * j * o - h * k * n);
        let s2 = b * (e * k * p - e * l * o - g * i * p + g * l * m + h * j * o - h * k * m);
        let s3 = c * (e * j * p - e * l * n - f * i * p + f * l * m + h * i * n - h * j * m);
        let s4 = d * (e * j * o - e * k * n - f * i * o + f * k * m + g * i * n - g * j * m);

        (s1 + s2 + s3 + s4) / 6.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vol1() {
        let o = Obj { faces: vec![] };
        let p1 = Point3::new(0.0, 0.0, 1.0);
        let p2 = Point3::new(1.0, 0.0, 0.0);
        let p3 = Point3::new(0.0, 1.0, 0.0);
        let p4 = Point3::new(0.0, 0.0, 0.0);

        println!("{:?}", o.tetrahedron_volume(&[p1, p2, p3, p4]));
    }
}
