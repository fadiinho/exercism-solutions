// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = ((speed as u64) * 221) as f64;

    let correct_rate: f64;

    if (1..5).contains(&speed) {
        correct_rate = rate
    } else if (5..9).contains(&speed) {
        correct_rate = rate * 0.9;
    } else if (9..11).contains(&speed) {
        correct_rate = rate * 0.77;
    } else {
        correct_rate = 0.0
    }

   correct_rate 
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let per_hour = production_rate_per_hour(speed);

    (per_hour / 60.0) as u32
}
