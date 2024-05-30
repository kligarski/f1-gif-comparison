use std::cmp::{min, max};

use imageproc::drawing::{BresenhamLineIter, draw_filled_circle_mut};
use image::{Rgba, RgbaImage};
use crate::data_fetcher::DriverData;

use super::BACKGROUND_COLOR;

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

// reversed alpha-blending to preserve driver colors with transparency
pub fn compute_foreground_rgba(result: [u8; 3], background: [u8; 3]) -> Result<[u8; 4], String> {
    let (r_r, g_r, b_r) = (result[0] as f32, result[1] as f32, result[2] as f32);
    let (r_b, g_b, b_b) = (background[0] as f32, background[1] as f32, background[2] as f32);

    let mut alpha_r = 0.0;
    let mut alpha_g = 0.0;
    let mut alpha_b = 0.0;

    if r_r != r_b {
        alpha_r = (r_r - r_b) / (255.0 - r_b);
    }
    if g_r != g_b {
        alpha_g = (g_r - g_b) / (255.0 - g_b);
    }
    if b_r != b_b {
        alpha_b = (b_r - b_b) / (255.0 - b_b);
    }

    let alpha = alpha_r.max(alpha_g).max(alpha_b);

    if alpha <= 0.0 {
        return Err(String::from("Alpha value is out of bounds, indicating no valid foreground color"));
    }

    let r_f = ((r_r - (1.0 - alpha) * r_b) / alpha).clamp(0.0, 255.0);
    let g_f = ((g_r - (1.0 - alpha) * g_b) / alpha).clamp(0.0, 255.0);
    let b_f = ((b_r - (1.0 - alpha) * b_b) / alpha).clamp(0.0, 255.0);

    Ok([r_f as u8, g_f as u8, b_f as u8, (alpha * 256.0) as u8])
}

// Returns driver colors coresponding to team's colors in Rgba<u8>
// If both drivers are from the same team, the second driver is assigned 
// a complementary color and also changes it in the DriverData struct
pub fn get_driver_colors(driver1: &DriverData, driver2: &mut DriverData) -> (Rgba<u8>, Rgba<u8>) {
    let d1_color = driver1.team_color.clone();
    let mut d2_color = driver2.team_color.clone();

    if d1_color == d2_color {
        d2_color = get_complementary_color(d2_color);
        driver2.team_color = d2_color.clone();
    }

    let d1_3 = [d1_color[0], d1_color[1], d1_color[2]];
    let d2_3 = [d2_color[0], d2_color[1], d2_color[2]];
    let bg_3 = [BACKGROUND_COLOR[0], BACKGROUND_COLOR[1], BACKGROUND_COLOR[2]];

    let d1_calc = compute_foreground_rgba(d1_3, bg_3).unwrap_or(d1_color);
    let d2_calc = compute_foreground_rgba(d2_3, bg_3).unwrap_or(d2_color);

    return (Rgba::from(d1_calc), Rgba::from(d2_calc));
}   