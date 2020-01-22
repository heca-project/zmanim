use chrono::Duration;
use chrono::{DateTime, FixedOffset, NaiveDate, TimeZone};
use std::convert::TryFrom;
use zmanim::get;
use zmanim::prelude::Zmanim;

#[test]
fn check_time_zones() {
    use zmanim::prelude::tz::*;
    for l in include_str!("tz_list").lines() {
        TimeZone::try_from(l).expect(&format!("{} {}: {}", file!(), line!(), l));
    }
}

#[test]
fn check_zmanim_windhoek() {
    use zmanim::prelude::tz::*;
    let tz = get(
        Zmanim::Sunset,
        -22.0,
        17.00,
        NaiveDate::from_ymd(2020, 1, 22),
        TimeZone::Africa(Africa::Windhoek),
    );

    let tz_offset = FixedOffset::east(2 * 60 * 60);
    let orig_time: DateTime::<FixedOffset> = tz_offset.from_local_datetime(&NaiveDate::from_ymd(2020, 1, 22).and_hms(19, 41,13)).unwrap();
    if tz > orig_time + Duration::minutes(5) || tz < orig_time - Duration::minutes(5) {
        panic!("{} {} {:?} is off {:?}", file!(), line!(), tz, orig_time);
    }
}

#[test]
fn check_zmanim_cairo() {
    use zmanim::prelude::tz::*;
    let tz = get(
        Zmanim::Sunset,
        30.044,
        31.2357,
        NaiveDate::from_ymd(2020, 1, 22),
        TimeZone::Africa(Africa::Cairo)
    );
    let tz_offset = FixedOffset::east(2 * 60 * 60);
    let orig_time: DateTime::<FixedOffset> = tz_offset.from_local_datetime(&NaiveDate::from_ymd(2020, 1, 22).and_hms(17, 22, 49)).unwrap();
    if tz > orig_time + Duration::minutes(5) || tz < orig_time - Duration::minutes(5) {
        panic!("{} {} {:?} is off {:?}", file!(), line!(), tz, orig_time);
    }
}

#[test]
fn check_zmanim_madagascar() {
    use zmanim::prelude::tz::*;
    let tz = get(
        Zmanim::Sunset,
        -18.8792,
        47.5079,
        NaiveDate::from_ymd(2020, 1, 22),
        TimeZone::Africa(Africa::Nairobi)
    );

    let tz_offset = FixedOffset::east(3 * 60 * 60);
    let orig_time: DateTime::<FixedOffset> = tz_offset.from_local_datetime(&NaiveDate::from_ymd(2020, 1, 22).and_hms(18, 33, 22)).unwrap();
    if tz > orig_time + Duration::minutes(5) || tz < orig_time - Duration::minutes(5) {
        panic!("{} {} {:?} is off {:?}", file!(), line!(), tz, orig_time);
    }
}
