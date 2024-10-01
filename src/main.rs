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
            // request_redraw();
        },
    );
}
