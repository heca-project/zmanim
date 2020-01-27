use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Antarctica::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
pub enum Antarctica {
    Casey,
    Davis,
    DumontDUrville,
    Macquarie,
    Mawson,
    McMurdo,
    Palmer,
    Rothera,
    SouthPole,
    Syowa,
    Troll,
    Vostok,
}
impl Antarctica {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        match p[0] {
            "South_Pole" => Ok(Self::SouthPole),
            other => Self::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
        }
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Self::Casey => Casey.from_utc_datetime(datetime),
            Self::Davis => Davis.from_utc_datetime(datetime),
            Self::DumontDUrville => DumontDUrville.from_utc_datetime(datetime),
            Self::Macquarie => Macquarie.from_utc_datetime(datetime),
            Self::Mawson => Mawson.from_utc_datetime(datetime),
            Self::McMurdo => McMurdo.from_utc_datetime(datetime),
            Self::Palmer => Palmer.from_utc_datetime(datetime),
            Self::Rothera => Rothera.from_utc_datetime(datetime),
            Self::SouthPole => South_Pole.from_utc_datetime(datetime),
            Self::Syowa => Syowa.from_utc_datetime(datetime),
            Self::Troll => Troll.from_utc_datetime(datetime),
            Self::Vostok => Vostok.from_utc_datetime(datetime),
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
