use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::US::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
pub enum US {
    Alaska,
    Aleutian,
    Arizona,
    Central,
    #[strum(serialize = "East-Indiana")]
    EastIndiana,
    Eastern,
    Hawaii,
    #[strum(serialize = "Indiana-Starke")]
    IndianaStarke,
    Michigan,
    Mountain,
    Pacific,
    #[strum(serialize = "Pacific-New")]
    PacificNew,
    Samoa,
}
impl US {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        Self::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Self::Pacific => Pacific.from_utc_datetime(datetime),
            Self::Alaska => Alaska.from_utc_datetime(datetime),
            Self::Aleutian => Aleutian.from_utc_datetime(datetime),
            Self::Arizona => Arizona.from_utc_datetime(datetime),
            Self::Central => Central.from_utc_datetime(datetime),
            Self::EastIndiana => EastIndiana.from_utc_datetime(datetime),
            Self::Eastern => Eastern.from_utc_datetime(datetime),
            Self::Hawaii => Hawaii.from_utc_datetime(datetime),
            Self::IndianaStarke => IndianaStarke.from_utc_datetime(datetime),
            Self::Michigan => Michigan.from_utc_datetime(datetime),
            Self::Mountain => Mountain.from_utc_datetime(datetime),
            Self::PacificNew => PacificNew.from_utc_datetime(datetime),
            Self::Samoa => Samoa.from_utc_datetime(datetime),
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
