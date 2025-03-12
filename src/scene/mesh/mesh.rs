#![forbid(unsafe_code)]

use crate::tools::vector3::Vector3;

pub struct Mesh {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: f32,
    pub vertices: Vec<Vector3<f32>>,
    pub indices: Vec<usize>,
    // pub textures: Vec<String>,
    // pub shaders: Vec<String>,
    // pub material: String,
    // pub mesh: String,
}

impl Mesh {
    pub fn new(
        position: Vector3<f32>,
        rotation: Vector3<f32>,
        scale: f32,
        vertices: Vec<Vector3<f32>>,
        indices: Vec<usize>,
    ) -> Self {
        Mesh {
            position,
            rotation,
            scale,
            vertices,
            indices,
        }
    }
}
