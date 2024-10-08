#![forbid(unsafe_code)]

pub struct Camera {
    pub position: Vec<f64>,
    pub rotation: Vec<f64>,
    pub fov: f64,
    pub aspect_ratio: f64,
    pub near_plane: f64,
    pub far_plane: f64,
}

impl Camera {
    pub fn new(
        position: Vec<f64>,
        rotation: Vec<f64>,
        fov: f64,
        aspect_ratio: f64,
        near_plane: f64,
        far_plane: f64,
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
