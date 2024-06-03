use serde::{Deserialize, Deserializer, de::Error};
use std::{fs, process::Command};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DriverTelemetryData {
    pub x: i32,
    pub y: i32,
    pub session_time: i64,
    pub speed: i32,
    pub relative_distance: f64
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LapData {
    pub lap_time: i32,
    pub sector1_time: i32,
    pub sector2_time: i32,
    pub sector3_time: i32,
    pub sector1_session_time: i64,
    pub sector2_session_time: i64,
    pub sector3_session_time: i64
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DriverData {
    pub broadcast_name: String,
    pub team_name: String,
    #[serde(deserialize_with = "from_hex")]
    pub team_color: [u8; 4]
}

pub struct CompleteDriverData {
    pub telemetry: Vec<DriverTelemetryData>,
    pub lap: LapData,
    pub driver: DriverData
}

fn from_hex<'de, D>(deserializer: D) -> Result<[u8; 4], D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let r = u8::from_str_radix(&s[0..2], 16)
        .map_err(D::Error::custom)?;
    let g = u8::from_str_radix(&s[2..4], 16)
        .map_err(D::Error::custom)?;
    let b = u8::from_str_radix(&s[4..6], 16)
        .map_err(D::Error::custom)?;

    Ok([r, g, b, 255])
}

fn read_and_parse_driver_data(driver_index: i32) -> Result<CompleteDriverData, String> {
    let lap_json = fs::read_to_string(format!("lap{}_data.json", driver_index))
        .map_err(|_| String::from("Unable to read json file"))?;
    let lap_data: LapData = serde_json::from_str(&lap_json)
        .map_err(|_| String::from("Unable to parse json file"))?;

    let driver_json = fs::read_to_string(format!("driver{}_data.json", driver_index))
        .map_err(|_| String::from("Unable to read json file"))?;
    let driver_data: DriverData = serde_json::from_str(&driver_json)
        .map_err(|_| String::from("Unable to parse json file"))?;

    let telemetry_json = fs::read_to_string(format!("telemetry{}_data.json", driver_index))
        .map_err(|_| String::from("Unable to read json file"))?;
    let telemetry_data: Vec<DriverTelemetryData> = serde_json::from_str(&telemetry_json)
        .map_err(|_| String::from("Unable to parse json file"))?;

    Ok(CompleteDriverData {telemetry: telemetry_data, lap: lap_data, driver: driver_data})
}

pub fn fetch(framerate: u32, year: u32, country: &str, driver1: &str, driver2: &str, use_cached: bool) -> Result<(CompleteDriverData, CompleteDriverData), String> {
    if !use_cached {
        let status = Command::new("python")
            .arg("./f1_fast/fetch.py")
            .arg(framerate.to_string())
            .arg(year.to_string())
            .arg(country)
            .arg(driver1)
            .arg(driver2)
            .status();

        match status {
            Ok(status) if status.success() => (),
            Ok(status) => {
                if let Some(status_code) = status.code() {
                    return Err(String::from(format!("Script reported an error code {} when fetching data", status_code)));
                } else {
                    return Err(String::from("Script reported an unknown error when fetching data"));
                }
            },
            _ => return Err(String::from("Unable to run data-fetching script"))
        }
    }
    
    let d1_data = read_and_parse_driver_data(1)?;
    let d2_data = read_and_parse_driver_data(2)?;

    Ok((d1_data, d2_data))     
}