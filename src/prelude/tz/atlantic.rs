use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Atlantic::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
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
            Self::Azores => Azores.from_utc_datetime(datetime),
            Self::Bermuda => Bermuda.from_utc_datetime(datetime),
            Self::Canary => Canary.from_utc_datetime(datetime),
            Self::CapeVerde => Cape_Verde.from_utc_datetime(datetime),
            Self::Faeroe => Faeroe.from_utc_datetime(datetime),
            Self::Faroe => Faroe.from_utc_datetime(datetime),
            Self::JanMayen => Jan_Mayen.from_utc_datetime(datetime),
            Self::Madeira => Madeira.from_utc_datetime(datetime),
            Self::Reykjavik => Reykjavik.from_utc_datetime(datetime),
            Self::SouthGeorgia => South_Georgia.from_utc_datetime(datetime),
            Self::StHelena => St_Helena.from_utc_datetime(datetime),
            Self::Stanley => Stanley.from_utc_datetime(datetime),
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
