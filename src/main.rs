#![forbid(unsafe_code)]

mod renderer;
mod scene;
mod window;

use renderer::Renderer;
use scene::camera::Camera;
use scene::light::Light;
use scene::mesh::Mesh;
use scene::Scene;
use window::Window;

fn main() {
    let cameras = vec![Camera::new(
        vec![0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0],
        100.0,
        1.777777778,
        10.0,
        10000.0,
    )];
    let lights = vec![Light::new(vec![0.0, 1000.0, 500.0], 100)];
    let meshes = vec![
        // Cube
        Mesh::new(
            vec![0.0, 0.0, 1000.0], // Position
            vec![0.0, 0.0, 0.0],    // Rotation
            100.0,                  // Scale
            vec![
                -1.0, -1.0, -1.0, // Vertex 0: Bottom-left-back
                1.0, -1.0, -1.0, //  Vertex 1: Bottom-right-back
                1.0, 1.0, -1.0, //   Vertex 2: Top-right-back
                -1.0, 1.0, -1.0, //  Vertex 3: Top-left-back
                -1.0, -1.0, 1.0, //  Vertex 4: Bottom-left-front
                1.0, -1.0, 1.0, //   Vertex 5: Bottom-right-front
                1.0, 1.0, 1.0, //    Vertex 6: Top-right-front
                -1.0, 1.0, 1.0, //   Vertex 7: Top-left-front
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
