mod image_resize;
mod drawing_utils;

use std::cmp::max;
use std::fs;
use std::io::Write;
use image::{codecs::gif::GifEncoder, Delay, Frame, Rgba, RgbaImage, imageops::overlay};
use crate::data_fetcher::{CompleteDriverData, DriverTelemetryData};
use image_resize::*;
use drawing_utils::*;


const THICKNESS: i32 = 3;
const PADDING: u32 = THICKNESS as u32 * 5;
const FRAME_TIME: u32 = 50;
const SIDEBAR_WIDTH: u32 = 256;

const BACKGROUND_COLOR: Rgba<u8> = Rgba([15, 15, 15, 255]);
const DRIVER1_COLOR: Rgba<u8> = Rgba([127, 127, 255, 127]);
const DRIVER2_COLOR: Rgba<u8> = Rgba([255, 127, 127, 127]);
const TRANSPARENT: Rgba<u8> = Rgba([255, 255, 255, 0]);

fn can_create_frame(driver_data: &Vec<DriverTelemetryData>, current_frame: usize) -> bool {
    current_frame + 1 < driver_data.len()
}

fn draw_frame(image_buffer: &mut RgbaImage, driver_data: &Vec<DriverTelemetryData>, current_frame: usize, color: Rgba<u8>, thickness: i32) {
    let p1 = (driver_data[current_frame].y, driver_data[current_frame].x);
    let p2 = (driver_data[current_frame + 1].y, driver_data[current_frame + 1].x);
    draw_thick_line_mut(image_buffer, p1, p2, color, thickness);
}

fn draw_driver_frames(d1_buffer: &mut RgbaImage, d2_buffer: &mut RgbaImage, d1: &Vec<DriverTelemetryData>, d2: &Vec<DriverTelemetryData>, current_frame: usize) {
    if can_create_frame(&d1, current_frame) {
        draw_frame(d1_buffer, &d1, current_frame, DRIVER1_COLOR, THICKNESS);
    }

    if can_create_frame(&d2, current_frame) {
        draw_frame(d2_buffer, &d2, current_frame, DRIVER2_COLOR, THICKNESS);
    }
}

fn overlay_driver_frames(output_buffer: &mut RgbaImage, d1_buffer: &RgbaImage, d2_buffer: &RgbaImage) {
    overlay(output_buffer, d1_buffer, 0, 0);
    overlay(output_buffer, d2_buffer, 0, 0);
}

fn save_frame_to_gif<W>(encoder: &mut GifEncoder<W>, output_buffer: RgbaImage)
where
    W: Write, 
{
    let frame = Frame::from_parts(output_buffer, 0, 0, 
        Delay::from_numer_denom_ms(FRAME_TIME, 1));

    encoder.encode_frame(frame).expect("Can't encode frame");
}

pub fn generate_gif(mut complete_d1_data: CompleteDriverData, mut complete_d2_data: CompleteDriverData, track_width: u32, track_height: u32, output_path: &str) {  
    let mut d1 = &mut complete_d1_data.telemetry;
    let mut d2 = &mut complete_d2_data.telemetry;
    
    let output_gif = fs::File::create(output_path).expect("Unable to create file");
    let writer = std::io::BufWriter::new(output_gif);
    let mut encoder = GifEncoder::new_with_speed(writer, 30);

    let mut d1_buffer = RgbaImage::from_pixel(track_width, track_height, TRANSPARENT);
    let mut d2_buffer = RgbaImage::from_pixel(track_width, track_height, TRANSPARENT);

    resize_data_to_dims(&mut d1, &mut d2, track_width - 2 * PADDING, track_height - 2 * PADDING);
    center_data_to_dims(&mut d1, &mut d2, track_width, track_height);

    let no_frames = max(d1.len(), d2.len()) - 1;
    for i in 0..no_frames {
        println!("Frame {} / {}", i, no_frames - 1);

        draw_driver_frames(&mut d1_buffer, &mut d2_buffer, &d1, &d2, i);

        let mut combined_img = RgbaImage::from_pixel(track_width + SIDEBAR_WIDTH, track_height, BACKGROUND_COLOR);
        overlay_driver_frames(&mut combined_img, &d1_buffer, &d2_buffer);
        
        save_frame_to_gif(&mut encoder, combined_img);
    }

}