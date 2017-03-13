// use vector::Vec3;
// use triangle::Triangles;
//
// pub fn linearTessellation<T>(u: &T, v: &T, t: &Triangle<T>, vertices: &[Vertex]) -> Vec3<T> {
//     let w = 1 - (u + v);
//     (vertices[t.indices[0]].pos * u) + (vertices[t.indices[1]].pos * v) + (vertices[t.indices[2]].pos * w)
// }
//
// pub fn projectOnPlane<T>(p: &Vec3<T>, pop: &Vec3<T>, nop: &Vec3<T>) -> Vec3<T> {
//     p + (nop.dot(pop - p) * nop)
// }
//
// pub fn curveTesselation<T>(u: &T, v: &T, t: &Triangle<T>, vertices: &[Vertex]) -> Vec3<T> {
//     let w = 1 - (u + v);
//     let p = (vertices[t.indices[0]].pos * u) + (vertices[t.indices[1]].pos * v) + (vertices[t.indices[2]].pos * w);
//     let c0 = projectOnPlane(p, vertices[t.indices[0]].pos, vertices[t.indices[0]].normal);
//     let c1 = projectOnPlane(p, vertices[t.indices[1]].pos, vertices[t.indices[1]].normal);
//     let c2 = projectOnPlane(p, vertices[t.indices[2]].pos, vertices[t.indices[2]].normal);
//     c0 * u + c1 * v + c2 * w
// }
//
// pub fn curveTesselation<T>(u: &T, v: &T, t: &Triangle<T>, vertices: &[Vertex]) -> Vec3<T> {
//     let w = 1 - (u + v);
//     let p = (vertices[t.indices[0]].pos * u) + (vertices[t.indices[1]].pos * v) + (vertices[t.indices[2]].pos * w);
//     let c0 = projectOnPlane(p, vertices[t.indices[0]].pos, vertices[t.indices[0]].normal);
//     let c1 = projectOnPlane(p, vertices[t.indices[1]].pos, vertices[t.indices[1]].normal);
//     let c2 = projectOnPlane(p, vertices[t.indices[2]].pos, vertices[t.indices[2]].normal);
//     c0 * u + c1 * v + c2 * w
// }
//
// pub fn curveWithAlphaWeightTesselation<T>(alpha: &T, u: &T, v: &T, t: &Triangle<T>, vertices: &[Vertex]) -> Vec3<T> {
//     let w = 1 - (u + v);
//     let p = (vertices[t.indices[0]].pos * u) + (vertices[t.indices[1]].pos * v) + (vertices[t.indices[2]].pos * w);
//     let c0 = projectOnPlane(p, vertices[t.indices[0]].pos, vertices[t.indices[0]].normal);
//     let c1 = projectOnPlane(p, vertices[t.indices[1]].pos, vertices[t.indices[1]].normal);
//     let c2 = projectOnPlane(p, vertices[t.indices[2]].pos, vertices[t.indices[2]].normal);
//     let c = c0 * u + c1 * v + c2 * w
//     p * (1.0 - alpha) + c * alpha
// }
//
// pub fn does_intersect<T>(t: &Triangle<T>, r: Ray3<T>, v: &[Vertex]) -> bool {
//     let p0_p2 = v[t.indices[0]].pos - v[t.indices[2]].pos;
//     let p1_p2 = v[t.indices[1]].pos - v[t.indices[2]].pos;
//     let p0_p2_n0_n0 = p0_p2.dot(v[t.indices[0]].nrm) * v[t.indices[0]].nrm;
//     let p0_p2_n1_n1 = p0_p2.dot(v[t.indices[1]].nrm) * v[t.indices[1]].nrm;
//     let p0_p2_n2_n2 = p0_p2.dot(v[t.indices[2]].nrm) * v[t.indices[2]].nrm;
//     let p1_p2_n0_n0 = p1_p2.dot(v[t.indices[0]].nrm) * v[t.indices[0]].nrm;
//     let p1_p2_n1_n1 = p1_p2.dot(v[t.indices[1]].nrm) * v[t.indices[1]].nrm;
//     let p1_p2_n2_n2 = p1_p2.dot(v[t.indices[2]].nrm) * v[t.indices[2]].nrm;
//     let d = p0_p2_n2_n2 - p0_p2_n0_n0;
//     let c = p1_p2_n2_n2 - p0_p2_n1_n1;
//     let e = p1_p2_n2_n2 + p0_p2_n2_n2 - (p1_p2_n0_n0 + p1_p2_n1_n1);
//     const POS_EPSILON: T = 0.000001
//     const NEG_EPSILON: T = -0.000001
//     if r.d.x < POS_EPSILON && r.d.x > NEG_EPSILON {
//         if d.x < POS_EPSILON && d.x > NEG_EPSILON {
//             P
//         } else {
//
//         }
//     } else if r.d.y < POS_EPSILON && r.d.y > NEG_EPSILON {
//
//     } else {
//
//     }
// }
