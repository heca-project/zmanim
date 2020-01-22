use crate::prelude::tz::TimeZone;
use crate::prelude::Zmanim;
use chrono::prelude::*;

pub mod prelude;

pub fn get(
    zmanim: Zmanim,
    latitude: f64,
    longitude: f64,
    date: NaiveDate,
    time_zone: TimeZone,
) -> DateTime<FixedOffset> {
    let (sunrise, sunset) =
        sunrise::sunrise_sunset(latitude, longitude, date.year(), date.month(), date.day());
    let naive_sunrise = NaiveDateTime::from_timestamp(sunrise, 0);
    let naive_sunset = NaiveDateTime::from_timestamp(sunset, 0);
    let fixed_offset_sunrise = time_zone.get_tz(&naive_sunrise);
    let fixed_offset_sunset = time_zone.get_tz(&naive_sunset);
    let sunrise = DateTime::<FixedOffset>::from_utc(naive_sunrise, fixed_offset_sunrise);
    let sunset = DateTime::<FixedOffset>::from_utc(naive_sunset, fixed_offset_sunset);
    match zmanim {
        Zmanim::Sunrise => sunrise,
        Zmanim::Sunset => sunset,
        _ => unreachable!(),
    }
}
