use crate::prelude::tz::*;
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
            other => {
                Self::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
            }
        }
    }
}
