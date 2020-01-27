use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Etc::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
pub enum Etc {
    GMT,
    GMT0,
    #[strum(serialize = "GMT-0")]
    GMTMinus0,
    #[strum(serialize = "GMT-1")]
    GMTMinus1,
    #[strum(serialize = "GMT-2")]
    GMTMinus2,
    #[strum(serialize = "GMT-3")]
    GMTMinus3,
    #[strum(serialize = "GMT-4")]
    GMTMinus4,
    #[strum(serialize = "GMT-5")]
    GMTMinus5,
    #[strum(serialize = "GMT-6")]
    GMTMinus6,
    #[strum(serialize = "GMT-7")]
    GMTMinus7,
    #[strum(serialize = "GMT-8")]
    GMTMinus8,
    #[strum(serialize = "GMT-9")]
    GMTMinus9,
    #[strum(serialize = "GMT-10")]
    GMTMinus10,
    #[strum(serialize = "GMT-11")]
    GMTMinus11,
    #[strum(serialize = "GMT-12")]
    GMTMinus12,
    #[strum(serialize = "GMT-13")]
    GMTMinus13,
    #[strum(serialize = "GMT-14")]
    GMTMinus14,
    #[strum(serialize = "GMT+0")]
    GMTPlus0,
    #[strum(serialize = "GMT+1")]
    GMTPlus1,
    #[strum(serialize = "GMT+2")]
    GMTPlus2,
    #[strum(serialize = "GMT+3")]
    GMTPlus3,
    #[strum(serialize = "GMT+4")]
    GMTPlus4,
    #[strum(serialize = "GMT+5")]
    GMTPlus5,
    #[strum(serialize = "GMT+6")]
    GMTPlus6,
    #[strum(serialize = "GMT+7")]
    GMTPlus7,
    #[strum(serialize = "GMT+8")]
    GMTPlus8,
    #[strum(serialize = "GMT+9")]
    GMTPlus9,
    #[strum(serialize = "GMT+10")]
    GMTPlus10,
    #[strum(serialize = "GMT+11")]
    GMTPlus11,
    #[strum(serialize = "GMT+12")]
    GMTPlus12,
    Greenwich,
    UCT,
    UTC,
    Universal,
    Zulu,
}
impl Etc {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        Self::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Self::GMT => GMT.from_utc_datetime(datetime),
            Self::GMT0 => GMT0.from_utc_datetime(datetime),
            Self::GMTMinus0 => GMTMinus0.from_utc_datetime(datetime),
            Self::GMTMinus1 => GMTMinus1.from_utc_datetime(datetime),
            Self::GMTMinus2 => GMTMinus2.from_utc_datetime(datetime),
            Self::GMTMinus3 => GMTMinus3.from_utc_datetime(datetime),
            Self::GMTMinus4 => GMTMinus4.from_utc_datetime(datetime),
            Self::GMTMinus5 => GMTMinus5.from_utc_datetime(datetime),
            Self::GMTMinus6 => GMTMinus6.from_utc_datetime(datetime),
            Self::GMTMinus7 => GMTMinus7.from_utc_datetime(datetime),
            Self::GMTMinus8 => GMTMinus8.from_utc_datetime(datetime),
            Self::GMTMinus9 => GMTMinus9.from_utc_datetime(datetime),
            Self::GMTMinus10 => GMTMinus10.from_utc_datetime(datetime),
            Self::GMTMinus11 => GMTMinus11.from_utc_datetime(datetime),
            Self::GMTMinus12 => GMTMinus12.from_utc_datetime(datetime),
            Self::GMTMinus13 => GMTMinus13.from_utc_datetime(datetime),
            Self::GMTMinus14 => GMTMinus14.from_utc_datetime(datetime),
            Self::GMTPlus0 => GMTPlus0.from_utc_datetime(datetime),
            Self::GMTPlus1 => GMTPlus1.from_utc_datetime(datetime),
            Self::GMTPlus2 => GMTPlus2.from_utc_datetime(datetime),
            Self::GMTPlus3 => GMTPlus3.from_utc_datetime(datetime),
            Self::GMTPlus4 => GMTPlus4.from_utc_datetime(datetime),
            Self::GMTPlus5 => GMTPlus5.from_utc_datetime(datetime),
            Self::GMTPlus6 => GMTPlus6.from_utc_datetime(datetime),
            Self::GMTPlus7 => GMTPlus7.from_utc_datetime(datetime),
            Self::GMTPlus8 => GMTPlus8.from_utc_datetime(datetime),
            Self::GMTPlus9 => GMTPlus9.from_utc_datetime(datetime),
            Self::GMTPlus10 => GMTPlus10.from_utc_datetime(datetime),
            Self::GMTPlus11 => GMTPlus11.from_utc_datetime(datetime),
            Self::GMTPlus12 => GMTPlus12.from_utc_datetime(datetime),
            Self::Greenwich => Greenwich.from_utc_datetime(datetime),
            Self::UCT => UCT.from_utc_datetime(datetime),
            Self::UTC => UTC.from_utc_datetime(datetime),
            Self::Universal => Universal.from_utc_datetime(datetime),
            Self::Zulu => Zulu.from_utc_datetime(datetime),
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
