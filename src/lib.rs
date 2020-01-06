use crate::prelude::Zmanim;
use chrono::prelude::*;

pub mod prelude;

pub fn get(time: Zmanim, latitude: f64, longitude: f64, date: NaiveDate, fixed_offset: FixedOffset) -> DateTime<FixedOffset> {
    let (sunrise, sunset) = sunrise::sunrise_sunset(latitude, longitude, date.year(), date.month(), date.day());
    let sunrise = DateTime::<FixedOffset>::from_utc(NaiveDateTime::from_timestamp(sunrise, 0), fixed_offset);
    let sunset = DateTime::<FixedOffset>::from_utc(NaiveDateTime::from_timestamp(sunset, 0), fixed_offset);
    match time {
        Zmanim::Sunrise => {
            sunrise
        }
        Zmanim::Sunset => {
            sunset
        }
    }
}