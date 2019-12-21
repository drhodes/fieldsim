use rand::Rng;
use std::error::Error;

pub type FieldResult<T> = Result<T, Box<dyn Error>>;

#[derive(Clone, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
    pub zmin: f32,
    pub zmax: f32,
    pub rng: rand::rngs::ThreadRng,
}
