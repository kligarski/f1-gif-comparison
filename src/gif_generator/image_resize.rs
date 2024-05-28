use std::cmp::{min, max};
use crate::data_fetcher::DriverPosition;

pub fn find_extrema(d1: &Vec<DriverPosition>, d2: &Vec<DriverPosition>) -> ((i32, i32), (i32, i32)) {
    let min_x_d1 = d1.iter().map(|s| s.x).min().expect("Invalid data");
    let min_x_d2 = d2.iter().map(|s| s.x).min().expect("Invalid data");

    let max_x_d1 = d1.iter().map(|s| s.x).max().expect("Invalid data");
    let max_x_d2 = d2.iter().map(|s| s.x).max().expect("Invalid data");

    let min_y_d1 = d1.iter().map(|s| s.y).min().expect("Invalid data");
    let min_y_d2 = d2.iter().map(|s| s.y).min().expect("Invalid data");

    let max_y_d1 = d1.iter().map(|s| s.y).max().expect("Invalid data");
    let max_y_d2 = d2.iter().map(|s| s.y).max().expect("Invalid data");

    let x_range = (min(min_x_d1, min_x_d2), max(max_x_d1, max_x_d2));
    let y_range = (min(min_y_d1, min_y_d2), max(max_y_d1, max_y_d2));

    (x_range, y_range)    
}

pub fn resize_data_to_dims(d1: &mut Vec<DriverPosition>, d2: &mut Vec<DriverPosition>, width: u32, height: u32) {
    let (mut range_x, mut range_y) = find_extrema(&d1, &d2);

    if range_x.0 < 0 {
        let dx = range_x.0.abs();

        range_x.1 += dx;
        range_x.0 += dx;

        for pos in &mut *d1 {
            pos.x += dx;
        }
        for pos in &mut *d2 {
            pos.x += dx;
        }
    }

    if range_y.0 < 0 {
        let dy = range_y.0.abs();
        range_y.1 += dy;
        range_y.0 += dy;

        for pos in &mut *d1 {
            pos.y += dy;
        }
        for pos in &mut *d2 {
            pos.y += dy;
        }
    }

    if range_x.1 as u32 > height || range_y.1 as u32 > width {
        let dx = range_x.1 as f32 / height as f32;
        let dy = range_y.1 as f32 / width as f32;

        let ratio = dx.max(dy);

        for pos in &mut *d1 {
            pos.x = (pos.x as f32 / ratio).round() as i32;
            pos.y = (pos.y as f32 / ratio).round() as i32;
        }

        for pos in &mut *d2 {
            pos.x = (pos.x as f32 / ratio).round() as i32;
            pos.y = (pos.y as f32 / ratio).round() as i32;
        }
    }
}

pub fn center_data_to_dims(d1: &mut Vec<DriverPosition>, d2: &mut Vec<DriverPosition>, width: u32, height: u32) {
    let (range_x, range_y) = find_extrema(d1, d2);

    let x_size = range_x.1 - range_x.0;
    let y_size = range_y.1 - range_y.0;

    let dx = (height as i32 - x_size) / 2;
    let dy = (width as i32 - y_size) / 2;

    for pos in d1 {
        pos.x += dx;
        pos.y += dy;
    }

    for pos in d2 {
        pos.x += dx;
        pos.y += dy;
    }
}