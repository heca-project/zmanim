pub use crate::prelude::tz::africa::Africa;
pub use crate::prelude::tz::america::America;
pub use crate::prelude::tz::antarctica::Antarctica;
pub use crate::prelude::tz::arctic::Arctic;
pub use crate::prelude::tz::asia::Asia;
pub use crate::prelude::tz::atlantic::Atlantic;
pub use crate::prelude::tz::australia::Australia;
pub use crate::prelude::tz::brazil::Brazil;
pub use crate::prelude::tz::canada::Canada;
pub use crate::prelude::tz::chile::Chile;
pub use crate::prelude::tz::etc::Etc;
pub use crate::prelude::tz::europe::Europe;
pub use crate::prelude::tz::indian::Indian;
pub use crate::prelude::tz::mexico::Mexico;
pub use crate::prelude::tz::pacific::Pacific;
use crate::prelude::tz::us::US;
use chrono::{FixedOffset, NaiveDateTime};
use std::convert::TryFrom;

mod africa;
mod america;
mod antarctica;
mod arctic;
mod asia;
mod atlantic;
mod australia;
mod brazil;
mod canada;
mod chile;
mod etc;
mod europe;
mod indian;
mod mexico;
mod pacific;
mod us;

#[non_exhaustive]
#[derive(Debug)]
pub enum TimeZone {
    Africa(Africa),
    America(America),
    Antarctica(Antarctica),
    Arctic(Arctic),
    Asia(Asia),
    Atlantic(Atlantic),
    Australia(Australia),
    Brazil(Brazil),
    Canada(Canada),
    Chile(Chile),
    Etc(Etc),
    Europe(Europe),
    Indian(Indian),
    Mexico(Mexico),
    Pacific(Pacific),
    US(US),

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

impl TimeZone {
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        match self {
            Self::Africa(africa) => africa.get_tz(datetime),
            _ => unimplemented!(),
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
