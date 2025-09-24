#![forbid(unsafe_code)]

mod renderer;
mod scene;
mod tools;
mod window;

use std::{
    cell::RefCell,
    collections::HashSet,
    rc::Rc,
    sync::{Arc, Mutex},
};

use renderer::Renderer;
use scene::camera::Camera;
use scene::light::Light;
use scene::mesh::Mesh;
use scene::Scene;
use tools::vector3::Vector3;
use window::Window;
use winit::keyboard::KeyCode;

fn main() {
    let scene = Arc::new(Mutex::new(Scene::new(
        vec![Camera::new(
            Vector3::new(0.0, 0.0, -10.0), // Position
            Vector3::new(0.0, 0.0, 0.0),   // Rotation
            90.0,                          // FOV
            None,                          // Aspect Ratio
            0.1,                           // Near Plane
            100.0,                         // Far Plane
        )],
        vec![Light::new(Vector3::new(0.0, 100.0, 50.0), 100)],
        vec![
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
        ],
    )));

    let render_scene = {
        let scene = Arc::clone(&scene);
        move |buffer: &mut [u32], width: u32, height: u32| {
            let scene = scene.lock().unwrap();
            Renderer::render(&scene, 0, buffer, width, height);
        }
    };

    let update_scene = {
        let scene = Arc::clone(&scene);
        move |keys: &HashSet<KeyCode>| {
            let mut scene = scene.lock().unwrap();
            let camera = &mut scene.cameras[0];

            if keys.contains(&KeyCode::KeyW) {
                camera.position.z += 0.1;
            }
            if keys.contains(&KeyCode::KeyS) {
                camera.position.z -= 0.1;
            }
            if keys.contains(&KeyCode::KeyA) {
                camera.position.x -= 0.1;
            }
            if keys.contains(&KeyCode::KeyD) {
                camera.position.x += 0.1;
            }
            if keys.contains(&KeyCode::ArrowUp) {
                camera.rotation.x -= 1.0;
            }
            if keys.contains(&KeyCode::ArrowDown) {
                camera.rotation.x += 1.0;
            }
            if keys.contains(&KeyCode::ArrowLeft) {
                camera.rotation.y -= 1.0;
            }
            if keys.contains(&KeyCode::ArrowRight) {
                camera.rotation.y += 1.0;
            }
        }
    };

    let mut window = Window::new(
        &|buffer: &mut [u32], width: u32, height: u32| {
            render_scene(buffer, width, height);
        },
        &|request_redraw, keys| {
            // Queue a RedrawRequested event.
            //
            // You only need to call this if you've determined that you need to redraw in
            // applications which do not always need to. Applications that redraw continuously
            // can render here instead.

            update_scene(keys);
            request_redraw();
        },
    );
}
