use crate::prelude::tz::*;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Europe {
    Amsterdam,
    Andorra,
    Astrakhan,
    Athens,
    Belfast,
    Belgrade,
    Berlin,
    Bratislava,
    Brussels,
    Bucharest,
    Budapest,
    Busingen,
    Chisinau,
    Copenhagen,
    Dublin,
    Gibraltar,
    Guernsey,
    Helsinki,
    #[strum(serialize = "Isle_of_Man")]
    IsleOfMan,
    Istanbul,
    Jersey,
    Kaliningrad,
    Kiev,
    Kirov,
    Lisbon,
    Ljubljana,
    London,
    Luxembourg,
    Madrid,
    Malta,
    Mariehamn,
    Minsk,
    Monaco,
    Moscow,
    Nicosia,
    Oslo,
    Paris,
    Podgorica,
    Prague,
    Riga,
    Rome,
    Samara,
    #[strum(serialize = "San_Marino")]
    SanMarino,
    Sarajevo,
    Saratov,
    Simferopol,
    Skopje,
    Sofia,
    Stockholm,
    Tallinn,
    Tirane,
    Tiraspol,
    Ulyanovsk,
    Uzhgorod,
    Vaduz,
    Vatican,
    Vienna,
    Vilnius,
    Volgograd,
    Warsaw,
    Zagreb,
    Zaporozhye,
    Zurich,
}

impl Europe {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        Self::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
}
