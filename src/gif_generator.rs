mod image_resize;
mod drawing_utils;
mod hud_overlay;
mod telemetry_plot;
mod gif_consts;
mod track_map;

use std::{cmp::max, io::BufWriter};
use std::fs::{self, File};
use std::io::Write;
use ab_glyph::FontRef;
use image::{codecs::gif::GifEncoder, imageops::overlay, 
    Delay, Frame, RgbaImage};
use track_map::TrackMap;
use crate::data_fetcher::CompleteDriverData;
use image_resize::*;
use drawing_utils::*;
use hud_overlay::*;
use telemetry_plot::*;
use gif_consts::*;

fn save_frame_to_gif<W>(encoder: &mut GifEncoder<W>, output_buffer: RgbaImage)
where
    W: Write, 
{
    let frame = Frame::from_parts(output_buffer, 0, 0, 
        Delay::from_numer_denom_ms(FRAME_TIME, 1));

    encoder.encode_frame(frame).expect("Can't encode frame");
}

fn get_encoder(output_path: &str) -> GifEncoder<BufWriter<File>> {
    let output_gif = fs::File::create(output_path).expect("Unable to create file");
    let writer = std::io::BufWriter::new(output_gif);
    GifEncoder::new_with_speed(writer, 30)
}

pub fn generate_gif(mut complete_d1_data: CompleteDriverData, mut complete_d2_data: CompleteDriverData, output_path: &str) {  
    let mut encoder = get_encoder(output_path);

    let regular_font = FontRef::try_from_slice(
        include_bytes!("../static/fonts/OpenSans-Regular.ttf")).expect("Unable to load font");
    let bold_font = FontRef::try_from_slice(
        include_bytes!("../static/fonts/OpenSans-Bold.ttf")).expect("Unable to load font");

    let (d1_draw_color, d2_draw_color) = 
        get_driver_colors(&complete_d1_data.driver, &mut complete_d2_data.driver);

    resize_data_to_dims(&mut complete_d1_data.telemetry, &mut complete_d2_data.telemetry, 
        TRACK_WIDTH - 2 * PADDING, TRACK_HEIGHT - 2 * PADDING);
    center_data_to_dims(&mut complete_d1_data.telemetry, &mut complete_d2_data.telemetry, 
        TRACK_WIDTH, TRACK_HEIGHT);

    let mut track_map = 
        TrackMap::new(&complete_d1_data, &complete_d2_data, 
            d1_draw_color, d2_draw_color);

    let mut telemetry_plot = 
        TelemetryPlot::new(&complete_d1_data, &complete_d2_data, 
            d1_draw_color, d2_draw_color, &regular_font);

    let hud = HUD::new(&complete_d1_data, &complete_d2_data, &regular_font, &bold_font);

    let no_frames = max(complete_d1_data.telemetry.len(), complete_d2_data.telemetry.len()) + 20;
    for i in 0..no_frames {
        println!("Frame {} / {}", i, no_frames - 1);

        track_map.draw_next_frame();
        telemetry_plot.draw_next_frame();

        let mut combined_img = 
            RgbaImage::from_pixel(GIF_WIDTH, GIF_HEIGHT, BACKGROUND_COLOR);

        overlay(&mut combined_img, &track_map.get_track_map(), 0, 0);

        overlay(&mut combined_img, &telemetry_plot.get_telemetry_plot(), 
            TELEMETRY_POSITION_X, TELEMETRY_POSITION_Y);
            
        overlay(&mut combined_img, &hud.get_hud(i), 
            HUD_POSITION_X, HUD_POSITION_Y);
        
        save_frame_to_gif(&mut encoder, combined_img);
    }

}