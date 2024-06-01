use std::cmp::max;

use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;

use crate::data_fetcher::{CompleteDriverData, DriverTelemetryData};


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

    let x = (width as f64 * telemetry.relative_distance) as u32;
    let y = height - (height as f64 * (telemetry.speed as f64 / plot_data.max_speed as f64)) as u32;

    Some((x, y))
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