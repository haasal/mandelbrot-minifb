use crate::complex::C;

fn i_to_xy(i: u32, win_width: u32) -> (u32, u32) {
    (i % win_width, i / win_width)
}

pub fn i_to_c(i: u32, x_min: f64, y_min: f64, x_width: f64, win_width: u32, win_height: u32) -> C {
    let (win_x, win_y) = i_to_xy(i, win_width);
    let y_width = y_width(x_width, win_width, win_height);
    let x = win_x as f64 / win_width as f64 * x_width + x_min;
    let y = win_y as f64 / win_height as f64 * y_width + y_min;

    C { im: y, re: x }
}

/// scales left side x0 to new x0
fn scale_origin(f: f64, x: f64, x0: f64) -> f64 {
    x - f * x + f * x0
}

pub fn scale_bounds(
    f: f64,
    x_min: f64,
    y_min: f64,
    x_width: f64,
    win_width: u32,
    win_height: u32,
    win_x: f64,
    win_y: f64,
) -> (f64, f64) {
    let x = x_min + x_width * (win_x / win_width as f64);
    let x_min_new = scale_origin(f, x, x_min);
    let y = y_min + y_width(x_width, win_width, win_height) * (win_y / win_height as f64);
    let y_min_new = scale_origin(f, y, y_min);

    (x_min_new, y_min_new)
}

fn y_width(x_width: f64, win_width: u32, win_height: u32) -> f64 {
    x_width / win_width as f64 * win_height as f64
}

pub fn palette(t: f64) -> u32 {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);
    let r = b.0 * (6.28318 * (c.0 * t + d.0)).cos() + a.0;
    let g = b.1 * (6.28318 * (c.1 * t + d.1)).cos() + a.1;
    let b = b.2 * (6.28318 * (c.2 * t + d.2)).cos() + a.2;
    (((255.0 * r) as u32) << 0x10) + (((255.0 * g) as u32) << 0x8) + ((255.0 * b) as u32)
}
