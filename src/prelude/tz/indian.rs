use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Indian::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
pub enum Indian {
    Antananarivo,
    Chagos,
    Christmas,
    Cocos,
    Comoro,
    Kerguelen,
    Mahe,
    Maldives,
    Mauritius,
    Mayotte,
    Reunion,
}
impl Indian {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        Self::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Self::Antananarivo => Antananarivo.from_local_datetime(datetime).unwrap(),
            Self::Chagos => Chagos.from_local_datetime(datetime).unwrap(),
            Self::Christmas => Christmas.from_local_datetime(datetime).unwrap(),
            Self::Cocos => Cocos.from_local_datetime(datetime).unwrap(),
            Self::Comoro => Comoro.from_local_datetime(datetime).unwrap(),
            Self::Kerguelen => Kerguelen.from_local_datetime(datetime).unwrap(),
            Self::Mahe => Mahe.from_local_datetime(datetime).unwrap(),
            Self::Maldives => Maldives.from_local_datetime(datetime).unwrap(),
            Self::Mauritius => Mauritius.from_local_datetime(datetime).unwrap(),
            Self::Mayotte => Mayotte.from_local_datetime(datetime).unwrap(),
            Self::Reunion => Reunion.from_local_datetime(datetime).unwrap(),
        };
        p.timezone()
            .offset_from_utc_date(&NaiveDate::from_ymd(
                datetime.year(),
                datetime.month(),
                datetime.day(),
            ))
            .fix()
    }
}
