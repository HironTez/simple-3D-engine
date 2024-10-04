#![forbid(unsafe_code)]

pub struct Camera {
    pub position: Vec<f32>,
    pub rotation: Vec<f32>,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl Camera {
    pub fn new(
        position: Vec<f32>,
        rotation: Vec<f32>,
        fov: f32,
        aspect_ratio: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Self {
        Camera {
            position,
            rotation,
            fov,
            aspect_ratio,
            near_plane,
            far_plane,
        }
    }
}
