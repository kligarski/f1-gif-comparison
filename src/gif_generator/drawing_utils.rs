use imageproc::drawing::{BresenhamLineIter, draw_filled_circle_mut};
use image::{Rgba, RgbaImage};

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