#![forbid(unsafe_code)]

use super::camera::Camera;
use super::light::Light;
use super::mesh::mesh::Mesh;

pub struct Scene {
    pub cameras: Vec<Camera>,
    pub lights: Vec<Light>,
    pub meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new(cameras: Vec<Camera>, lights: Vec<Light>, meshes: Vec<Mesh>) -> Self {
        Scene {
            cameras,
            lights,
            meshes,
        }
    }
}
