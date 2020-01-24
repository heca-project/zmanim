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
            Self::Pacific => Pacific.from_local_datetime(datetime).unwrap(),
            Self::Alaska => Alaska.from_local_datetime(datetime).unwrap(),
            Self::Aleutian => Aleutian.from_local_datetime(datetime).unwrap(),
            Self::Arizona => Arizona.from_local_datetime(datetime).unwrap(),
            Self::Central => Central.from_local_datetime(datetime).unwrap(),
            Self::EastIndiana => EastIndiana.from_local_datetime(datetime).unwrap(),
            Self::Eastern => Eastern.from_local_datetime(datetime).unwrap(),
            Self::Hawaii => Hawaii.from_local_datetime(datetime).unwrap(),
            Self::IndianaStarke => IndianaStarke.from_local_datetime(datetime).unwrap(),
            Self::Michigan => Michigan.from_local_datetime(datetime).unwrap(),
            Self::Mountain => Mountain.from_local_datetime(datetime).unwrap(),
            Self::PacificNew => PacificNew.from_local_datetime(datetime).unwrap(),
            Self::Samoa => Samoa.from_local_datetime(datetime).unwrap(),
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
