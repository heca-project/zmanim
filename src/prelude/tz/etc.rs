use crate::prelude::tz::*;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
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
}
