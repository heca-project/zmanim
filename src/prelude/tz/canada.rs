use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Canada::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString)]
pub enum Canada {
    Atlantic,
    Central,
    Eastern,
    Mountain,
    Newfoundland,
    Pacific,
    Saskatchewan,
    Yukon,
}
impl Canada {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        Self::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Self::Atlantic => Atlantic.from_local_datetime(datetime).unwrap(),
            Self::Central => Central.from_local_datetime(datetime).unwrap(),
            Self::Eastern => Eastern.from_local_datetime(datetime).unwrap(),
            Self::Mountain => Mountain.from_local_datetime(datetime).unwrap(),
            Self::Newfoundland => Newfoundland.from_local_datetime(datetime).unwrap(),
            Self::Pacific => Pacific.from_local_datetime(datetime).unwrap(),
            Self::Saskatchewan => Saskatchewan.from_local_datetime(datetime).unwrap(),
            Self::Yukon => Yukon.from_local_datetime(datetime).unwrap(),
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
