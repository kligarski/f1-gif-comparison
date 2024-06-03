use std::cmp::{min, max};

use imageproc::drawing::{BresenhamLineIter, draw_filled_circle_mut};
use image::{Rgba, RgbaImage};
use crate::data_fetcher::DriverData;

// Adapted from imageproc::drawing::draw_line_segment_mut
fn draw_thick_line_segment_mut(image: &mut RgbaImage, start: (f32, f32), end: (f32, f32), color: Rgba<u8>, radius: i32)
{
    let line_iterator = BresenhamLineIter::new(start, end);
    for point in line_iterator {
        draw_filled_circle_mut(image, point, radius, color);
    }
}

pub fn draw_thick_line_mut(image: &mut RgbaImage, start: (i32, i32), end: (i32, i32), color: Rgba<u8>, thickness: i32) {
    let radius = (thickness + 1) / 2;

    let start_f32 = (start.0 as f32, start.1 as f32);
    let end_f32 = (end.0 as f32, end.1 as f32);

    draw_thick_line_segment_mut(image, start_f32, end_f32, color, radius);
}

// Adapted from: https://stackoverflow.com/a/37675777
fn get_complementary_color(color: [u8; 4]) -> [u8; 4] {
    let r = color[0];
    let g = color[1];
    let b = color[2];
    let a = color[3];

    let max_rgb = max(r, max(g, b)) as u16;
    let min_rgb = min(r, min(g, b)) as u16;

    let c_r = (max_rgb + min_rgb - r as u16) as u8;
    let c_g = (max_rgb + min_rgb - g as u16) as u8;
    let c_b = (max_rgb + min_rgb - b as u16) as u8;

    [c_r, c_g, c_b, a]
}


// Returns driver colors coresponding to team's colors in Rgba<u8> with added transparency.
// If both drivers are from the same team, the second driver is assigned 
// a complementary color or blue (no complementary color to white)
pub fn get_driver_colors(driver1: &DriverData, driver2: &mut DriverData) -> (Rgba<u8>, Rgba<u8>) {
    let mut d1_color = driver1.team_color.clone();
    let mut d2_color = driver2.team_color.clone();

    if d1_color == d2_color {
        d2_color = if d2_color == [255, 255, 255, 255] {[102, 153, 255, 255]} else {get_complementary_color(d2_color)};
        driver2.team_color = d2_color.clone();
    }

    d1_color[3] = 180;
    d2_color[3] = 180;

    return (Rgba::from(d1_color), Rgba::from(d2_color));
}   