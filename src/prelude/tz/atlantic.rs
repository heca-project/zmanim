use crate::prelude::tz::*;
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
        Atlantic::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
}
