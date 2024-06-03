use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{imageops::overlay, Rgba, RgbaImage};
use imageproc::{definitions::HasWhite, drawing::{draw_text_mut, text_size}};

use crate::data_fetcher::CompleteDriverData;

use super::{DRIVER_FONT_SIZE, DRIVER_STATS_HEIGHT, DRIVER_TEAM_MARGIN, 
    LAP_SPEED_FONT_SIZE, NAME_LAP_SPEED_MARGIN, PADDING_LR, PADDING_TB, 
    PADDING_TB_INNER, SECTOR_FONT_SIZE, SECTOR_TIMES_MARGIN, 
    SIDEBAR_WIDTH, TEAM_FONT_SIZE, TRACK_HEIGHT, TRANSPARENT};

pub struct HUD<'a> {
    d1: &'a CompleteDriverData,
    d2: &'a CompleteDriverData,

    regular_font: &'a FontRef<'a>,
    bold_font: &'a FontRef<'a>
}

impl <'a> HUD<'a> {
    pub fn new(d1: &'a CompleteDriverData, d2: &'a CompleteDriverData, regular_font: &'a FontRef<'a>, bold_font: &'a FontRef<'a>) -> HUD<'a> {
        HUD { d1, d2, regular_font, bold_font }
    }

    pub fn get_hud(&self, frame: usize) -> RgbaImage {
        let mut combined_buffer = 
            RgbaImage::from_pixel(SIDEBAR_WIDTH, TRACK_HEIGHT, TRANSPARENT);
    
        let d1_stats = self.get_driver_stats(&self.d1, frame);
        let d2_stats = self.get_driver_stats(&self.d2, frame);
    
        overlay(&mut combined_buffer, &d1_stats, 0, PADDING_TB as i64);
        overlay(&mut combined_buffer, &d2_stats, 0, (TRACK_HEIGHT - PADDING_TB - DRIVER_STATS_HEIGHT) as i64);
        
        combined_buffer
    }

    fn has_finished(driver_data: &CompleteDriverData, current_frame: usize) -> bool {
        current_frame >= driver_data.telemetry.len()
    }
    
    fn get_driver_and_team_name(&self, driver_name: &str, team_name: &str, color: Rgba<u8>) -> RgbaImage {
        let driver_font = self.bold_font;
        let team_font = self.regular_font;
        
        let driver_font_scale = driver_font.pt_to_px_scale(DRIVER_FONT_SIZE as f32)
            .unwrap_or(PxScale::from(DRIVER_FONT_SIZE as f32));
        let driver_height = driver_font.as_scaled(driver_font_scale).height().ceil() as u32;
    
        let team_font_scale = team_font.pt_to_px_scale(TEAM_FONT_SIZE as f32)
            .unwrap_or(PxScale::from(TEAM_FONT_SIZE as f32));
        let team_height = team_font.as_scaled(team_font_scale).height().ceil() as u32;
    
        let mut name_buffer = RgbaImage::from_pixel(SIDEBAR_WIDTH, 
            driver_height + team_height, TRANSPARENT);
    
        draw_text_mut(&mut name_buffer, color, PADDING_LR as i32, 0, 
            driver_font_scale, driver_font, driver_name);
        draw_text_mut(&mut name_buffer, Rgba::white(), PADDING_LR as i32, 
            driver_height as i32 + DRIVER_TEAM_MARGIN, team_font_scale, team_font, team_name);
    
        name_buffer
    }
    
    fn get_speed(&self, driver_data: &CompleteDriverData, current_frame: usize) -> RgbaImage {
        let font_scale = self.bold_font.pt_to_px_scale(LAP_SPEED_FONT_SIZE as f32)
            .unwrap_or(PxScale::from(LAP_SPEED_FONT_SIZE as f32));
        let height = self.bold_font.as_scaled(font_scale).height().ceil() as u32;
    
        let speed = driver_data.telemetry[current_frame].speed;
        let speed_str = format!("{} km/h", speed);
    
        let (x, _) = text_size(font_scale, self.bold_font, &speed_str);
        let dx = (SIDEBAR_WIDTH - x) / 2;
    
        let mut speed_buffer = 
            RgbaImage::from_pixel(SIDEBAR_WIDTH, height, TRANSPARENT);
        draw_text_mut(&mut speed_buffer, Rgba::white(), 
            dx as i32, 0, font_scale, self.bold_font, &speed_str);
    
        speed_buffer
    }
    
    fn get_str_time(time: i32) -> String {
        let minutes = time / 60000;
        let seconds = (time / 1000) % 60;
        let miliseconds = time % 1000;
    
        format!("{}:{:0>2}.{:0>3}", minutes, seconds, miliseconds)
    }
    
    fn get_time(&self, time: i32) -> RgbaImage {
        let font_scale: PxScale = self.bold_font.pt_to_px_scale(LAP_SPEED_FONT_SIZE as f32)
            .unwrap_or(PxScale::from(LAP_SPEED_FONT_SIZE as f32));
        let height = self.bold_font.as_scaled(font_scale).height().ceil() as u32;
    
        let time_str = Self::get_str_time(time);
    
        let (x, _) = text_size(font_scale, self.bold_font, &time_str);
        let dx = (SIDEBAR_WIDTH - x) / 2;
    
        let mut time_buffer = 
            RgbaImage::from_pixel(SIDEBAR_WIDTH, height, TRANSPARENT);
        draw_text_mut(&mut time_buffer, Rgba::white(), 
            dx as i32, 0, font_scale, self.bold_font, &time_str);
    
        time_buffer
    }
    
    fn time_to_sector_time_str(time: i32) -> String {
        let seconds = (time / 1000) % 60;
        let miliseconds = time % 1000;
    
        format!("{:0>2}.{:0>3}", seconds, miliseconds)
    }
    
    fn get_sector_time_str(driver_data: &CompleteDriverData, sector: u8, current_frame: usize) -> String {
        let sector_session_time = match sector {
            1 => driver_data.lap.sector1_session_time,
            2 => driver_data.lap.sector2_session_time,
            3 => driver_data.lap.sector3_session_time,
            _ => i64::MAX
        };
    
        let sector_time = match sector {
            1 => driver_data.lap.sector1_time,
            2 => driver_data.lap.sector2_time,
            3 => driver_data.lap.sector3_time,
            _ => i32::MAX
        };
    
        if current_frame >= driver_data.telemetry.len() || driver_data.telemetry[current_frame].session_time >= sector_session_time {
            format!("Sector {}: {}", sector, Self::time_to_sector_time_str(sector_time))
        } else {
            String::from("")
        }
    }
    
    fn get_sector_times(&self, driver_data: &CompleteDriverData, current_frame: usize) -> RgbaImage {
        let font_scale = self.regular_font.pt_to_px_scale(SECTOR_FONT_SIZE as f32)
            .unwrap_or(PxScale::from(SECTOR_FONT_SIZE as f32));
        let height = self.regular_font.as_scaled(font_scale).height().ceil() as u32;
    
        let mut sector_times_buffer = 
            RgbaImage::from_pixel(SIDEBAR_WIDTH, 3 * height, TRANSPARENT);
    
        for i in 0..3 {
            let mut sector_time_buffer = 
                RgbaImage::from_pixel(SIDEBAR_WIDTH, height, TRANSPARENT);
            let sector_time_str = Self::get_sector_time_str(&driver_data, i + 1, current_frame);

            draw_text_mut(&mut sector_time_buffer, Rgba::white(), 
                PADDING_LR as i32, 0, font_scale, self.regular_font, &sector_time_str);
            overlay(&mut sector_times_buffer, &sector_time_buffer, 
                0, i as i64 * height as i64);
        }
    
        sector_times_buffer
    }
    
    fn get_driver_stats(&self, driver_data: &CompleteDriverData, current_frame: usize) -> RgbaImage {
        let mut stats = 
            RgbaImage::from_pixel(SIDEBAR_WIDTH, DRIVER_STATS_HEIGHT, TRANSPARENT);
    
        let driver_name_buffer = self.get_driver_and_team_name(&driver_data.driver.broadcast_name, 
            &driver_data.driver.team_name, Rgba::from(driver_data.driver.team_color));
    
        let time_or_speed = if Self::has_finished(driver_data, current_frame) {
            self.get_time(driver_data.lap.lap_time)
        } else {
            self.get_speed(driver_data, current_frame)
        };
        let sector_times = self.get_sector_times(driver_data, current_frame);
    
        overlay(&mut stats, &driver_name_buffer, 0, (PADDING_TB_INNER) as i64);
        overlay(&mut stats, &time_or_speed, 
            0, (PADDING_TB_INNER + driver_name_buffer.height() + NAME_LAP_SPEED_MARGIN) as i64);
        overlay(&mut stats, &sector_times, 
            0, (DRIVER_STATS_HEIGHT - sector_times.height() - SECTOR_TIMES_MARGIN) as i64);
    
        stats
    }
}