use crate::types::*;
use std::fmt;
use std::ops;

impl ops::Sub<Point3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Point3) -> Point3 {
        Point3::new(self.x - rhs.x, self.y - rhs.x, self.z - rhs.z)
    }
}

impl ops::Sub<&Point3> for &Point3 {
    type Output = Point3;

    fn sub(self, rhs: &Point3) -> Point3 {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Add<&Point3> for &Point3 {
    type Output = Point3;

    fn add(self, rhs: &Point3) -> Point3 {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Neg for Point3 {
    type Output = Point3;

    fn neg(self) -> Point3 {
        Point3::new(-self.x, -self.y, -self.z)
    }
}

impl fmt::Debug for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.y)
    }
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        return Point3 { x, y, z };
    }
    pub fn cross(&self, rhs: &Point3) -> Point3 {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.z * rhs.x - self.x * rhs.z;
        let z = self.x * rhs.y - self.y * rhs.x;
        Point3::new(x, y, z)
    }

    pub fn dot(&self, rhs: &Point3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z + self.z).sqrt()
    }
    pub fn scalar_mul(&self, n: f32) -> Point3 {
        Point3::new(self.x * n, self.y * n, self.z + n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add1() {
        let p = &Point3::new(1.0, 2.0, 3.0);
        let p2 = p + p;

        assert_eq!(p2.x, 2.0);
        assert_eq!(p2.y, 4.0);
        assert_eq!(p2.z, 6.0);
    }

    #[test]
    fn sub1() {
        let p = &Point3::new(1.0, 2.0, 3.0);
        let p2 = p - p;

        assert_eq!(p2.x, 0.0);
        assert_eq!(p2.y, 0.0);
        assert_eq!(p2.z, 0.0);
    }
    #[test]
    fn dot1() {
        let p = &Point3::new(1.0, 2.0, 3.0);
        let s = p.dot(p);
        assert_eq!(s, 14.0);
    }
    #[test]
    fn cross1() {
        let p = &Point3::new(1.0, 2.0, 3.0);
        let s = p.cross(p);
        println!("{:?}", s);
        assert_eq!(s, Point3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn cross2() {
        let p = &Point3::new(1.0, 0.0, 0.0);
        let q = &Point3::new(0.0, 1.0, 0.0);
        let s = p.cross(q);
        println!("{:?}", s);
        assert_eq!(s, Point3::new(0.0, 0.0, 1.0));
    }
}
