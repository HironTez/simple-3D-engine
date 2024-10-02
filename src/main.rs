mod window;

fn main() {
    let mut window = window::Window::new(
        &|buffer, width, height| {
            for index in 0..(width * height) {
                let y = index / width;
                let x = index % width;
                let red = x % 255;
                let green = y % 255;
                let blue = (x * y) % 255;

                buffer[index as usize] = blue | (green << 8) | (red << 16);
            }
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
