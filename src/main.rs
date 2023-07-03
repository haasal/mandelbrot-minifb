use std::{
    sync::{self, Arc, Mutex},
    thread,
};

use complex::C;
use handlers::{handle_mouse, redraw_handler};
use minifb::{Key, Window, WindowOptions};

mod complex;
mod conversion;
mod handlers;

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

/// Computes if the mandelbrot series converges for a complex c
fn check_convergence(c: C) -> Option<f64> {
    let max_iterations: u32 = 256;
    let mut i = 0;
    let mut z = C { im: 0., re: 0. };
    let cutoff = 36.;

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

fn mainloop(mut window: Window, buffer: Vec<u32>) {
    let (redraw_tx, redraw_rx) = sync::mpsc::channel();
    let (buffer_tx, buffer_rx) = sync::mpsc::channel();

    let thread_buffer = Arc::new(Mutex::new(buffer));
    let main_buffer = thread_buffer.clone();

    redraw_tx.send(None).unwrap();
    thread::spawn(move || redraw_handler(redraw_rx, thread_buffer, buffer_tx));

    let mut mouse_hold: bool = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_mouse(&window, &mut mouse_hold, &redraw_tx);

        if let Ok(_) = buffer_rx.try_recv() {
            let buffer = main_buffer.lock().unwrap();
            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        } else {
            window.update();
        }
    }
}
