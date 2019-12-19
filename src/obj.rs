use crate::types::*;
use std::error::Error;
use std::fs;
use std::path::Path;

impl Obj {
    pub fn from_file(filename: &Path) -> FieldResult<Obj> {
        let loader = OffLoader::from_file(filename)?;
        Ok(Obj {
            faces: loader.faces,
        })
    }

    pub fn point_inside(&self, p: &Point3) -> bool {
        // consider 3-space hash to reduce the number of checked faces.
        // it's easier to implement than OctTree with similar performance.
        //
    }
}