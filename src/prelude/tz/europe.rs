use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Europe::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
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
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Self::Amsterdam => Amsterdam.from_utc_datetime(datetime),
            Self::Andorra => Andorra.from_utc_datetime(datetime),
            Self::Astrakhan => Astrakhan.from_utc_datetime(datetime),
            Self::Athens => Athens.from_utc_datetime(datetime),
            Self::Belfast => Belfast.from_utc_datetime(datetime),
            Self::Belgrade => Belgrade.from_utc_datetime(datetime),
            Self::Berlin => Berlin.from_utc_datetime(datetime),
            Self::Bratislava => Bratislava.from_utc_datetime(datetime),
            Self::Brussels => Brussels.from_utc_datetime(datetime),
            Self::Bucharest => Bucharest.from_utc_datetime(datetime),
            Self::Budapest => Budapest.from_utc_datetime(datetime),
            Self::Busingen => Busingen.from_utc_datetime(datetime),
            Self::Chisinau => Chisinau.from_utc_datetime(datetime),
            Self::Copenhagen => Copenhagen.from_utc_datetime(datetime),
            Self::Dublin => Dublin.from_utc_datetime(datetime),
            Self::Gibraltar => Gibraltar.from_utc_datetime(datetime),
            Self::Guernsey => Guernsey.from_utc_datetime(datetime),
            Self::Helsinki => Helsinki.from_utc_datetime(datetime),
            Self::IsleOfMan => Isle_of_Man.from_utc_datetime(datetime),
            Self::Istanbul => Istanbul.from_utc_datetime(datetime),
            Self::Jersey => Jersey.from_utc_datetime(datetime),
            Self::Kaliningrad => Kaliningrad.from_utc_datetime(datetime),
            Self::Kiev => Kiev.from_utc_datetime(datetime),
            Self::Kirov => Kirov.from_utc_datetime(datetime),
            Self::Lisbon => Lisbon.from_utc_datetime(datetime),
            Self::Ljubljana => Ljubljana.from_utc_datetime(datetime),
            Self::London => London.from_utc_datetime(datetime),
            Self::Luxembourg => Luxembourg.from_utc_datetime(datetime),
            Self::Madrid => Madrid.from_utc_datetime(datetime),
            Self::Malta => Malta.from_utc_datetime(datetime),
            Self::Mariehamn => Mariehamn.from_utc_datetime(datetime),
            Self::Minsk => Minsk.from_utc_datetime(datetime),
            Self::Monaco => Monaco.from_utc_datetime(datetime),
            Self::Moscow => Moscow.from_utc_datetime(datetime),
            Self::Nicosia => Nicosia.from_utc_datetime(datetime),
            Self::Oslo => Oslo.from_utc_datetime(datetime),
            Self::Paris => Paris.from_utc_datetime(datetime),
            Self::Podgorica => Podgorica.from_utc_datetime(datetime),
            Self::Prague => Prague.from_utc_datetime(datetime),
            Self::Riga => Riga.from_utc_datetime(datetime),
            Self::Rome => Rome.from_utc_datetime(datetime),
            Self::Samara => Samara.from_utc_datetime(datetime),
            Self::SanMarino => San_Marino.from_utc_datetime(datetime),
            Self::Sarajevo => Sarajevo.from_utc_datetime(datetime),
            Self::Saratov => Saratov.from_utc_datetime(datetime),
            Self::Simferopol => Simferopol.from_utc_datetime(datetime),
            Self::Skopje => Skopje.from_utc_datetime(datetime),
            Self::Sofia => Sofia.from_utc_datetime(datetime),
            Self::Stockholm => Stockholm.from_utc_datetime(datetime),
            Self::Tallinn => Tallinn.from_utc_datetime(datetime),
            Self::Tirane => Tirane.from_utc_datetime(datetime),
            Self::Tiraspol => Tiraspol.from_utc_datetime(datetime),
            Self::Ulyanovsk => Ulyanovsk.from_utc_datetime(datetime),
            Self::Uzhgorod => Uzhgorod.from_utc_datetime(datetime),
            Self::Vaduz => Vaduz.from_utc_datetime(datetime),
            Self::Vatican => Vatican.from_utc_datetime(datetime),
            Self::Vienna => Vienna.from_utc_datetime(datetime),
            Self::Vilnius => Vilnius.from_utc_datetime(datetime),
            Self::Volgograd => Volgograd.from_utc_datetime(datetime),
            Self::Warsaw => Warsaw.from_utc_datetime(datetime),
            Self::Zagreb => Zagreb.from_utc_datetime(datetime),
            Self::Zaporozhye => Zaporozhye.from_utc_datetime(datetime),
            Self::Zurich => Zurich.from_utc_datetime(datetime),
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
