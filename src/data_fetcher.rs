use serde::Deserialize;
use std::{fs, process::Command};


#[derive(Deserialize, Debug)]
pub struct DriverPosition {
    pub x: i32,
    pub y: i32
}

pub fn fetch(year: u32, country: &str, driver1: &str, driver2: &str, use_cached: bool) -> (Vec<DriverPosition>, Vec<DriverPosition>) {
    if !use_cached {
        Command::new("python")
            .arg("./f1_fast/fetch.py")
            .arg(year.to_string())
            .arg(country)
            .arg(driver1)
            .arg(driver2)
            .status().expect("Unable to fetch data.");
    }
    
    let d1_json = fs::read_to_string("driver1_telemetry.json")
        .expect("Unable to read json file.");
    let d2_json =  fs::read_to_string("driver2_telemetry.json")
        .expect("Unable to read json file.");

    let d1_pos_data: Vec<DriverPosition> = serde_json::from_str(&d1_json).expect("Unable to parse json.");
    let d2_pos_data: Vec<DriverPosition> = serde_json::from_str(&d2_json).expect("Unable to parse json.");

    (d1_pos_data, d2_pos_data)        
}