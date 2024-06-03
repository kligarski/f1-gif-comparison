use std::cmp::max;

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{imageops::overlay, Rgba, RgbaImage};
use imageproc::{definitions::HasWhite, drawing::{draw_line_segment_mut, draw_text_mut, text_size}};
use image::imageops::rotate270;

use crate::data_fetcher::{CompleteDriverData, DriverTelemetryData};

use super::{TELEMETRY_PLOT_AXES_LABELS_MARGIN, TELEMETRY_PLOT_HEIGHT, TELEMETRY_PLOT_WIDTH, TRANSPARENT};

const TELEMETRY_LABEL_FONT_SIZE: u32 = 9;
const TELEMETRY_LABEL_MARGIN: u32 = 5;

pub struct TelemetryPlotData {
    current_point: Option<(u32, u32)>,
    max_speed: i32
}

impl TelemetryPlotData {
    pub fn new_pair(d1: &CompleteDriverData, d2: &CompleteDriverData) -> (TelemetryPlotData, TelemetryPlotData) {
        let max_speed = get_max_speed(d1, d2);
        (TelemetryPlotData{ current_point: None, max_speed },
            TelemetryPlotData{ current_point: None, max_speed })
    }
}

fn get_max_speed(d1: &CompleteDriverData, d2: &CompleteDriverData) -> i32 {
    let get_speed = |t: &DriverTelemetryData| -> i32 { t.speed };
    let speed1 = d1.telemetry.iter().map(get_speed).max().unwrap();
    let speed2 = d2.telemetry.iter().map(get_speed).max().unwrap();

    max(speed1, speed2)
}

fn get_point(driver_data: &CompleteDriverData, plot_data: &TelemetryPlotData, width: u32, height: u32, current_frame: usize) -> Option<(u32, u32)> {
    let telemetry = driver_data.telemetry.get(current_frame)?;

    let width = width - TELEMETRY_PLOT_AXES_LABELS_MARGIN;
    let height = height - TELEMETRY_PLOT_AXES_LABELS_MARGIN;

    let x = (width as f64 * telemetry.relative_distance) as u32 + TELEMETRY_PLOT_AXES_LABELS_MARGIN;
    let y = height - (height as f64 * (telemetry.speed as f64 / plot_data.max_speed as f64)) as u32;

    Some((x, y))
}

pub fn new_telemetry_buffer(font: &FontRef) -> RgbaImage {
    let mut buffer = RgbaImage::from_pixel(TELEMETRY_PLOT_WIDTH, TELEMETRY_PLOT_HEIGHT, TRANSPARENT);

    let scale = font.pt_to_px_scale(TELEMETRY_LABEL_FONT_SIZE as f32).unwrap_or(PxScale::from(TELEMETRY_LABEL_FONT_SIZE as f32));
    let (distance_label_width, _) = text_size(scale, font, "DISTANCE");
    let (speed_label_width, _) = text_size(scale, font, "SPEED");
    let height = font.as_scaled(scale).height().ceil() as u32;

    let distance_label_x = (TELEMETRY_PLOT_WIDTH - TELEMETRY_PLOT_AXES_LABELS_MARGIN) / 2 + TELEMETRY_PLOT_AXES_LABELS_MARGIN - distance_label_width / 2;
    let distance_label_y = TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN + TELEMETRY_LABEL_MARGIN;
    draw_text_mut(&mut buffer, Rgba::white(), distance_label_x as i32, distance_label_y as i32, scale, font, "DISTANCE");

    let mut speed_label = RgbaImage::from_pixel(speed_label_width, height, TRANSPARENT);
    draw_text_mut(&mut speed_label, Rgba::white(), 0, 0, scale, font, "SPEED");
    let speed_label_rotated = rotate270(&speed_label);

    let speed_label_x = TELEMETRY_PLOT_AXES_LABELS_MARGIN - TELEMETRY_LABEL_MARGIN - height;
    let speed_label_y = (TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN) / 2 - speed_label_width / 2;
    overlay(&mut buffer, &speed_label_rotated, speed_label_x as i64, speed_label_y as i64);

    draw_line_segment_mut(&mut buffer, 
        (TELEMETRY_PLOT_AXES_LABELS_MARGIN as f32, (TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN) as f32), 
        (TELEMETRY_PLOT_WIDTH as f32, (TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN) as f32), 
        Rgba::white());

        draw_line_segment_mut(&mut buffer, 
            (TELEMETRY_PLOT_AXES_LABELS_MARGIN as f32, 0.0), 
            (TELEMETRY_PLOT_AXES_LABELS_MARGIN as f32, (TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN) as f32), 
            Rgba::white());


    buffer
}

pub fn draw_telemetry(buffer: &mut RgbaImage, plot_data: &mut TelemetryPlotData, driver_data: &CompleteDriverData, color: Rgba<u8>, width: u32, height: u32, current_frame: usize) {
    if let Some(new_point) = get_point(driver_data, plot_data, width, height, current_frame) {
        if let Some(prev_point) = plot_data.current_point {
            let start = (prev_point.0 as f32, prev_point.1 as f32);
            let end = (new_point.0 as f32, new_point.1 as f32);
            draw_line_segment_mut(buffer, start, end, color);
        }
        plot_data.current_point = Some(new_point);
    }
    
}