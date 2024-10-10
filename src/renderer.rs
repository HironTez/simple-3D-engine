#![forbid(unsafe_code)]

use super::scene::Scene;
use crate::tools::vector3::Vector3;

pub struct Renderer {}

impl Renderer {
    pub fn render(scene: &Scene, camera_index: usize, buffer: &mut [u32], width: u32, height: u32) {
        buffer.fill(0); // Clear buffer

        let camera = scene
            .cameras
            .get(camera_index)
            .expect("Couldn't find the specified camera!");
        let aspect_ration = camera.aspect_ratio.unwrap_or(width as f32 / height as f32);

        let mut normalized_device_coordinates: Vec<Vector3<f32>> = Vec::new();

        // Create camera relative matrices
        let camera_view_matrix = create_view_matrix(&camera.position, &camera.rotation);
        let projection_matrix = create_perspective_projection_matrix(
            camera.fov,
            aspect_ration,
            camera.near_plane,
            camera.far_plane,
        );
        let perspective_matrix = multiply_matrices(&camera_view_matrix, &Some(projection_matrix));

        for mesh in scene.meshes.as_slice() {
            // Create mesh relative matrix
            let mesh_model_matrix = create_model_matrix(&mesh.position, &mesh.rotation, mesh.scale);
            let transformation_matrix = multiply_matrices(&mesh_model_matrix, &perspective_matrix);

            let clip_space_positions =
                apply_matrix_to_vertices(&transformation_matrix, &mesh.vertices);
            normalized_device_coordinates.extend(clip_space_positions);
        }

        for ndc in &normalized_device_coordinates {
            // TODO: render faces instead of vertices
            let screen_space =
                ndc_to_screen_space(&ndc, width, height, camera.near_plane, camera.far_plane);
            let pixel_index = (screen_space.y as u32 * width) + screen_space.x as u32;
            if (pixel_index as usize) < buffer.len() {
                buffer[pixel_index as usize] = 0xFF_FF_FF; // TODO: replace with actual color calculation
            }
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

fn ndc_to_screen_space(
    ndc: &Vector3<f32>,
    width: u32,
    height: u32,
    near_plane: f32,
    far_plane: f32,
) -> Vector3<f32> {
    let screen_x = (ndc.x + 1.0) / 2.0 * width as f32;
    let screen_y = (1.0 - ndc.y) / 2.0 * height as f32;
    // (depth = (ndc_position.z + 1) * (far-near) / 2 + near)
    let screen_z = (far_plane - near_plane) * ndc.z / 2.0 + (far_plane + near_plane) / 2.0;
    Vector3::new(screen_x, screen_y, screen_z)
}

fn apply_matrix_to_vertices(
    matrix: &Option<[f32; 16]>,
    vertices: &Vec<Vector3<f32>>,
) -> Vec<Vector3<f32>> {
    let mut transformed_vertices = Vec::new();
    for i in 0..vertices.len() {
        let vertex = &vertices[i];
        let transformed_vertex = apply_matrix_to_vertex(&matrix, &vertex);
        transformed_vertices.push(transformed_vertex);
    }

    transformed_vertices
}

pub fn apply_matrix_to_vertex(matrix: &Option<[f32; 16]>, position: &Vector3<f32>) -> Vector3<f32> {
    let vertex_matrix = [position.x, position.y, position.z, 1.0]; // TODO: support for vectors and faces (0 instead of 1)
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    let mut w = 0.0;

    if let Some(matrix) = matrix {
        for i in 0..4 {
            x += matrix[i] * vertex_matrix[i];
            y += matrix[i + 4] * vertex_matrix[i];
            z += matrix[i + 8] * vertex_matrix[i];
            w += matrix[i + 12] * vertex_matrix[i];
        }

        // TODO: conditional check if a coordinate is inside the clip space (-w <= x && x <= w && -w <= y && y <= w && -w <= z && z <= w)
        // Should be done before the perspective division

        let normalized_vector = divide_perspective(x, y, z, w);
        return normalized_vector;
    }

    position.clone()
}

fn divide_perspective(x: f32, y: f32, z: f32, w: f32) -> Vector3<f32> {
    if w == 0.0 || w == 1.0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    let new_x = x / w;
    let new_y = y / w;
    let new_z = z / w;

    Vector3::new(new_x, new_y, new_z)
}

pub fn multiply_matrices(
    matrix_1: &Option<[f32; 16]>,
    matrix_2: &Option<[f32; 16]>,
) -> Option<[f32; 16]> {
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

    // Return the non-None matrix if one of them is None
    if let Some(matrix_1) = matrix_1 {
        return Some(matrix_1.clone());
    } else if let Some(matrix_2) = matrix_2 {
        return Some(matrix_2.clone());
    }

    None
}

fn create_translation_matrix(x: f32, y: f32, z: f32) -> Option<[f32; 16]> {
    if x == 0.0 && y == 0.0 && z == 0.0 {
        return None;
    }

    // Translation matrix
    Some([
        // x, y, z, w
        1.0, 0.0, 0.0, 0.0, // x
        0.0, 1.0, 0.0, 0.0, // y
        0.0, 0.0, 1.0, 0.0, // z
        x, y, z, 1.0, //       w
    ])
}

fn create_x_rotation_matrix(x_rotation: f32) -> Option<[f32; 16]> {
    if x_rotation == 0.0 {
        return None;
    }

    let sin = x_rotation.to_radians().sin();
    let cos = x_rotation.to_radians().cos();

    // Rotation matrix
    Some([
        // x, y, z, w
        1.0, 0.0, 0.0, 0.0, //  x
        0.0, cos, sin, 0.0, //  y
        0.0, -sin, cos, 0.0, // z
        0.0, 0.0, 0.0, 1.0, //  w
    ])
}
fn create_y_rotation_matrix(y_rotation: f32) -> Option<[f32; 16]> {
    if y_rotation == 0.0 {
        return None;
    }

    let sin = y_rotation.to_radians().sin();
    let cos = y_rotation.to_radians().cos();

    // Rotation matrix
    Some([
        // x, y, z, w
        cos, 0.0, -sin, 0.0, // x
        0.0, 1.0, 0.0, 0.0, //  y
        sin, 0.0, cos, 0.0, //  z
        0.0, 0.0, 0.0, 1.0, //  w
    ])
}
fn create_z_rotation_matrix(z_rotation: f32) -> Option<[f32; 16]> {
    if z_rotation == 0.0 {
        return None;
    }

    let sin = z_rotation.to_radians().sin();
    let cos = z_rotation.to_radians().cos();

    // Rotation matrix
    Some([
        // x, y, z, w
        cos, sin, 0.0, 0.0, //  x
        -sin, cos, 0.0, 0.0, // y
        0.0, 0.0, 1.0, 0.0, //  z
        0.0, 0.0, 0.0, 1.0, //  w
    ])
}
fn create_scale_matrix(scale: f32) -> Option<[f32; 16]> {
    if scale == 1.0 {
        return None;
    }

    // Scale matrix
    Some([
        // x, y, z, w
        scale, 0.0, 0.0, 0.0, // x
        0.0, scale, 0.0, 0.0, // y
        0.0, 0.0, scale, 0.0, // z
        0.0, 0.0, 0.0, 1.0, //   w
    ])
}

fn create_rotation_matrix(x_rotation: f32, y_rotation: f32, z_rotation: f32) -> Option<[f32; 16]> {
    if x_rotation == 0.0 && y_rotation == 0.0 && z_rotation == 0.0 {
        return None;
    }

    let x_rotation_matrix = create_x_rotation_matrix(x_rotation);
    let y_rotation_matrix = create_y_rotation_matrix(y_rotation);
    let z_rotation_matrix = create_z_rotation_matrix(z_rotation);

    let mut rotation_matrix: Option<[f32; 16]>;
    rotation_matrix = multiply_matrices(&x_rotation_matrix, &y_rotation_matrix);
    rotation_matrix = multiply_matrices(&rotation_matrix, &z_rotation_matrix);
    rotation_matrix
}

pub fn create_model_matrix(
    position: &Vector3<f32>,
    rotation: &Vector3<f32>,
    scale: f32,
) -> Option<[f32; 16]> {
    let translation_matrix = create_translation_matrix(position.x, position.y, position.z);
    let rotation_matrix = create_rotation_matrix(rotation.x, rotation.y, rotation.z);
    let scale_matrix = create_scale_matrix(scale);

    let mut model_matrix: Option<[f32; 16]>;
    model_matrix = multiply_matrices(&translation_matrix, &rotation_matrix);
    model_matrix = multiply_matrices(&model_matrix, &scale_matrix);
    model_matrix
}

pub fn create_view_matrix(
    camera_position: &Vector3<f32>,
    camera_rotation: &Vector3<f32>,
) -> Option<[f32; 16]> {
    let translation_matrix =
        create_translation_matrix(-camera_position.x, -camera_position.y, -camera_position.z);
    let rotation_matrix =
        create_rotation_matrix(-camera_rotation.x, -camera_rotation.y, -camera_rotation.z);

    let model_matrix = multiply_matrices(&translation_matrix, &rotation_matrix);
    model_matrix
}

pub fn create_perspective_projection_matrix(
    fov: f32,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
) -> [f32; 16] {
    let fov_tan = (fov / 2.0).to_radians().tan();
    let a = 1.0 / (aspect_ratio * fov_tan);
    let b = 1.0 / fov_tan;
    let c = -(far_plane + near_plane) / (far_plane - near_plane);
    let d = -(2.0 * far_plane * near_plane) / (far_plane - near_plane);

    [
        // x, y, z, w
        a, 0.0, 0.0, 0.0, //    x
        0.0, b, 0.0, 0.0, //    y
        0.0, 0.0, c, d, //      z
        0.0, 0.0, -1.0, 0.0, // w
    ]
}
