use complex::C;
use minifb::{Key, Window, WindowOptions};

mod complex;

const WIDTH: usize = 1980;
const HEIGHT: usize = 1080;

fn main() {
    let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    mainloop(window, buffer);
}

fn i_to_xy(i: u32, win_width: u32) -> (u32, u32) {
    (i % win_width, i / win_width)
}

fn scale(x: f64, min: f64, max: f64, new_min: f64, new_max: f64) -> f64 {
    let delta = max - min;
    let new_delta = new_max - new_min;
    x / delta * new_delta + new_min
}

fn i_to_c(i: u32, x_min: f64, y_min: f64, x_width: f64, win_width: u32, win_height: u32) -> C {
    let (win_x, win_y) = i_to_xy(i, win_width);
    let y_width = x_width / win_width as f64 * win_height as f64;
    let x = win_x as f64 / win_width as f64 * x_width + x_min;
    let y = win_y as f64 / win_height as f64 * y_width + y_min;

    C { im: y, re: x }
}

fn palette(t: f64) -> u32 {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);
    let r = b.0 * (6.28318 * (c.0 * t + d.0)).cos() + a.0;
    let g = b.1 * (6.28318 * (c.1 * t + d.1)).cos() + a.1;
    let b = b.2 * (6.28318 * (c.2 * t + d.2)).cos() + a.2;
    (((255.0 * r) as u32) << 0x10) + (((255.0 * g) as u32) << 0x8) + ((255.0 * b) as u32)
}

/// Computes if the mandelbrot series converges for a complex c
fn check_convergence(c: C) -> Option<f64> {
    let max_iterations: u32 = 256;
    let mut i = 0;
    let mut z = C { im: 0., re: 0. };
    let cutoff = 32.;

    loop {
        if z.norm() > cutoff {
            return Some((i as f64 - z.norm().log2().log2()) / max_iterations as f64);
        }

        if i > max_iterations {
            return None;
        }

        z = z * z + c;
        i += 1;
    }
}

fn mainloop(mut window: Window, mut buffer: Vec<u32>) {
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, v) in buffer.iter_mut().enumerate() {
            match check_convergence(i_to_c(i as u32, -2., -1., 4., WIDTH as u32, HEIGHT as u32)) {
                None => *v = 0,
                Some(stability) => *v = palette(1. - stability),
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
