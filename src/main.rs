#![forbid(unsafe_code)]

mod renderer;
mod scene;
mod tools;
mod window;

use renderer::{
    apply_matrix_to_vertex, create_model_matrix, create_perspective_projection_matrix,
    create_view_matrix, multiply_matrices, Renderer,
};
use scene::camera::Camera;
use scene::light::Light;
use scene::mesh::Mesh;
use scene::Scene;
use tools::vector3::Vector3;
use window::Window;

fn main() {
    // let vertex = Vector3::new(-1.0, -1.0, -1.0);
    // let vertex2 = Vector3::new(1.0, 1.0, 1.0);
    // println!("vertex x: {}, y: {}, z: {}", vertex.x, vertex.y, vertex.z);
    // println!(
    //     "vertex2 x: {}, y: {}, z: {}",
    //     vertex2.x, vertex2.y, vertex2.z
    // );

    // let view_matrix: Option<[f32; 16]> =
    //     create_view_matrix(&Vector3::new(0.0, 0.0, -10.0), &Vector3::new(0.0, 0.0, 0.0));

    // println!("view_matrix");
    // for i in 0..4 {
    //     for j in 0..4 {
    //         print!(
    //             "{}, ",
    //             view_matrix.expect("view_matrix has not been created")[i * 4 + j]
    //         );
    //     }
    //     println!();
    // }

    // let projection_matrix = create_perspective_projection_matrix(90.0, 1.0, 0.1, 100.0);

    // println!("projection matrix");
    // for i in 0..4 {
    //     for j in 0..4 {
    //         print!("{}, ", projection_matrix[i * 4 + j]);
    //     }
    //     println!();
    // }

    // let model_matrix = create_model_matrix(
    //     &Vector3::new(0.0, 0.0, 0.0),
    //     &Vector3::new(0.0, 0.0, 0.0),
    //     2.0,
    // );

    // println!("model matrix");
    // for i in 0..4 {
    //     for j in 0..4 {
    //         print!(
    //             "{}, ",
    //             model_matrix.expect("model matrix has not been created")[i * 4 + j]
    //         );
    //     }
    //     println!();
    // }

    // let final_transform_matrix = multiply_matrices(
    //     &model_matrix,
    //     &multiply_matrices(&view_matrix, &Some(projection_matrix)),
    // );

    // println!("final matrix");
    // for i in 0..4 {
    //     for j in 0..4 {
    //         print!(
    //             "{}, ",
    //             final_transform_matrix.expect("final has not been created")[i * 4 + j]
    //         );
    //     }
    //     println!();
    // }

    // let vertex_camera_space = apply_matrix_to_vertex(&final_transform_matrix, &vertex);
    // println!(
    //     "vertex after x: {}, y: {}, z: {}",
    //     vertex_camera_space.x, vertex_camera_space.y, vertex_camera_space.z
    // );
    // let vertex2_camera_space = apply_matrix_to_vertex(&final_transform_matrix, &vertex2);
    // println!(
    //     "vertex2 after x: {}, y: {}, z: {}",
    //     vertex2_camera_space.x, vertex2_camera_space.y, vertex2_camera_space.z
    // );

    let cameras = vec![Camera::new(
        Vector3::new(0.0, 0.0, -10.0), // Position
        Vector3::new(0.0, 0.0, 0.0),   // Rotation
        90.0,                          // FOV
        None,                          // Aspect Ratio
        0.1,                           // Near Plane
        100.0,                         // Far Plane
    )];
    let lights = vec![Light::new(Vector3::new(0.0, 100.0, 50.0), 100)];
    let meshes = vec![
        // Cube
        Mesh::new(
            Vector3::new(0.0, 0.0, 0.0), // Position
            Vector3::new(0.0, 0.0, 0.0), // Rotation
            1.0,                         // Scale
            vec![
                Vector3::new(-1.0, -1.0, -1.0), // Vertex 0: Bottom-left-back
                Vector3::new(1.0, -1.0, -1.0),  // Vertex 1: Bottom-right-back
                Vector3::new(1.0, 1.0, -1.0),   // Vertex 2: Top-right-back
                Vector3::new(-1.0, 1.0, -1.0),  // Vertex 3: Top-left-back
                Vector3::new(-1.0, -1.0, 1.0),  // Vertex 4: Bottom-left-front
                Vector3::new(1.0, -1.0, 1.0),   // Vertex 5: Bottom-right-front
                Vector3::new(1.0, 1.0, 1.0),    // Vertex 6: Top-right-front
                Vector3::new(-1.0, 1.0, 1.0),   // Vertex 7: Top-left-front
            ],
            vec![
                0, 1, 2, 2, 3, 0, // Back face
                4, 5, 6, 6, 7, 4, // Front face
                0, 4, 7, 7, 3, 0, // Left face
                1, 5, 6, 6, 2, 1, // Right face
                3, 2, 6, 6, 7, 3, // Top face
                0, 1, 5, 5, 4, 0, // Bottom face
            ],
        ),
    ];

    let scene = Scene::new(cameras, lights, meshes);

    let render_scene = |buffer: &mut [u32], width: u32, height: u32| {
        Renderer::render(&scene, 0, buffer, width, height);
    };

    let mut window = Window::new(
        &|buffer, width, height| {
            render_scene(buffer, width, height);
        },
        &|request_redraw| {
            // Queue a RedrawRequested event.
            //
            // You only need to call this if you've determined that you need to redraw in
            // applications which do not always need to. Applications that redraw continuously
            // can render here instead.

            // request_redraw();
        },
    );
}
