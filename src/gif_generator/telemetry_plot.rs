use std::cmp::max;

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{imageops::overlay, Rgba, RgbaImage};
use imageproc::{definitions::HasWhite, 
    drawing::{draw_line_segment_mut, draw_text_mut, text_size}};
use image::imageops::rotate270;

use crate::data_fetcher::{CompleteDriverData, DriverTelemetryData};

use super::{TELEMETRY_LABEL_FONT_SIZE, TELEMETRY_LABEL_MARGIN, 
    TELEMETRY_PLOT_AXES_LABELS_MARGIN, TELEMETRY_PLOT_HEIGHT, TELEMETRY_PLOT_WIDTH, TRANSPARENT};

struct TelemetryPlotDriverData<'a> {
    data: &'a CompleteDriverData,
    buffer: RgbaImage,
    color: Rgba<u8>,
    current_point: Option<(u32, u32)>,
}

pub struct TelemetryPlot<'a> {
    d1: TelemetryPlotDriverData<'a>,
    d2: TelemetryPlotDriverData<'a>,
    
    base_buffer: RgbaImage,
    max_speed: i32,
    current_frame: usize
}

impl <'a> TelemetryPlot<'a> {
    pub fn new(d1_complete_data: &'a CompleteDriverData, d2_complete_data: &'a CompleteDriverData, 
        d1_color: Rgba<u8>, d2_color: Rgba<u8>, font: &FontRef) -> TelemetryPlot<'a> {
        let d1 = TelemetryPlotDriverData {
            data: d1_complete_data, 
            color: d1_color, 
            buffer: RgbaImage::from_pixel(TELEMETRY_PLOT_WIDTH - TELEMETRY_PLOT_AXES_LABELS_MARGIN, 
                TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN, TRANSPARENT),
            current_point: None, 
        };

        let d2= TelemetryPlotDriverData {
            data: d2_complete_data, 
            color: d2_color, 
            buffer: RgbaImage::from_pixel(TELEMETRY_PLOT_WIDTH - TELEMETRY_PLOT_AXES_LABELS_MARGIN, 
                TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN, TRANSPARENT),
            current_point: None, 
        };

        TelemetryPlot { d1, d2, 
            max_speed: Self::get_max_speed(d1_complete_data, d2_complete_data),
            base_buffer: Self::draw_base(font),
            current_frame: 0
        }
    }

    pub fn draw_next_frame(&mut self) {
        Self::draw_telemetry(self.max_speed, &mut self.d1, self.current_frame);
        Self::draw_telemetry(self.max_speed, &mut self.d2, self.current_frame);

        self.current_frame += 1;
    }

    pub fn get_telemetry_plot(&self) -> RgbaImage {
        let mut buffer = self.base_buffer.clone();
        overlay(&mut buffer, &self.d1.buffer, TELEMETRY_PLOT_AXES_LABELS_MARGIN as i64, 0);
        overlay(&mut buffer, &self.d2.buffer, TELEMETRY_PLOT_AXES_LABELS_MARGIN as i64, 0);

        buffer
    }

    fn draw_axes(buffer: &mut RgbaImage) {
        draw_line_segment_mut(buffer, 
            (TELEMETRY_PLOT_AXES_LABELS_MARGIN as f32, (TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN) as f32), 
            (TELEMETRY_PLOT_WIDTH as f32, (TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN) as f32), 
            Rgba::white());
    
        draw_line_segment_mut(buffer, 
            (TELEMETRY_PLOT_AXES_LABELS_MARGIN as f32, 0.0), 
            (TELEMETRY_PLOT_AXES_LABELS_MARGIN as f32, (TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN) as f32), 
            Rgba::white());
    }

    fn draw_distance_label(buffer: &mut RgbaImage, font: &FontRef, scale: PxScale) {
        let (distance_label_width, _) = text_size(scale, font, "DISTANCE");
        let distance_label_x = (TELEMETRY_PLOT_WIDTH - TELEMETRY_PLOT_AXES_LABELS_MARGIN) / 2 
            + TELEMETRY_PLOT_AXES_LABELS_MARGIN - distance_label_width / 2;
        let distance_label_y = TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN + TELEMETRY_LABEL_MARGIN;
        draw_text_mut(buffer, Rgba::white(), 
            distance_label_x as i32, distance_label_y as i32, scale, font, "DISTANCE");
    }

    fn draw_speed_label(buffer: &mut RgbaImage, font: &FontRef, scale: PxScale) {
        let (speed_label_width, _) = text_size(scale, font, "SPEED");
        let height = font.as_scaled(scale).height().ceil() as u32;
    
        let mut speed_label = RgbaImage::from_pixel(speed_label_width, height, TRANSPARENT);
        draw_text_mut(&mut speed_label, Rgba::white(), 0, 0, scale, font, "SPEED");

        let speed_label_rotated = rotate270(&speed_label);
    
        let speed_label_x = TELEMETRY_PLOT_AXES_LABELS_MARGIN - TELEMETRY_LABEL_MARGIN - height;
        let speed_label_y = (TELEMETRY_PLOT_HEIGHT - TELEMETRY_PLOT_AXES_LABELS_MARGIN) / 2 - speed_label_width / 2;

        overlay(buffer, &speed_label_rotated, speed_label_x as i64, speed_label_y as i64);
    }

    fn draw_base(font: &FontRef) -> RgbaImage {
        let mut buffer = 
            RgbaImage::from_pixel(TELEMETRY_PLOT_WIDTH, TELEMETRY_PLOT_HEIGHT, TRANSPARENT);
    
        let scale = font.pt_to_px_scale(TELEMETRY_LABEL_FONT_SIZE as f32)
            .unwrap_or(PxScale::from(TELEMETRY_LABEL_FONT_SIZE as f32));

        Self::draw_distance_label(&mut buffer, font, scale);
        Self::draw_speed_label(&mut buffer, font, scale);
        Self::draw_axes(&mut buffer);
    
        buffer
    }

    fn get_max_speed(d1: &CompleteDriverData, d2: &CompleteDriverData) -> i32 {
        let get_speed = |t: &DriverTelemetryData| -> i32 { t.speed };
        let speed1 = d1.telemetry.iter().map(get_speed).max().unwrap();
        let speed2 = d2.telemetry.iter().map(get_speed).max().unwrap();
    
        max(speed1, speed2)
    }

    fn get_point(driver: &TelemetryPlotDriverData, max_speed: i32, width: u32, height: u32, current_frame: usize) -> Option<(u32, u32)> {
        let telemetry = driver.data.telemetry.get(current_frame)?;
    
        let x = (width as f64 * telemetry.relative_distance) as u32;
        let y = height - (height as f64 * (telemetry.speed as f64 / max_speed as f64)) as u32;
    
        Some((x, y))
    }

    fn draw_telemetry(max_speed: i32, driver: &mut TelemetryPlotDriverData, current_frame: usize) {
        if let Some(new_point) = Self::get_point(driver, max_speed, driver.buffer.width(), driver.buffer.height(), current_frame) {
            if let Some(prev_point) = driver.current_point {
                let start = (prev_point.0 as f32, prev_point.1 as f32);
                let end = (new_point.0 as f32, new_point.1 as f32);
                draw_line_segment_mut(&mut driver.buffer, start, end, driver.color);
            }
            driver.current_point = Some(new_point);
        }
    }
}





