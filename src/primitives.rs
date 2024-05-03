#![allow(dead_code, unused_variables)]
use cgmath::vec3;

type VertexType = u16;

pub static CUBE_VERTICES: [cgmath::Vector3<f32>; 8] = [
    vec3(0.5, 0.5, 0.5),
    vec3(-0.5, 0.5, 0.5),
    vec3(0.5, -0.5, 0.5),
    vec3(-0.5, -0.5, 0.5),
    vec3(0.5, 0.5, -0.5),
    vec3(-0.5, 0.5, -0.5),
    vec3(0.5, -0.5, -0.5),
    vec3(-0.5, -0.5, -0.5),
];

// untested
pub static CUBE_INDICES: &[VertexType] = &[
    0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 0, 2, 4, 4, 6, 0, 1, 3, 5, 5, 7, 1, 0, 1, 4, 4, 5, 0, 2, 3,
    6, 6, 7, 2,
];
