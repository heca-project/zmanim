use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Antarctica::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString)]
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
            Self::Casey => Casey.from_local_datetime(datetime).unwrap(),
            Self::Davis => Davis.from_local_datetime(datetime).unwrap(),
            Self::DumontDUrville => DumontDUrville.from_local_datetime(datetime).unwrap(),
            Self::Macquarie => Macquarie.from_local_datetime(datetime).unwrap(),
            Self::Mawson => Mawson.from_local_datetime(datetime).unwrap(),
            Self::McMurdo => McMurdo.from_local_datetime(datetime).unwrap(),
            Self::Palmer => Palmer.from_local_datetime(datetime).unwrap(),
            Self::Rothera => Rothera.from_local_datetime(datetime).unwrap(),
            Self::SouthPole => South_Pole.from_local_datetime(datetime).unwrap(),
            Self::Syowa => Syowa.from_local_datetime(datetime).unwrap(),
            Self::Troll => Troll.from_local_datetime(datetime).unwrap(),
            Self::Vostok => Vostok.from_local_datetime(datetime).unwrap(),
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
