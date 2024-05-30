use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{imageops::overlay, Rgba, RgbaImage};
use imageproc::{definitions::HasWhite, drawing::{draw_text_mut, text_size}};

use crate::data_fetcher::CompleteDriverData;

use super::{SIDEBAR_WIDTH, TRACK_HEIGHT};

const DRIVER_STATS_HEIGHT: u32 = 200;
const PADDING_LR: u32 = 20;
const PADDING_TB: u32 = 38;
const PADDING_TB_INNER: u32 = 20;

const DRIVER_TEAM_MARGIN: i32 = -5;
const NAME_LAP_SPEED_MARGIN: u32 = 5;

const DRIVER_FONT_SIZE: u32 = 20;
const TEAM_FONT_SIZE: u32 = 12;
const LAP_SPEED_FONT_SIZE: u32 = 24;

fn has_finished(driver_data: &CompleteDriverData, current_frame: usize) -> bool {
    current_frame >= driver_data.telemetry.len()
}

fn get_driver_and_team_name(driver_name: &str, team_name: &str, driver_font: &FontRef, team_font: &FontRef, color: Rgba<u8>) -> RgbaImage {
    let driver_font_scale = driver_font.pt_to_px_scale(DRIVER_FONT_SIZE as f32).unwrap_or(PxScale::from(DRIVER_FONT_SIZE as f32));
    let driver_height = driver_font.as_scaled(driver_font_scale).height().ceil() as u32;

    let team_font_scale = team_font.pt_to_px_scale(TEAM_FONT_SIZE as f32).unwrap_or(PxScale::from(TEAM_FONT_SIZE as f32));
    let team_height = team_font.as_scaled(team_font_scale).height().ceil() as u32;

    let mut name_buffer = RgbaImage::from_pixel(SIDEBAR_WIDTH, 
        driver_height + team_height, Rgba::from([255, 255, 255, 0]));

    draw_text_mut(&mut name_buffer, color, PADDING_LR as i32, 0, driver_font_scale, driver_font, driver_name);
    draw_text_mut(&mut name_buffer, Rgba::white(), PADDING_LR as i32, 
        driver_height as i32 + DRIVER_TEAM_MARGIN, team_font_scale, team_font, team_name);

    name_buffer
}

fn get_speed(driver_data: &CompleteDriverData, current_frame: usize, font: &FontRef) -> RgbaImage {
    let font_scale = font.pt_to_px_scale(LAP_SPEED_FONT_SIZE as f32).unwrap_or(PxScale::from(LAP_SPEED_FONT_SIZE as f32));
    let height = font.as_scaled(font_scale).height().ceil() as u32;

    let speed = driver_data.telemetry[current_frame].speed;
    let speed_str = format!("{} km/h", speed);

    let (x, _) = text_size(font_scale, font, &speed_str);
    let dx = (SIDEBAR_WIDTH - x) / 2;

    let mut speed_buffer = RgbaImage::from_pixel(SIDEBAR_WIDTH, height, Rgba::from([255, 255, 255, 0]));
    draw_text_mut(&mut speed_buffer, Rgba::white(), dx as i32, 0, font_scale, font, &speed_str);

    speed_buffer
}

fn get_str_time(time: i32) -> String {
    let minutes = time / 60000;
    let seconds = (time / 1000) % 60;
    let miliseconds = time % 1000;

    format!("{}:{:0>2}.{:0>3}", minutes, seconds, miliseconds)
}

fn get_time(time: i32, font: &FontRef) -> RgbaImage {
    let font_scale = font.pt_to_px_scale(LAP_SPEED_FONT_SIZE as f32).unwrap_or(PxScale::from(LAP_SPEED_FONT_SIZE as f32));
    let height = font.as_scaled(font_scale).height().ceil() as u32;

    let time_str = get_str_time(time);

    let (x, _) = text_size(font_scale, font, &time_str);
    let dx = (SIDEBAR_WIDTH - x) / 2;

    let mut time_buffer = RgbaImage::from_pixel(SIDEBAR_WIDTH, height, Rgba::from([255, 255, 255, 0]));
    draw_text_mut(&mut time_buffer, Rgba::white(), dx as i32, 0, font_scale, font, &time_str);

    time_buffer
}

fn get_driver_stats(driver_data: &CompleteDriverData, current_frame: usize, regular_font: &FontRef, bold_font: &FontRef) -> RgbaImage {
    let mut stats = RgbaImage::from_pixel(SIDEBAR_WIDTH, DRIVER_STATS_HEIGHT, Rgba::from([255, 255, 255, 0]));

    let driver_name_buffer = get_driver_and_team_name(&driver_data.driver.broadcast_name, 
        &driver_data.driver.team_name, bold_font, regular_font, Rgba::from(driver_data.driver.team_color));

    let time_or_speed = if has_finished(driver_data, current_frame) {
        get_time(driver_data.lap.lap_time, bold_font)
    } else {
        get_speed(driver_data, current_frame, bold_font)
    };
    // let sector_times = get_sector_times();

    overlay(&mut stats, &driver_name_buffer, 0, (PADDING_TB_INNER) as i64);
    overlay(&mut stats, &time_or_speed, 0, (PADDING_TB_INNER + driver_name_buffer.height() + NAME_LAP_SPEED_MARGIN) as i64);

    stats
}

pub fn get_hud(d1: &CompleteDriverData, d2: &CompleteDriverData, current_frame: usize, regular_font: &FontRef, bold_font: &FontRef) -> RgbaImage {
    let mut combined_buffer = RgbaImage::from_pixel(SIDEBAR_WIDTH, TRACK_HEIGHT, Rgba::from([255, 255, 255, 0]));

    let d1_stats = get_driver_stats(d1, current_frame, regular_font, bold_font);
    let d2_stats = get_driver_stats(d2, current_frame, regular_font, bold_font);

    overlay(&mut combined_buffer, &d1_stats, 0, PADDING_TB as i64);
    overlay(&mut combined_buffer, &d2_stats, 0, (TRACK_HEIGHT - PADDING_TB - DRIVER_STATS_HEIGHT) as i64);
    
    combined_buffer
}