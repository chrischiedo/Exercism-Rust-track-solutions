// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const BASE_NO_CARS: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed_f64 = speed as f64;

    match speed {
        0 => 0.0,
        1..=4 => speed_f64 * BASE_NO_CARS,
        5..=8 => speed_f64 * BASE_NO_CARS * 0.9,
        9..=10 => speed_f64 * BASE_NO_CARS * 0.77,
        _ => panic!("The range of speed should be between 0 and 10")
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
