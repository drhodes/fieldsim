// // J.J. Jiménez et al. / Computational Geometry 43 (2010) 474–492
// // page 480
// pub fn jimenez(&self, seg: &Segment) -> bool {
//     let [v1, v2, v3] = &self.verts;
//     let (q1, q2) = (&seg.p1, &seg.p2);

//     let eps = 1e-6;

//     let a = q1 - v3;
//     let b = v1 - v3;
//     let c = v2 - v3;
//     let w1 = b.cross(&c);
//     let w = a.dot(&w1);
//     let d = q2 - v3;
//     let s = d.dot(&w1);

//     if w > eps {
//         if s > eps {
//             return false;
//         }
//         let w2 = a.cross(&d);
//         let t = w2.dot(&c);
//         if t < -eps {
//             return false;
//         }
//         let u = -(w2.dot(&b));
//         if u < -eps {
//             return false;
//         }
//         if w < (s + t + u) {
//             return false;
//         }
//     } else if w < -eps {
//         if s < -eps {
//             return false;
//         }
//         let w2 = a.cross(&d);
//         let t = w2.dot(&c);
//         if t < eps {
//             return false;
//         }
//         let u = -w2.dot(&b);
//         if u > eps {
//             return false;
//         }
//         if w > (s + t + u) {
//             return false;
//         }
//     } else {
//         if s > eps {
//             let w2 = d.cross(&a);
//             let t = w2.dot(&c);
//             if t < -eps {
//                 return false;
//             }
//             let u = -w2.dot(&b);
//             if u < -eps {
//                 return false;
//             }
//             if -s < (t + u) {
//                 return false;
//             }
//         } else if s < -eps {
//             let w2 = d.cross(&a);
//             let t = w2.dot(&c);
//             if t > eps {
//                 return false;
//             }
//             let u = -w2.dot(&b);
//             if u > eps {
//                 return false;
//             }
//             if -s > (t + u) {
//                 return false;
//             }
//         } else {
//             return false;
//         }
//     }
//     return true;
// }

// pub fn moller_trumbore(&self, seg: &Segment) -> bool {
//     const EPSILON: f64 = 1e-8;
//     let [v0, v1, v2] = &self.verts;
//     let (ray_origin, ray_end) = (&seg.p1, &seg.p2);
//     let ray = ray_end - ray_origin;

//     let edge1 = v1 - v2;
//     let edge2 = v2 - v0;
//     let h = ray.cross(&edge2);
//     let a = edge1.dot(&h);

//     if a > -EPSILON || a < EPSILON {
//         return false;
//     }

//     let f = 1.0 / a;
//     let s = ray_origin - v0;
//     let u = f * s.dot(&h);
//     if u < 0.0 || u > 1.0 {
//         return false;
//     }

//     let q = s.cross(&edge1);
//     let v = f * ray.dot(&q);
//     let t = f * edge2.dot(&q);

//     return t > EPSILON && t < 1.0 / EPSILON;
// }

// pub fn signed_volume(a: &Point3, b: &Point3, c: &Point3, d: &Point3) -> f64 {
//     let r = (b - a).cross(&(c - a)).dot(&(d - a));
//     r / 6.0
// }

// // Ported to rust from
// // http://geomalgorithms.com/a06-_intersect-2.html#intersect3D_RayTriangle()
// //
// // Derived from
// // Copyright 2001 softSurfer, 2012 Dan Sunday
// // This code may be freely used and modified for any purpose
// // providing that this copyright notice is included with it.
// // SoftSurfer makes no warranty for this code, and cannot be held
// // liable for any real or imagined damage resulting from its use.
// // Users of this code must verify correctness for their application.
// pub fn intersect(&self, seg: &Segment) -> bool {
//     // get triangle edge vectors and plane normal
//     let [v0, v1, v2] = &self.verts;
//     let (p1, p0) = (&seg.p1, &seg.p2);
//     const SMALL_NUM: f64 = 1e-8;

//     let u = v1 - v0;
//     let v = v2 - v0;
//     let n = u.cross(&v); // cross product
//     if n.len() == 0.0 {
//         // triangle is degenerate
//         return false; // do not deal with this case
//     }

//     let dir = p1 - p0; // ray direction vector
//     let w0 = p0 - v0;
//     let a = -n.dot(&w0);
//     let b = n.dot(&dir);
//     if b.abs() < SMALL_NUM {
//         // ray is  parallel to triangle plane
//         if a == 0.0 {
//             // ray lies in triangle plane
//             return false;
//         } else {
//             return false; // ray disjoint from plane
//         }
//     }

//     // get intersect point of ray with triangle plane
//     let r = a / b;
//     if r < 0.0 {
//         // ray goes away from triangle
//         return false; // => no intersect
//     }
//     // for a segment, also test if (r > 1.0) => no intersect

//     let i = p0 + &dir.scalar_mul(r); // intersect point of ray and plane

//     // is I inside T?
//     // float    uu, uv, vv, wu, wv, D;
//     let uu = u.dot(&u);
//     let uv = u.dot(&v);
//     let vv = v.dot(&v);
//     let w = i - v0.clone();
//     let wu = w.dot(&u);
//     let wv = w.dot(&v);
//     let d = uv * uv - uu * vv;

//     // get and test parametric coords
//     let s = (uv * wv - vv * wu) / d;
//     if s < 0.0 || s > 1.0 {
//         // I is outside T
//         return false;
//     }
//     let t = (uv * wu - uu * wv) / d;
//     if t < 0.0 || (s + t) > 1.0 {
//         // I is outside T
//         false;
//     }
//     return true; // I is in T
// }

//     // Segment Triangle Intersection
//     // https://stackoverflow.com/questions/53962225/
//     // pg 237, Computational Geometry in C
//     pub fn intersect(&self, seg: &Segment) -> bool {
//         let [a, b, c] = &self.verts;
//         let (q, r) = (&seg.p1, &seg.p2);

//         let vol0 = Face::volume_sign(q, a, b, r);
//         let vol1 = Face::volume_sign(q, b, c, r);
//         let vol2 = Face::volume_sign(q, c, a, r);

//         let cond1 = vol0 == Neg && vol1 == Neg && vol2 == Neg;
//         let cond2 = vol0 == Pos && vol1 == Pos && vol2 == Pos;
//         if cond1 || cond2 {
//             return true;
//         }

//         let cond3 = vol0 == Neg || vol1 == Neg || vol2 == Neg;
//         let cond4 = vol0 == Pos || vol1 == Pos || vol2 == Pos;
//         if cond3 && cond4 {
//             return false;
//         } else if (vol0 == Zero && vol1 == Zero)
//             || (vol0 == Zero && vol2 == Zero)
//             || (vol1 == Zero && vol2 == Zero)
//         {
//             return true;
//         } else if vol0 == Zero || vol1 == Zero || vol2 == Zero {
//             return true;
//         } else {
//             return false;
//         }

//         // ignores degenerate forms: segment in plane of triangle, segment intersects points.
//     }

//     // pg 131, Computational Geometry in C
//     fn volume_sign(a: &Point3, b: &Point3, c: &Point3, d: &Point3) -> Sign {
//         let ax = a.x;
//         let ay = a.y;
//         let az = a.z;
//         let bx = b.x;
//         let by = b.y;
//         let bz = b.z;
//         let cx = c.x;
//         let cy = c.y;
//         let cz = c.z;
//         let dx = d.x;
//         let dy = d.y;
//         let dz = d.z;

//         let bxdx = bx - dx;
//         let bydy = by - dy;
//         let bzdz = bz - dz;
//         let cxdx = cx - dx;
//         let cydy = cy - dy;
//         let czdz = cz - dz;
//         let vol = (az - dz) * (bxdx * cydy - bydy * cxdx)
//             + (ay - dy) * (bzdz * cxdx - bxdx * czdz)
//             + (ax - dx) * (bydy * czdz - bzdz * cydy);

//         let eps = 1e-10;

//         if vol > eps {
//             Pos
//         } else if vol < -eps {
//             Neg
//         } else {
//             Zero
//         }
//     }
// }

// pub fn intersect(&self, seg: &Segment) -> bool {
//     let [v1, v2, v3] = &self.verts;
//     let (q1, q2) = (&seg.p1, &seg.p2);
//     let eps = 1e-10;

//     let a = q1 - v3;
//     let b = v1 - v3;
//     let c = v2 - v3;
//     let w1 = b.cross(&c);
//     let w = a.dot(&w1);
//     if w > eps {
//         let d = q2 - v3;
//         let s = d.dot(&w1);
//         if s > eps {
//             return false;
//         }
//         let w2 = a.cross(&d);
//         let t = w2.dot(&c);
//         if t < -eps {
//             return false;
//         }
//         let u = -w2.dot(&b);
//         if u < -eps {
//             return false;
//         }
//         if w < (s + t + u) {
//             return false;
//         }
//     } else if w < -eps {
//         return false;
//     } else {
//         let d = q2 - v3;
//         let s = d.dot(&w1);
//         if s > eps {
//             return false;
//         } else if s < -eps {
//             let w2 = d.cross(&a);
//             let t = w2.dot(&c);
//             if t > eps {
//                 return false;
//             }
//             let u = -w2.dot(&b);
//             if u > eps {
//                 return false;
//             }
//             if -s > t + u {
//                 return false;
//             }
//         } else {
//             return false;
//         }
//     }
//     return true;
// }
