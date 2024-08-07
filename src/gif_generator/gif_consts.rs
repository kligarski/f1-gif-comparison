use image::Rgba;

pub const PADDING: u32 = THICKNESS as u32 * 5;
pub const PADDING_LR: u32 = 20;
pub const PADDING_TB: u32 = 38;
pub const PADDING_TB_INNER: u32 = 10;

pub const TRACK_WIDTH: u32 = 512;
pub const TRACK_HEIGHT: u32 = 512;
pub const SIDEBAR_WIDTH: u32 = 256;
pub const TELEMETRY_HEIGHT: u32 = 256;
pub const TELEMETRY_PLOT_WIDTH: u32 = TRACK_WIDTH + SIDEBAR_WIDTH - 2 * PADDING;
pub const TELEMETRY_PLOT_HEIGHT: u32 = TELEMETRY_HEIGHT - 2 * PADDING;
pub const DRIVER_STATS_HEIGHT: u32 = 200;

pub const TELEMETRY_PLOT_AXES_LABELS_MARGIN: u32 = 32;
pub const TELEMETRY_LABEL_MARGIN: u32 = 5;
pub const DRIVER_TEAM_MARGIN: i32 = -5;
pub const NAME_LAP_SPEED_MARGIN: u32 = 5;
pub const SECTOR_TIMES_MARGIN: u32 = 3;

pub const GIF_WIDTH: u32 = TRACK_WIDTH + SIDEBAR_WIDTH;
pub const GIF_HEIGHT: u32 = TRACK_HEIGHT + TELEMETRY_HEIGHT;

pub const TELEMETRY_LABEL_FONT_SIZE: u32 = 9;
pub const DRIVER_FONT_SIZE: u32 = 20;
pub const TEAM_FONT_SIZE: u32 = 12;
pub const LAP_SPEED_FONT_SIZE: u32 = 24;
pub const SECTOR_FONT_SIZE: u32 = TEAM_FONT_SIZE;

pub const TELEMETRY_POSITION_X: i64 = PADDING as i64;
pub const TELEMETRY_POSITION_Y: i64 = (TRACK_HEIGHT + PADDING) as i64;

pub const HUD_POSITION_X: i64 = TRACK_WIDTH as i64;
pub const HUD_POSITION_Y: i64 = 0;

pub const BACKGROUND_COLOR: Rgba<u8> = Rgba([15, 15, 15, 255]);
pub const TRANSPARENT: Rgba<u8> = Rgba([255, 255, 255, 0]);

pub const THICKNESS: i32 = 3;


