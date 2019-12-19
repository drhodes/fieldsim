use std::error::Error;

pub type FieldResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 { x, y, z }
    }
}

pub struct Segment {
    pub p1: Point3,
    pub p2: Point3,
}

pub struct Face {
    pub verts: [Point3; 3],
}

#[derive(Clone, PartialEq)]
pub enum Sign {
    Zero,
    Pos,
    Neg,
}
pub use Sign::*;

pub struct Obj {
    pub faces: Vec<Face>,
}

pub struct OffLoader {
    pub verts: Vec<Point3>,
    pub faces: Vec<Face>,
}
