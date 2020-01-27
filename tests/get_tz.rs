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
    use zmanim::prelude::tz::africa::*;
    use zmanim::prelude::tz::*;
    let tz = get(
        &Zmanim::Sunset,
        -22.0,
        17.00,
        NaiveDate::from_ymd(2020, 1, 22),
        &TimeZone::Africa(Africa::Windhoek),
    )
    .unwrap();

    let tz_offset = FixedOffset::east(2 * 60 * 60);
    let orig_time: DateTime<FixedOffset> = tz_offset
        .from_local_datetime(&NaiveDate::from_ymd(2020, 1, 22).and_hms(19, 41, 13))
        .unwrap();
    if tz > orig_time + Duration::minutes(5) || tz < orig_time - Duration::minutes(5) {
        panic!("{} {} {:?} is off {:?}", file!(), line!(), tz, orig_time);
    }
}

#[test]
fn check_zmanim_cairo() {
    use zmanim::prelude::tz::africa::*;
    use zmanim::prelude::tz::*;
    let tz = get(
        &Zmanim::Sunset,
        30.044,
        31.2357,
        NaiveDate::from_ymd(2020, 1, 22),
        &TimeZone::Africa(Africa::Cairo),
    )
    .unwrap();
    let tz_offset = FixedOffset::east(2 * 60 * 60);
    let orig_time: DateTime<FixedOffset> = tz_offset
        .from_local_datetime(&NaiveDate::from_ymd(2020, 1, 22).and_hms(17, 22, 49))
        .unwrap();
    if tz > orig_time + Duration::minutes(5) || tz < orig_time - Duration::minutes(5) {
        panic!("{} {} {:?} is off {:?}", file!(), line!(), tz, orig_time);
    }
    let _tz = get(
        &Zmanim::Sunset,
        30.044,
        31.2357,
        NaiveDate::from_ymd(2020, 1, 3),
        &TimeZone::Africa(Africa::Cairo),
    )
    .unwrap();
}

#[test]
fn check_zmanim_madagascar() {
    use zmanim::prelude::tz::africa::*;
    use zmanim::prelude::tz::*;
    let tz = get(
        &Zmanim::Sunset,
        -18.8792,
        47.5079,
        NaiveDate::from_ymd(2020, 1, 22),
        &TimeZone::Africa(Africa::Nairobi),
    )
    .unwrap();

    let tz_offset = FixedOffset::east(3 * 60 * 60);
    let orig_time: DateTime<FixedOffset> = tz_offset
        .from_local_datetime(&NaiveDate::from_ymd(2020, 1, 22).and_hms(18, 33, 22))
        .unwrap();
    if tz > orig_time + Duration::minutes(5) || tz < orig_time - Duration::minutes(5) {
        panic!("{} {} {:?} is off {:?}", file!(), line!(), tz, orig_time);
    }
}

#[test]
fn check_zmanim_nyc() {
    use zmanim::prelude::tz::america::*;
    use zmanim::prelude::tz::*;
    let tz = get(
        &Zmanim::Sunset,
        40.7128,
        -74.0060,
        NaiveDate::from_ymd(2020, 1, 22),
        &TimeZone::America(America::NewYork),
    )
    .unwrap();

    let tz_offset = FixedOffset::west(5 * 60 * 60);
    let orig_time: DateTime<FixedOffset> = tz_offset
        .from_local_datetime(&NaiveDate::from_ymd(2020, 1, 22).and_hms(17, 0, 0))
        .unwrap();
    if tz > orig_time + Duration::minutes(5) || tz < orig_time - Duration::minutes(5) {
        panic!("{} {} {:?} is off {:?}", file!(), line!(), tz, orig_time);
    }
}

#[test]
fn check_zmanim_south_pole() {
    use zmanim::prelude::tz::america::*;
    use zmanim::prelude::tz::*;
    let orig_date = NaiveDate::from_ymd(2020, 1, 22);
    for i in 0..365 {
        let tz = get(
            &Zmanim::Sunset,
            65.000,
            45.0000,
            orig_date + Duration::days(i),
            &TimeZone::America(America::NewYork),
        );
        assert_ne!(tz, None);
    }
}

#[test]
fn check_zmanim_helsinki() {
    use zmanim::prelude::tz::europe::*;
    use zmanim::prelude::tz::*;
    let tz = get(
        &Zmanim::Sunset,
        60.17,
        24.97,
        NaiveDate::from_ymd(2020, 1, 23),
        &TimeZone::Europe(Europe::Helsinki),
    )
    .unwrap();

    let tz_offset = FixedOffset::east(2 * 60 * 60);
    let orig_time: DateTime<FixedOffset> = tz_offset
        .from_local_datetime(&NaiveDate::from_ymd(2020, 1, 23).and_hms(16, 08, 0))
        .unwrap();
    if tz > orig_time + Duration::minutes(5) || tz < orig_time - Duration::minutes(5) {
        panic!("{} {} {:?} is off {:?}", file!(), line!(), tz, orig_time);
    }
}

#[test]
fn check_nyc_panic() {
    use zmanim::prelude::tz::america::*;
    use zmanim::prelude::tz::*;
    for i in 0..100_000 {
        let d = NaiveDate::from_ymd(1800, 1, 1) + Duration::days(i);
        let tz = get(
            &Zmanim::Sunset,
            40.7128,
            -74.0060,
            d,
            &TimeZone::America(America::NewYork),
        )
        .unwrap();
        eprintln!("{:?}", tz);
    }
}

#[test]
fn check_losangeles_panic() {
    use zmanim::prelude::tz::america::*;
    use zmanim::prelude::tz::*;
    for i in 0..100_000 {
        let d = NaiveDate::from_ymd(1800, 1, 1) + Duration::days(i);
        let tz = get(
            &Zmanim::Sunset,
            34.0522,
            -118.2437,
            d,
            &TimeZone::America(America::LosAngeles),
        )
        .unwrap();
        eprintln!("{:?}", tz);
    }
}
