#![forbid(unsafe_code)]

pub struct Light {
    pub position: Vec<f32>,
    pub brightness: u8,
}

impl Light {
    pub fn new(position: Vec<f32>, brightness: u8) -> Self {
        Light {
            position,
            brightness,
        }
    }
}
