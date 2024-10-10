#![forbid(unsafe_code)]

use crate::tools::vector3::Vector3;

pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub fov: f32,
    pub aspect_ratio: Option<f32>,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl Camera {
    pub fn new(
        position: Vector3<f32>,
        rotation: Vector3<f32>,
        fov: f32,
        aspect_ratio: Option<f32>,
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
