use image::{imageops::overlay, Rgba, RgbaImage};

use crate::data_fetcher::CompleteDriverData;

use super::{draw_thick_line_mut, BACKGROUND_COLOR, THICKNESS, TRACK_HEIGHT, TRACK_WIDTH, TRANSPARENT};

struct TrackMapDriverData<'a> {
    data: &'a CompleteDriverData,
    buffer: RgbaImage,
    color: Rgba<u8>
}

pub struct TrackMap<'a> {
    d1: TrackMapDriverData<'a>,
    d2: TrackMapDriverData<'a>,

    current_frame: usize
}

impl <'a> TrackMap<'a> {
    pub fn new(d1_complete_data: &'a CompleteDriverData, d2_complete_data: &'a CompleteDriverData, d1_color: Rgba<u8>, d2_color: Rgba<u8>) -> TrackMap<'a> {
        let d1 = TrackMapDriverData {
            data: d1_complete_data, 
            color: d1_color, 
            buffer: RgbaImage::from_pixel(TRACK_WIDTH, TRACK_HEIGHT, TRANSPARENT) 
        };

        let d2= TrackMapDriverData {
            data: d2_complete_data, 
            color: d2_color, 
            buffer: RgbaImage::from_pixel(TRACK_WIDTH, TRACK_HEIGHT, TRANSPARENT) 
        };

        TrackMap { d1, d2, current_frame: 0 }
    }

    fn can_create_frame(driver: &TrackMapDriverData, current_frame: usize) -> bool {
        current_frame + 1 < driver.data.telemetry.len()
    }
    
    fn draw_frame(driver: &mut TrackMapDriverData, current_frame: usize) {
        let p1 = 
            (driver.data.telemetry[current_frame].y, driver.data.telemetry[current_frame].x);
        let p2 = 
            (driver.data.telemetry[current_frame + 1].y, driver.data.telemetry[current_frame + 1].x);

        draw_thick_line_mut(&mut driver.buffer, p1, p2, driver.color, THICKNESS);
    }

    pub fn draw_next_frame(&mut self) {
        if Self::can_create_frame(&self.d1, self.current_frame) {
            Self::draw_frame(&mut self.d1, self.current_frame);
        }
    
        if Self::can_create_frame(&self.d2, self.current_frame) {
            Self::draw_frame(&mut self.d2, self.current_frame);
        }

        self.current_frame += 1;
    }

    pub fn get_track_map(&self) -> RgbaImage {
        let mut track_map = RgbaImage::from_pixel(TRACK_WIDTH, TRACK_HEIGHT, BACKGROUND_COLOR);
        overlay(&mut track_map, &self.d1.buffer, 0, 0);
        overlay(&mut track_map, &self.d2.buffer, 0, 0);

        track_map
    }
}