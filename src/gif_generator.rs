use std::cmp::{min, max};
use std::fs;

use image::{codecs::gif::GifEncoder, Delay, Frame, Rgba, RgbaImage, imageops::overlay};
use imageproc::{drawing::draw_antialiased_line_segment_mut, pixelops::interpolate};

use crate::data_fetcher::DriverPosition;

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

fn transform_data(d1: &mut Vec<DriverPosition>, d2: &mut Vec<DriverPosition>, width: u32, height: u32) {
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


pub fn generate_gif(mut d1: Vec<DriverPosition>, mut d2: Vec<DriverPosition>, track_width: u32, track_height: u32) {
    transform_data(&mut d1, &mut d2, track_width, track_height);

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
            draw_antialiased_line_segment_mut(&mut d1_buf, p1, p2, Rgba([0, 0, 255, 127]), interpolate);
        }

        if i + 1 < d2.len() {
            let p1 = (d2[i].y, d2[i].x);
            let p2 = (d2[i + 1].y, d2[i + 1].x);
            draw_antialiased_line_segment_mut(&mut d2_buf, p1, p2, Rgba([255, 0, 0, 127]), interpolate);
        }

        let mut combined_img = RgbaImage::from_pixel(track_width, track_height, Rgba([255, 255, 255, 255]));
        overlay(&mut combined_img, &d1_buf, 0, 0);
        overlay(&mut combined_img, &d2_buf, 1, 1);
        
        let frame = Frame::from_parts(combined_img, 0, 0, 
            Delay::from_numer_denom_ms(50, 1));

        encoder.encode_frame(frame).expect("Can't encode frame");
    }

}