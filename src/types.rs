use rand::Rng;
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

#[derive(Debug)]
pub struct Segment {
    pub p1: Point3,
    pub p2: Point3,
}

#[derive(Debug)]
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
    pub bbox: BoundingBox,
    pub faces: Vec<Face>,
}

pub struct OffLoader {
    pub verts: Vec<Point3>,
    pub faces: Vec<Face>,
}

pub struct BoundingBox {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
    pub rng: rand::rngs::ThreadRng,
}
