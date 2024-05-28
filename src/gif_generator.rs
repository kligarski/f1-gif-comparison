use std::cmp::{min, max};
use std::fs;

use image::{codecs::gif::GifEncoder, Delay, Frame, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::BresenhamLineIter, drawing::draw_filled_circle_mut};

use crate::data_fetcher::DriverPosition;

const THICKNESS: i32 = 3;                       // cicrle radius = (THICKNESS + 1) / 2
const PADDING: u32 = THICKNESS as u32 * 5;      // Padding on each side
const FRAME_TIME: u32 = 50;                     // Time in ms

const BACKGROUND_COLOR: Rgba<u8> = Rgba([31, 31, 31, 255]);
const DRIVER1_COLOR: Rgba<u8> = Rgba([127, 127, 255, 127]);
const DRIVER2_COLOR: Rgba<u8> = Rgba([255, 127, 127, 127]);

// fn align_data(d1: &Vec<DriverPosition>, d2: &mut Vec<DriverPosition>) {
//     let d1_first = d1.get(0).expect("Invalid data");
//     let d2_first = d2.get(0).expect("Invalid data");

//     let dx = d1_first.x - d2_first.x;
//     let dy = d1_first.y - d2_first.y;

//     for dp in d2 {
//         dp.x += dx;
//         dp.y += dy;
//     }
// }

fn find_extrema(d1: &Vec<DriverPosition>, d2: &Vec<DriverPosition>) -> ((i32, i32), (i32, i32)) {
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

fn resize_data_to_dims(d1: &mut Vec<DriverPosition>, d2: &mut Vec<DriverPosition>, width: u32, height: u32) {
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

fn center_data_to_dims(d1: &mut Vec<DriverPosition>, d2: &mut Vec<DriverPosition>, width: u32, height: u32) {
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

// Adapted from imageproc::drawing::draw_line_segment_mut
fn draw_thick_line_segment_mut(image: &mut RgbaImage, start: (f32, f32), end: (f32, f32), color: Rgba<u8>, radius: i32)
{
    let line_iterator = BresenhamLineIter::new(start, end);
    for point in line_iterator {
        draw_filled_circle_mut(image, point, radius, color);
    }
}

fn draw_thick_line_mut(image: &mut RgbaImage, start: (i32, i32), end: (i32, i32), color: Rgba<u8>, thickness: i32) {
    let radius = (thickness + 1) / 2;

    let start_f32 = (start.0 as f32, start.1 as f32);
    let end_f32 = (end.0 as f32, end.1 as f32);

    draw_thick_line_segment_mut(image, start_f32, end_f32, color, radius);
}

pub fn generate_gif(mut d1: Vec<DriverPosition>, mut d2: Vec<DriverPosition>, track_width: u32, track_height: u32) {
    resize_data_to_dims(&mut d1, &mut d2, track_width - 2 * PADDING, track_height - 2 * PADDING);
    center_data_to_dims(&mut d1, &mut d2, track_width, track_height);

    let output_gif = fs::File::create("animation.gif").expect("Unable to create file");
    let writer = std::io::BufWriter::new(output_gif);
    let mut encoder = GifEncoder::new_with_speed(writer, 30);

    let mut d1_buf = RgbaImage::from_pixel(track_width, track_height, Rgba([255, 255, 255, 0]));
    let mut d2_buf = RgbaImage::from_pixel(track_width, track_height, Rgba([255, 255, 255, 0]));

    for i in 0..max(d1.len(), d2.len())-1 {
        println!("Frame {} / {}", i, max(d1.len(), d2.len())-2);
        if i + 1 < d1.len() {
            let p1 = (d1[i].y, d1[i].x);
            let p2 = (d1[i + 1].y, d1[i + 1].x);
            draw_thick_line_mut(&mut d1_buf, p1, p2, DRIVER1_COLOR, THICKNESS);
        }

        if i + 1 < d2.len() {
            let p1 = (d2[i].y, d2[i].x);
            let p2 = (d2[i + 1].y, d2[i + 1].x);
            draw_thick_line_mut(&mut d2_buf, p1, p2, DRIVER2_COLOR, THICKNESS);
        }

        let mut combined_img = RgbaImage::from_pixel(track_width, track_height, BACKGROUND_COLOR);
        overlay(&mut combined_img, &d1_buf, 0, 0);
        overlay(&mut combined_img, &d2_buf, 0, 0);
        
        let frame = Frame::from_parts(combined_img, 0, 0, 
            Delay::from_numer_denom_ms(FRAME_TIME, 1));

        encoder.encode_frame(frame).expect("Can't encode frame");
    }

}