use crate::prelude::tz::*;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Arctic {
    Longyearbyen,
}

impl Arctic {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        Self::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
}
