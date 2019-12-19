// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]

// //use stl::{BinaryStlFile};
// use stl_io;
// use rand::Rng;
// use tri_mesh;

// struct Obj {
//     filename: String,
//     rand_src: rand::prelude::ThreadRng,
// }

// type Point = [f32; 3];

// /// does the ray from point p intersect the triangle t?
// // fn does_intersect(t: &stl::Triangle, p: Point, ray: Point) -> bool {
// //     let plane_p = t.v1;
// //     //let n = t.normal

// //     // H(p) = d > 0?

// //     return true;
// // }

// impl Obj {
//     pub fn new(filename: String) -> Obj {
//         let msg = format!("Couldn't open model from file: {}", filename.as_str());

//         let mut infile = std::fs::File::open("./models/cube.stl").unwrap();
//         let stl_file = stl_io::read_stl(&mut infile).unwrap();
//         let rand_src = rand::thread_rng();

//         Obj{filename, rand_src}
//     }

//     // https://courses.cs.washington.edu/courses/csep557/10au/lectures/triangle_intersection.pdf
//     ///
//     /// The idea is to drop N points inside the volume.  where each point is
//     /// acts as a point charge q0=N/Q, Q is the total charge of the volume.  A
//     /// charge density function will be passed in and be used to increase the
//     /// charge in those areas.

//     pub fn point_inside(&mut self, p: Point) {
//         //let t = self.stl_file.triangles[0];
//     }

//     fn random_ray() -> Point {
//         [1.,2.,3.]
//     }

//     // Another idea what about integrating the field of arbitrary
//     // polygons? so divide the mesh into a number of face adjacent
//     // polygons, for each of those .. nah.
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn can_read() {
//         let mut infile = std::fs::File::open("./models/cube.stl").unwrap();
//         let _model = stl_io::read_stl(&mut infile).unwrap();
//     }

//     #[test]
//     fn intersect1() {
//         let mut infile = std::fs::File::open("./models/cube.stl").unwrap();
//         ///let mesh = tri_mesh::MeshBuilder::new();
//         let mesh = tri_mesh::mesh::Mesh::new();

//         let model = stl_io::read_stl(&mut infile).unwrap();
//         for face in model.faces {
//         }
//     }
// }
