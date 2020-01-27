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
            Self::Antananarivo => Antananarivo.from_utc_datetime(datetime),
            Self::Chagos => Chagos.from_utc_datetime(datetime),
            Self::Christmas => Christmas.from_utc_datetime(datetime),
            Self::Cocos => Cocos.from_utc_datetime(datetime),
            Self::Comoro => Comoro.from_utc_datetime(datetime),
            Self::Kerguelen => Kerguelen.from_utc_datetime(datetime),
            Self::Mahe => Mahe.from_utc_datetime(datetime),
            Self::Maldives => Maldives.from_utc_datetime(datetime),
            Self::Mauritius => Mauritius.from_utc_datetime(datetime),
            Self::Mayotte => Mayotte.from_utc_datetime(datetime),
            Self::Reunion => Reunion.from_utc_datetime(datetime),
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
