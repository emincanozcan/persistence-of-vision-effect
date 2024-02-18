use iter_tools::Itertools;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn points_for_line(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    // Bresenham's line algorithm
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    let mut points = Vec::new();

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();

    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };

    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    loop {
        points.push((x, y));

        if x == x2 && y == y2 {
            break;
        }

        let e2 = 2 * err;

        if e2 > -dy {
            err -= dy;
            x += sx;
        }

        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    points
}

pub fn points_for_square(x1: i32, y1: i32, r: i32) -> Vec<(i32, i32)> {
    let mut points = vec![];

    let x2 = x1 + r;
    let y2 = y1 + r;

    for point in points_for_line(x1 as i32, y1 as i32, x2 as i32, y1 as i32) {
        points.push(point);
    }
    for point in points_for_line(x2 as i32, y1 as i32, x2 as i32, y2 as i32) {
        points.push(point);
    }
    for point in points_for_line(x2 as i32, y2 as i32, x1 as i32, y2 as i32) {
        points.push(point);
    }
    for point in points_for_line(x1 as i32, y2 as i32, x1 as i32, y1 as i32) {
        points.push(point);
    }

    points.into_iter().unique().collect()
}

pub fn points_for_cube(x1: i32, y1: i32, r: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    let offset = r / 5;
    let x1 = x1 - offset / 2;
    let y1 = y1 - offset / 2;

    for point in points_for_square(x1, y1, r) {
        points.push(point);
        points.push((point.0+1, point.1 + 1));
        points.push((point.0+1, point.1 + 2));
        points.push((point.0+2, point.1 + 1));
        points.push((point.0+2, point.1 + 2));
    }

    for point in points_for_square(x1 + offset, y1 + offset, r) {
        points.push(point);
        points.push((point.0+1, point.1 + 1));
        points.push((point.0+1, point.1 + 2));
        points.push((point.0+2, point.1 + 1));
        points.push((point.0+2, point.1 + 2));
    }

    for point in points_for_line(x1, y1, x1 + offset, y1 + offset) {
        points.push(point);
        points.push((point.0+1, point.1 + 1));
        points.push((point.0+1, point.1 + 2));
        points.push((point.0+2, point.1 + 1));
        points.push((point.0+2, point.1 + 2));
    }

    for point in points_for_line(x1 + r, y1, x1 + r + offset, y1 + offset) {
        points.push(point);
        points.push((point.0+1, point.1 + 1));
        points.push((point.0+1, point.1 + 2));
        points.push((point.0+2, point.1 + 1));
        points.push((point.0+2, point.1 + 2));
    }
    for point in points_for_line(x1, y1 + r, x1 + offset, y1 + r + offset) {
        points.push(point);
        points.push((point.0+1, point.1 + 1));
        points.push((point.0+1, point.1 + 2));
        points.push((point.0+2, point.1 + 1));
        points.push((point.0+2, point.1 + 2));
    }
    for point in points_for_line(x1 + r, y1 + r, x1 + r + offset, y1 + r + offset) {
        points.push(point);
        points.push((point.0+1, point.1 + 1));
        points.push((point.0+1, point.1 + 2));
        points.push((point.0+2, point.1 + 1));
        points.push((point.0+2, point.1 + 2));
    }

    return points.into_iter().unique().collect();
}

pub fn rotate_shape(points: &mut Vec<(i32, i32)>, angle: f32) {
    // rotate
    let mut total_x = 0.0;
    let mut total_y = 0.0;
    let num_points = points.len() as i32;

    for point in &mut *points {
        total_x += point.0 as f32;
        total_y += point.1 as f32;
    }

    let cx = total_x / num_points as f32;
    let cy = total_y / num_points as f32;

    let sin_a = angle.sin();
    let cos_a = angle.cos();

    for point in points {
        let x = point.0 as f32 - cx as f32;
        let y = point.1 as f32 - cy as f32;
        point.0 = (x * cos_a - y * sin_a + cx) as i32;
        point.1 = (x * sin_a + y * cos_a + cy) as i32;
    }
}
