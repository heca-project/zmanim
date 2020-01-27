use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Brazil::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
pub enum Brazil {
    Acre,
    DeNoronha,
    East,
    West,
}
impl Brazil {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        Self::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Self::Acre => Acre.from_utc_datetime(datetime),
            Self::DeNoronha => DeNoronha.from_utc_datetime(datetime),
            Self::East => East.from_utc_datetime(datetime),
            Self::West => West.from_utc_datetime(datetime),
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
