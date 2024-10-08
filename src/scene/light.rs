#![forbid(unsafe_code)]

pub struct Light {
    pub position: Vec<f64>,
    pub brightness: u8,
}

impl Light {
    pub fn new(position: Vec<f64>, brightness: u8) -> Self {
        Light {
            position,
            brightness,
        }
    }
}
