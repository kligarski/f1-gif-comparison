use image::Rgba;

pub const THICKNESS: i32 = 3;
pub const PADDING: u32 = THICKNESS as u32 * 5;
pub const FRAME_TIME: u32 = 50;

pub const TRACK_WIDTH: u32 = 512;
pub const TRACK_HEIGHT: u32 = 512;
pub const SIDEBAR_WIDTH: u32 = 256;
pub const TELEMETRY_HEIGHT: u32 = 256;
pub const TELEMETRY_PLOT_WIDTH: u32 = TRACK_WIDTH + SIDEBAR_WIDTH - 2 * PADDING;
pub const TELEMETRY_PLOT_HEIGHT: u32 = TELEMETRY_HEIGHT - 2 * PADDING;
pub const TELEMETRY_PLOT_AXES_LABELS_MARGIN: u32 = 32;

pub const BACKGROUND_COLOR: Rgba<u8> = Rgba([15, 15, 15, 255]);
pub const TRANSPARENT: Rgba<u8> = Rgba([255, 255, 255, 0]);