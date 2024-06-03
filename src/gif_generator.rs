mod image_resize;
mod drawing_utils;
mod hud_overlay;
mod telemetry_plot;
mod gif_consts;
mod track_map;

use std::cmp::max;
use std::fs;
use std::io::Write;
use ab_glyph::FontRef;
use image::{codecs::gif::GifEncoder, imageops::overlay, Delay, Frame, Rgba, RgbaImage};
use track_map::TrackMap;
use crate::data_fetcher::{CompleteDriverData, DriverTelemetryData};
use image_resize::*;
use drawing_utils::*;
use hud_overlay::*;
use telemetry_plot::*;
use gif_consts::*;

fn draw_telemetry_plot(d1_plot: &mut RgbaImage, d2_plot: &mut RgbaImage, d1_data: &CompleteDriverData, d2_data: &CompleteDriverData, d1_plot_data: &mut TelemetryPlotData, d2_plot_data: &mut TelemetryPlotData, d1_color: Rgba<u8>, d2_color: Rgba<u8>, current_frame: usize) {
    draw_telemetry(d1_plot, d1_plot_data, d1_data, d1_color, TELEMETRY_PLOT_WIDTH, TELEMETRY_PLOT_HEIGHT, current_frame);
    draw_telemetry(d2_plot, d2_plot_data, d2_data, d2_color, TELEMETRY_PLOT_WIDTH, TELEMETRY_PLOT_HEIGHT, current_frame);
}

fn overlay_telemetry(output_buffer: &mut RgbaImage, d1_plot: &RgbaImage, d2_plot: &RgbaImage) {
    overlay(output_buffer, d1_plot, PADDING as i64, (TRACK_HEIGHT + PADDING) as i64);
    overlay(output_buffer, d2_plot, PADDING as i64, (TRACK_HEIGHT + PADDING) as i64);
}

fn save_frame_to_gif<W>(encoder: &mut GifEncoder<W>, output_buffer: RgbaImage)
where
    W: Write, 
{
    let frame = Frame::from_parts(output_buffer, 0, 0, 
        Delay::from_numer_denom_ms(FRAME_TIME, 1));

    encoder.encode_frame(frame).expect("Can't encode frame");
}

pub fn generate_gif(mut complete_d1_data: CompleteDriverData, mut complete_d2_data: CompleteDriverData, output_path: &str) {  
    let (d1_draw_color, d2_draw_color) = get_driver_colors(&complete_d1_data.driver, &mut complete_d2_data.driver);

    let output_gif = fs::File::create(output_path).expect("Unable to create file");
    let writer = std::io::BufWriter::new(output_gif);
    let mut encoder = GifEncoder::new_with_speed(writer, 30);

    resize_data_to_dims(&mut complete_d1_data.telemetry, &mut complete_d2_data.telemetry, TRACK_WIDTH - 2 * PADDING, TRACK_HEIGHT - 2 * PADDING);
    center_data_to_dims(&mut complete_d1_data.telemetry, &mut complete_d2_data.telemetry, TRACK_WIDTH, TRACK_HEIGHT);

    let mut track_map = TrackMap::new(&complete_d1_data, &complete_d2_data, d1_draw_color, d2_draw_color);

    let regular_font = FontRef::try_from_slice(include_bytes!("../static/fonts/OpenSans-Regular.ttf")).expect("Unable to load font");
    let bold_font = FontRef::try_from_slice(include_bytes!("../static/fonts/OpenSans-Bold.ttf")).expect("Unable to load font");

    let (mut d1_plot_data, mut d2_plot_data) = TelemetryPlotData::new_pair(&complete_d1_data, &complete_d2_data);
    let mut d1_plot = new_telemetry_buffer(&regular_font);
    let mut d2_plot = new_telemetry_buffer(&regular_font);

    let no_frames = max(complete_d1_data.telemetry.len(), complete_d2_data.telemetry.len()) + 20;
    for i in 0..no_frames {
        println!("Frame {} / {}", i, no_frames - 1);

        track_map.draw_next_frame();
        draw_telemetry_plot(&mut d1_plot, &mut d2_plot, &complete_d1_data, &complete_d2_data, &mut d1_plot_data, &mut d2_plot_data, d1_draw_color, d2_draw_color, i);

        let hud = get_hud(&complete_d1_data, &complete_d2_data, i, &regular_font, &bold_font);

        let mut combined_img = RgbaImage::from_pixel(TRACK_WIDTH + SIDEBAR_WIDTH, TRACK_HEIGHT + TELEMETRY_HEIGHT, BACKGROUND_COLOR);
        overlay(&mut combined_img, &track_map.get_track_map(), 0, 0);
        overlay_telemetry(&mut combined_img, &d1_plot, &d2_plot);
        overlay(&mut combined_img, &hud, TRACK_WIDTH as i64, 0);
        
        save_frame_to_gif(&mut encoder, combined_img);
    }

}