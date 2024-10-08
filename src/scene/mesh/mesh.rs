#![forbid(unsafe_code)]

pub struct Mesh {
    pub position: Vec<f64>,
    pub rotation: Vec<f64>,
    pub scale: f64,
    pub vertices: Vec<f64>,
    pub indices: Vec<usize>,
    // pub textures: Vec<String>,
    // pub shaders: Vec<String>,
    // pub material: String,
    // pub mesh: String,
}

impl Mesh {
    pub fn new(
        position: Vec<f64>,
        rotation: Vec<f64>,
        scale: f64,
        vertices: Vec<f64>,
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
