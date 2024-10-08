#![forbid(unsafe_code)]

use super::scene::Scene;

pub struct Renderer {}

impl Renderer {
    pub fn render(scene: &Scene, camera_index: usize, buffer: &mut [u32], width: u32, height: u32) {
        let camera = scene
            .cameras
            .get(camera_index)
            .expect("Couldn't find the specified camera!");

        for mesh in scene.meshes.as_slice() {
            let vertices_count = mesh.vertices.len() as f64 / 3.0;
            // for (let i = 0; i <)
        }

        // for index in 0..(width * height) {
        //     let y = index / width;
        //     let x = index % width;
        //     let red = x % 255;
        //     let green = y % 255;
        //     let blue = (x * y) % 255;

        //     buffer[index as usize] = blue | (green << 8) | (red << 16);
        // }
    }
}

fn multiply_matrices(
    matrix_1: &Option<[f64; 16]>,
    matrix_2: &Option<[f64; 16]>,
) -> Option<[f64; 16]> {
    if let (Some(matrix_1), Some(matrix_2)) = (matrix_1, matrix_2) {
        let mut result = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i * 4 + j] += matrix_1[i * 4 + k] * matrix_2[k * 4 + j];
                }
            }
        }
        return Some(result);
    }

    if let Some(matrix_1) = matrix_1 {
        return Some(matrix_1.clone());
    } else if let Some(matrix_2) = matrix_2 {
        return Some(matrix_2.clone());
    }

    None
}

fn create_translation_matrix(x: f64, y: f64, z: f64) -> Option<[f64; 16]> {
    if x == 0.0 && y == 0.0 && z == 0.0 {
        return None;
    }

    // Translation matrix
    Some([
        1.0, 0.0, 0.0, x, //   x
        0.0, 1.0, 0.0, y, //   y
        0.0, 0.0, 1.0, z, //   z
        0.0, 0.0, 0.0, 1.0, // w
    ])
}

fn create_x_rotation_matrix(x_rotation: f64) -> Option<[f64; 16]> {
    if x_rotation == 0.0 {
        return None;
    }

    let sin = x_rotation.to_radians().sin();
    let cos = x_rotation.to_radians().cos();

    // Rotation matrix
    Some([
        1.0, 0.0, 0.0, 0.0, //  x
        0.0, cos, -sin, 0.0, // y
        0.0, sin, cos, 0.0, //  z
        0.0, 0.0, 0.0, 1.0, //  w
    ])
}
fn create_y_rotation_matrix(y_rotation: f64) -> Option<[f64; 16]> {
    if y_rotation == 0.0 {
        return None;
    }

    let sin = y_rotation.to_radians().sin();
    let cos = y_rotation.to_radians().cos();

    // Rotation matrix
    Some([
        cos, 0.0, sin, 0.0, //  x
        0.0, 1.0, 0.0, 0.0, //  y
        -sin, 0.0, cos, 0.0, // z
        0.0, 0.0, 0.0, 1.0, //  w
    ])
}
fn create_z_rotation_matrix(z_rotation: f64) -> Option<[f64; 16]> {
    if z_rotation == 0.0 {
        return None;
    }

    let sin = z_rotation.to_radians().sin();
    let cos = z_rotation.to_radians().cos();

    // Rotation matrix
    Some([
        cos, -sin, 0.0, 0.0, // x
        sin, cos, 0.0, 0.0, //  y
        0.0, 0.0, 1.0, 0.0, //  z
        0.0, 0.0, 0.0, 1.0, //  w
    ])
}
fn create_scale_matrix(scale: f64) -> Option<[f64; 16]> {
    if scale == 1.0 {
        return None;
    }

    // Scale matrix
    Some([
        scale, 0.0, 0.0, 0.0, // x
        0.0, scale, 0.0, 0.0, // y
        0.0, 0.0, scale, 0.0, // z
        0.0, 0.0, 0.0, 1.0, //   w
    ])
}

fn create_rotation_matrix(x_rotation: f64, y_rotation: f64, z_rotation: f64) -> Option<[f64; 16]> {
    if x_rotation == 0.0 && y_rotation == 0.0 && z_rotation == 0.0 {
        return None;
    }

    let x_rotation_matrix = create_x_rotation_matrix(x_rotation);
    let y_rotation_matrix = create_y_rotation_matrix(y_rotation);
    let z_rotation_matrix = create_z_rotation_matrix(z_rotation);

    let mut rotation_matrix: Option<[f64; 16]>;
    rotation_matrix = multiply_matrices(&x_rotation_matrix, &y_rotation_matrix);
    rotation_matrix = multiply_matrices(&rotation_matrix, &z_rotation_matrix);
    rotation_matrix
}

fn create_model_matrix(
    x: f64,
    y: f64,
    z: f64,
    x_rotation: f64,
    y_rotation: f64,
    z_rotation: f64,
    scale: f64,
) -> Option<[f64; 16]> {
    let translation_matrix = create_translation_matrix(x, y, z);
    let rotation_matrix = create_rotation_matrix(x_rotation, y_rotation, z_rotation);
    let scale_matrix = create_scale_matrix(scale);

    let mut model_matrix: Option<[f64; 16]>;
    model_matrix = multiply_matrices(&translation_matrix, &rotation_matrix);
    model_matrix = multiply_matrices(&model_matrix, &scale_matrix);
    model_matrix
}
