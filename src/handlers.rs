use std::sync::{
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

use minifb::{MouseButton, MouseMode, Window};

use crate::{
    check_convergence,
    conversion::{i_to_c, palette, scale_bounds},
    HEIGHT, WIDTH,
};

pub fn handle_mouse(
    window: &Window,
    mouse_hold: &mut bool,
    redraw_tx: &Sender<Option<(f32, f32)>>,
) {
    if window.get_mouse_down(MouseButton::Left) && !*mouse_hold {
        *mouse_hold = true;
        if let Some(pos) = window.get_mouse_pos(MouseMode::Discard) {
            redraw_tx.send(Some(pos)).unwrap();
        }
    } else if !window.get_mouse_down(MouseButton::Left) && *mouse_hold {
        *mouse_hold = false;
    }
}

pub fn redraw_handler(
    redraw_rx: Receiver<Option<(f32, f32)>>,
    thread_buffer: Arc<Mutex<Vec<u32>>>,
    buffer_tx: Sender<()>,
) -> ! {
    let (mut x_min, mut y_min, mut x_width) = (-3., -1., 4.);

    loop {
        if let Ok(pos) = redraw_rx.try_recv() {
            if let Some((x, y)) = pos {
                (x_min, y_min) = scale_bounds(
                    0.7,
                    x_min,
                    y_min,
                    x_width,
                    WIDTH as u32,
                    HEIGHT as u32,
                    x as f64,
                    y as f64,
                );
                x_width *= 0.7;

                println!("Rescaled to {} {} {}", x_min, y_min, x_width);
            }

            let mut buffer = thread_buffer.lock().unwrap();

            for (i, v) in buffer.iter_mut().enumerate() {
                match check_convergence(i_to_c(
                    i as u32,
                    x_min,
                    y_min,
                    x_width,
                    WIDTH as u32,
                    HEIGHT as u32,
                )) {
                    None => *v = 0,
                    Some(stability) => {
                        *v = palette((2.0 * stability + 0.5) % 1.0);
                    }
                }
            }

            drop(buffer);

            buffer_tx.send(()).unwrap();
        }
    }
}
