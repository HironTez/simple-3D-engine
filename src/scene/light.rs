#![forbid(unsafe_code)]

use crate::tools::vector3::Vector3;

pub struct Light {
    pub position: Vector3<f32>,
    pub brightness: u8,
}

impl Light {
    pub fn new(position: Vector3<f32>, brightness: u8) -> Self {
        Light {
            position,
            brightness,
        }
    }
}
