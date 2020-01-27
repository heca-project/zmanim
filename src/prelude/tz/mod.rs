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
#[derive(Deserialize, Debug, Clone)]
#[serde(try_from = "&str")]
#[non_exhaustive]
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
}
#[derive(Debug)]
#[non_exhaustive]
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
            "CET" => Ok(Self::Europe(Europe::Paris)),
            "Cuba" => Ok(Self::America(America::Havana)),
            "CST6CDT" => Ok(Self::America(America::Chicago)),
            "EET" => Ok(Self::Europe(Europe::Sofia)),
            "Egypt" => Ok(Self::Africa(Africa::Cairo)),
            "Eire" => Ok(Self::Europe(Europe::Dublin)),
            "EST" => Ok(Self::America(America::Cancun)),
            "EST5EDT" => Ok(Self::America(America::NewYork)),
            "GB" => Ok(Self::Europe(Europe::London)),
            "GB-Eire" => Ok(Self::Europe(Europe::London)),
            "GMT" => Ok(Self::Etc(Etc::GMT)),
            "GMT+0" => Ok(Self::Etc(Etc::GMT)),
            "GMT0" => Ok(Self::Etc(Etc::GMT)),
            "GMT-0" => Ok(Self::Etc(Etc::GMT)),
            "Greenwich" => Ok(Self::Etc(Etc::GMT)),
            "Hongkong" => Ok(Self::Asia(Asia::HongKong)),
            "HST" => Ok(Self::Pacific(Pacific::Honolulu)),
            "Iceland" => Ok(Self::Atlantic(Atlantic::Reykjavik)),
            "Iran" => Ok(Self::Asia(Asia::Tehran)),
            "Israel" => Ok(Self::Asia(Asia::Jerusalem)),
            "Jamaica" => Ok(Self::America(America::Jamaica)),
            "Japan" => Ok(Self::Asia(Asia::Tokyo)),
            "Kwajalein" => Ok(Self::Pacific(Pacific::Kwajalein)),
            "Libya" => Ok(Self::Africa(Africa::Tripoli)),
            "MET" => Ok(Self::Europe(Europe::Paris)),
            "MST" => Ok(Self::America(America::Phoenix)),
            "MST7MDT" => Ok(Self::America(America::Denver)),
            "Navajo" => Ok(Self::America(America::Denver)),
            "NZ" => Ok(Self::Pacific(Pacific::Auckland)),
            "NZ-CHAT" => Ok(Self::Pacific(Pacific::Chatham)),
            "Poland" => Ok(Self::Europe(Europe::Warsaw)),
            "Portugal" => Ok(Self::Europe(Europe::Lisbon)),
            "PRC" => Ok(Self::Asia(Asia::Shanghai)),
            "PST8PDT" => Ok(Self::America(America::LosAngeles)),
            "ROC" => Ok(Self::Asia(Asia::Taipei)),
            "ROK" => Ok(Self::Asia(Asia::Seoul)),
            "Singapore" => Ok(Self::Asia(Asia::Singapore)),
            "Turkey" => Ok(Self::Asia(Asia::Istanbul)),
            "UCT" => Ok(Self::Etc(Etc::UCT)),
            "Universal" => Ok(Self::Etc(Etc::UTC)),
            "UTC" => Ok(Self::Etc(Etc::UTC)),
            "WET" => Ok(Self::Europe(Europe::Lisbon)),
            "W-SU" => Ok(Self::Europe(Europe::Moscow)),
            "Zulu" => Ok(Self::Etc(Etc::UTC)),
            tz => Err(Error::WrongTimeZone(tz.to_string())),
        }
    }
}
