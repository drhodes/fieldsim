use crate::types::*;
use rand::Rng;

impl BoundingBox {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn from_faces(faces: &Vec<Face>) -> BoundingBox {
        const SMALL: f64 = 1e-20;
        const BIG: f64 = 1e20;
        
        let (mut xmin, mut xmax) = (BIG, SMALL);
        let (mut ymin, mut ymax) = (BIG, SMALL);
        let (mut zmin, mut zmax) = (BIG, SMALL);
        for face in faces {
            let (x, y, z) = face.mins();
            if x < xmin { xmin = x }
            if y < ymin { ymin = y }
            if z < zmin { zmin = z }
            
            let (x, y, z) = face.maxs();
            if x > xmax { xmax = x }
            if y > ymax { ymax = y }
            if z > zmax { zmax = z }
        }

        let rng = rand::thread_rng();
        BoundingBox{rng, xmin, xmax, ymin, ymax, zmin, zmax}
    }

    pub fn make_point_inside(&mut self) -> Point3 {
        let dx = self.xmax - self.xmin;
        let x = self.xmin + dx * self.rng.gen::<f64>();
        let dy = self.ymax - self.ymin;
        let y = self.ymin + dy * self.rng.gen::<f64>();
        let dz = self.zmax - self.zmin;
        let z = self.zmin + dz * self.rng.gen::<f64>();
        Point3{x,y,z}
    }

    pub fn make_point_outside(&mut self) -> Point3 {
        let x = 200.0 * self.xmax + 0.1; 
        let y = 300.0 * self.ymax + 0.1;
        let z = 500.0 * self.zmax + 0.1;
        Point3{x,y,z}
    }

}

 
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inside1() {
        let n = 5.0;
        let rng = rand::thread_rng();
        let (xmin, xmax, ymin, ymax, zmin, zmax) = (-n, n, -n, n, -n, n);
        let mut bb = BoundingBox{rng, xmin, xmax, ymin, ymax, zmin, zmax};

        let p = bb.make_point_inside();
        assert!(-n < p.x && p.x < n);
        assert!(-n < p.y && p.y < n);
        assert!(-n < p.z && p.z < n);
        println!("point: {:?}", p);
    }

    #[test]
    fn outside1() {
        let n = 5.0;
        let rng = rand::thread_rng();
        let (xmin, xmax, ymin, ymax, zmin, zmax) = (-n, n, -n, n, -n, n);
        let mut bb = BoundingBox{rng, xmin, xmax, ymin, ymax, zmin, zmax};

        let p = bb.make_point_outside();
        assert!(p.x > n);
        assert!(p.y > n);
        assert!(p.z > n);
        println!("point: {:?}", p);
    }

}
