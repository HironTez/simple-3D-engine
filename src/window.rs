#![forbid(unsafe_code)]

use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::{Buffer, Context, Surface};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

#[derive(Default)]
struct App<'buffer_drawer, 'reinit_draw> {
    window: Option<Rc<winit::window::Window>>,
    surface: Option<Surface<Rc<winit::window::Window>, Rc<winit::window::Window>>>,
    draw_buffer: Option<
        &'buffer_drawer dyn Fn(
            &mut Buffer<Rc<winit::window::Window>, Rc<winit::window::Window>>,
            u32,
            u32,
        ),
    >,
    reinit_draw: Option<&'reinit_draw dyn Fn(&dyn Fn())>,
}

impl<'buffer_drawer, 'reinit_draw> ApplicationHandler for App<'buffer_drawer, 'reinit_draw> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let window = event_loop
                .create_window(winit::window::Window::default_attributes())
                .expect("Couldn't create window");
            Rc::new(window)
        };

        let context = Context::new(window.clone()).expect("Couldn't create context");
        let surface = Surface::new(&context, window.clone()).expect("Couldn't create surface");

        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.
                let window = self
                    .window
                    .as_ref()
                    .expect("Couldn't get window (not initialized)");
                let surface = self
                    .surface
                    .as_mut()
                    .expect("Couldn't get surface (not initialized)");

                let size = window.inner_size();
                let width = size.width;
                println!("width: {:#?}", width);
                let height = size.height;
                println!("height: {:#?}", height);
                let non_zero_width =
                    NonZeroU32::new(width).expect("winit::window::Window width should not be zero");
                let non_zero_height: std::num::NonZero<u32> = NonZeroU32::new(height)
                    .expect("winit::window::Window height should not be zero");

                surface
                    .resize(non_zero_width, non_zero_height)
                    .expect("Couldn't resize surface");

                let mut buffer = surface.buffer_mut().expect("Couldn't allocate buffer");

                // Call the draw function
                self.draw_buffer.expect("Couldn't access the draw function")(
                    &mut buffer,
                    width,
                    height,
                );

                buffer.present().expect("Couldn't present buffer");

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.

                self.reinit_draw
                    .expect("Couldn't access the reinit_draw function")(&|| {
                    window.request_redraw()
                });
            }
            _ => (),
        }
    }
}

pub struct Window<'buffer_drawer, 'reinit_draw> {
    app: App<'buffer_drawer, 'reinit_draw>,
}

impl<'buffer_drawer, 'reinit_draw> Window<'buffer_drawer, 'reinit_draw> {
    pub fn new<DrawBuffer, InitRedraw>(
        draw_buffer: &'buffer_drawer DrawBuffer,
        reinit_draw: &'reinit_draw InitRedraw,
    ) -> Self
    where
        DrawBuffer: Fn(&mut Buffer<Rc<winit::window::Window>, Rc<winit::window::Window>>, u32, u32),
        InitRedraw: Fn(&dyn Fn()),
    {
        let mut app = App {
            draw_buffer: Some(draw_buffer),
            reinit_draw: Some(reinit_draw),
            ..Default::default()
        };

        let event_loop = EventLoop::new().expect("Couldn't create event loop");

        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        event_loop.set_control_flow(ControlFlow::Poll);

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        event_loop.set_control_flow(ControlFlow::Wait);

        event_loop.run_app(&mut app).expect("Failed to run");

        Window { app }
    }
}
