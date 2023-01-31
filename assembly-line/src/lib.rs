// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    // unimplemented!("calculate hourly production rate at speed: {}", speed)
    let cars_per_hour: f64 = 221.;
    let rate: f64 = match speed{
        n if n >=1 && n <=4 => speed as f64 * 1. * cars_per_hour,
        n if n >= 5 && n <= 8 => speed as f64 * 0.9 * cars_per_hour,
        n if n >= 9 => speed as f64 * 0.77 * cars_per_hour,
        _ => 0.0
    };
    rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    let rate: f64 = production_rate_per_hour(speed);
    (rate / 60.) as u32
}
