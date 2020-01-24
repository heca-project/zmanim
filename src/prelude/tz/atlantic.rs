use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Atlantic::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString)]
pub enum Atlantic {
    Azores,
    Bermuda,
    Canary,
    #[strum(serialize = "Cape_Verde")]
    CapeVerde,
    Faeroe,
    Faroe,
    #[strum(serialize = "Jan_Mayen")]
    JanMayen,
    Madeira,
    Reykjavik,
    #[strum(serialize = "South_Georgia")]
    SouthGeorgia,
    #[strum(serialize = "St_Helena")]
    StHelena,
    Stanley,
}
impl Atlantic {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        Self::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Self::Azores => Azores.from_local_datetime(datetime).unwrap(),
            Self::Bermuda => Bermuda.from_local_datetime(datetime).unwrap(),
            Self::Canary => Canary.from_local_datetime(datetime).unwrap(),
            Self::CapeVerde => Cape_Verde.from_local_datetime(datetime).unwrap(),
            Self::Faeroe => Faeroe.from_local_datetime(datetime).unwrap(),
            Self::Faroe => Faroe.from_local_datetime(datetime).unwrap(),
            Self::JanMayen => Jan_Mayen.from_local_datetime(datetime).unwrap(),
            Self::Madeira => Madeira.from_local_datetime(datetime).unwrap(),
            Self::Reykjavik => Reykjavik.from_local_datetime(datetime).unwrap(),
            Self::SouthGeorgia => South_Georgia.from_local_datetime(datetime).unwrap(),
            Self::StHelena => St_Helena.from_local_datetime(datetime).unwrap(),
            Self::Stanley => Stanley.from_local_datetime(datetime).unwrap(),
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
