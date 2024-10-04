#![forbid(unsafe_code)]

pub struct Mesh {
    pub position: Vec<f32>,
    pub rotation: Vec<f32>,
    pub scale: f32,
    pub vertices: Vec<f32>,
    pub indices: Vec<usize>,
    // pub textures: Vec<String>,
    // pub shaders: Vec<String>,
    // pub material: String,
    // pub mesh: String,
}

impl Mesh {
    pub fn new(
        position: Vec<f32>,
        rotation: Vec<f32>,
        scale: f32,
        vertices: Vec<f32>,
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
