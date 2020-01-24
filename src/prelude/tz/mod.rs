use crate::prelude::tz::africa::Africa;
use crate::prelude::tz::america::America;
use crate::prelude::tz::antarctica::Antarctica;
use crate::prelude::tz::arctic::Arctic;
use crate::prelude::tz::asia::Asia;
use crate::prelude::tz::atlantic::Atlantic;
use crate::prelude::tz::australia::Australia;
use crate::prelude::tz::brazil::Brazil;
use crate::prelude::tz::canada::Canada;
use crate::prelude::tz::chile::Chile;
use crate::prelude::tz::etc::Etc;
use crate::prelude::tz::europe::Europe;
use crate::prelude::tz::indian::Indian;
use crate::prelude::tz::mexico::Mexico;
use crate::prelude::tz::pacific::Pacific;
use crate::prelude::tz::us::US;
use chrono::{FixedOffset, NaiveDateTime};
use serde::Deserialize;
use std::convert::TryFrom;
use std::fmt;
pub mod africa;
pub mod america;
pub mod antarctica;
pub mod arctic;
pub mod asia;
pub mod atlantic;
pub mod australia;
pub mod brazil;
pub mod canada;
pub mod chile;
pub mod etc;
pub mod europe;
pub mod indian;
pub mod mexico;
pub mod pacific;
pub mod us;
#[non_exhaustive]
#[derive(Deserialize, Debug)]
#[serde(try_from = "&str")]
pub enum TimeZone {
    Africa(africa::Africa),
    America(america::America),
    Antarctica(antarctica::Antarctica),
    Arctic(arctic::Arctic),
    Asia(asia::Asia),
    Atlantic(atlantic::Atlantic),
    Australia(australia::Australia),
    Brazil(brazil::Brazil),
    Canada(canada::Canada),
    Chile(chile::Chile),
    Etc(etc::Etc),
    Europe(europe::Europe),
    Indian(indian::Indian),
    Mexico(mexico::Mexico),
    Pacific(pacific::Pacific),
    US(us::US),
    CET,
    CST6CDT,
    Cuba,
    EET,
    Egypt,
    Eire,
    EST,
    EST5EDT,
    Factory,
    GB,
    GBEire,
    GMT,
    GMTPlus0,
    GMT0,
    GMTMinus0,
    Greenwich,
    Hongkong,
    HST,
    Iceland,
    Iran,
    Israel,
    Jamaica,
    Japan,
    Kwajalein,
    Libya,
    MET,
    MST,
    MST7MDT,
    Navajo,
    NZ,
    NZCHAT,
    Poland,
    Portugal,
    PRC,
    PST8PDT,
    ROC,
    ROK,
    Singapore,
    Turkey,
    UCT,
    Universal,
    UTC,
    WET,
    WSu,
    Zulu,
}
#[derive(Debug)]
pub enum Error {
    WrongTimeZone(String),
    TooManyElements(usize),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongTimeZone(wtz) => write!(f, "TimeZone \"{}\" not understood", wtz),
            Self::TooManyElements(e) => write!(f, "TimeZone nested too deeply (\"{}\")", e),
        }
    }
}
impl TimeZone {
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        match self {
            Self::Africa(africa) => africa.get_tz(datetime),
            Self::America(america) => america.get_tz(datetime),
            Self::Antarctica(antarctica) => antarctica.get_tz(datetime),
            Self::Arctic(arctic) => arctic.get_tz(datetime),
            Self::Asia(asia) => asia.get_tz(datetime),
            Self::Atlantic(atlantic) => atlantic.get_tz(datetime),
            Self::Australia(australia) => australia.get_tz(datetime),
            Self::Brazil(brazil) => brazil.get_tz(datetime),
            Self::Canada(canada) => canada.get_tz(datetime),
            Self::Chile(chile) => chile.get_tz(datetime),
            Self::Etc(etc) => etc.get_tz(datetime),
            Self::Europe(europe) => europe.get_tz(datetime),
            Self::Indian(indian) => indian.get_tz(datetime),
            Self::Mexico(mexico) => mexico.get_tz(datetime),
            Self::Pacific(pacific) => pacific.get_tz(datetime),
            Self::US(us) => us.get_tz(datetime),
            _ => {
                todo!();
            }
        }
    }
}
impl TryFrom<&str> for TimeZone {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let p: Vec<&str> = value.split('/').collect();
        match p[0] {
            "Africa" => Ok(Self::Africa(Africa::try_from_path(&p[1..])?)),
            "America" => Ok(Self::America(America::try_from_path(&p[1..])?)),
            "Antarctica" => Ok(Self::Antarctica(Antarctica::try_from_path(&p[1..])?)),
            "Arctic" => Ok(Self::Arctic(Arctic::try_from_path(&p[1..])?)),
            "Asia" => Ok(Self::Asia(Asia::try_from_path(&p[1..])?)),
            "Atlantic" => Ok(Self::Atlantic(Atlantic::try_from_path(&p[1..])?)),
            "Australia" => Ok(Self::Australia(Australia::try_from_path(&p[1..])?)),
            "Brazil" => Ok(Self::Brazil(Brazil::try_from_path(&p[1..])?)),
            "Canada" => Ok(Self::Canada(Canada::try_from_path(&p[1..])?)),
            "Chile" => Ok(Self::Chile(Chile::try_from_path(&p[1..])?)),
            "Etc" => Ok(Self::Etc(Etc::try_from_path(&p[1..])?)),
            "Europe" => Ok(Self::Europe(Europe::try_from_path(&p[1..])?)),
            "Indian" => Ok(Self::Indian(Indian::try_from_path(&p[1..])?)),
            "Mexico" => Ok(Self::Mexico(Mexico::try_from_path(&p[1..])?)),
            "Pacific" => Ok(Self::Pacific(Pacific::try_from_path(&p[1..])?)),
            "US" => Ok(Self::US(US::try_from_path(&p[1..])?)),
            "CET" => Ok(Self::CET),
            "Cuba" => Ok(Self::Cuba),
            "CST6CDT" => Ok(Self::CST6CDT),
            "EET" => Ok(Self::EET),
            "Egypt" => Ok(Self::Egypt),
            "Eire" => Ok(Self::Eire),
            "EST" => Ok(Self::EST),
            "EST5EDT" => Ok(Self::EST5EDT),
            "Factory" => Ok(Self::Factory),
            "GB" => Ok(Self::GB),
            "GB-Eire" => Ok(Self::GBEire),
            "GMT" => Ok(Self::GMT),
            "GMT+0" => Ok(Self::GMTPlus0),
            "GMT0" => Ok(Self::GMT0),
            "GMT-0" => Ok(Self::GMTMinus0),
            "Greenwich" => Ok(Self::Greenwich),
            "Hongkong" => Ok(Self::Hongkong),
            "HST" => Ok(Self::HST),
            "Iceland" => Ok(Self::Iceland),
            "Iran" => Ok(Self::Iran),
            "Israel" => Ok(Self::Israel),
            "Jamaica" => Ok(Self::Jamaica),
            "Japan" => Ok(Self::Japan),
            "Kwajalein" => Ok(Self::Kwajalein),
            "Libya" => Ok(Self::Libya),
            "MET" => Ok(Self::MET),
            "MST" => Ok(Self::MST),
            "MST7MDT" => Ok(Self::MST7MDT),
            "Navajo" => Ok(Self::Navajo),
            "NZ" => Ok(Self::NZ),
            "NZ-CHAT" => Ok(Self::NZCHAT),
            "Poland" => Ok(Self::Poland),
            "Portugal" => Ok(Self::Portugal),
            "PRC" => Ok(Self::PRC),
            "PST8PDT" => Ok(Self::PST8PDT),
            "ROC" => Ok(Self::ROC),
            "ROK" => Ok(Self::ROK),
            "Singapore" => Ok(Self::Singapore),
            "Turkey" => Ok(Self::Turkey),
            "UCT" => Ok(Self::UCT),
            "Universal" => Ok(Self::Universal),
            "UTC" => Ok(Self::UTC),
            "WET" => Ok(Self::WET),
            "W-SU" => Ok(Self::WSu),
            "Zulu" => Ok(Self::Zulu),
            tz => Err(Error::WrongTimeZone(tz.to_string())),
        }
    }
}
