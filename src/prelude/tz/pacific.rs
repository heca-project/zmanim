use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Pacific::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
pub enum Pacific {
    Apia,
    Auckland,
    Bougainville,
    Chatham,
    Chuuk,
    Easter,
    Efate,
    Enderbury,
    Fakaofo,
    Fiji,
    Funafuti,
    Galapagos,
    Gambier,
    Guadalcanal,
    Guam,
    Honolulu,
    Johnston,
    Kiritimati,
    Kosrae,
    Kwajalein,
    Majuro,
    Marquesas,
    Midway,
    Nauru,
    Niue,
    Norfolk,
    Noumea,
    #[strum(serialize = "Pago_Pago")]
    PagoPago,
    Palau,
    Pitcairn,
    Pohnpei,
    Ponape,
    #[strum(serialize = "Port_Moresby")]
    PortMoresby,
    Rarotonga,
    Saipan,
    Samoa,
    Tahiti,
    Tarawa,
    Tongatapu,
    Truk,
    Wake,
    Wallis,
    Yap,
}
impl Pacific {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        Self::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Self::Apia => Apia.from_utc_datetime(datetime),
            Self::Auckland => Auckland.from_utc_datetime(datetime),
            Self::Bougainville => Bougainville.from_utc_datetime(datetime),
            Self::Chatham => Chatham.from_utc_datetime(datetime),
            Self::Chuuk => Chuuk.from_utc_datetime(datetime),
            Self::Easter => Easter.from_utc_datetime(datetime),
            Self::Efate => Efate.from_utc_datetime(datetime),
            Self::Enderbury => Enderbury.from_utc_datetime(datetime),
            Self::Fakaofo => Fakaofo.from_utc_datetime(datetime),
            Self::Fiji => Fiji.from_utc_datetime(datetime),
            Self::Funafuti => Funafuti.from_utc_datetime(datetime),
            Self::Galapagos => Galapagos.from_utc_datetime(datetime),
            Self::Gambier => Gambier.from_utc_datetime(datetime),
            Self::Guadalcanal => Guadalcanal.from_utc_datetime(datetime),
            Self::Guam => Guam.from_utc_datetime(datetime),
            Self::Honolulu => Honolulu.from_utc_datetime(datetime),
            Self::Johnston => Johnston.from_utc_datetime(datetime),
            Self::Kiritimati => Kiritimati.from_utc_datetime(datetime),
            Self::Kosrae => Kosrae.from_utc_datetime(datetime),
            Self::Kwajalein => Kwajalein.from_utc_datetime(datetime),
            Self::Majuro => Majuro.from_utc_datetime(datetime),
            Self::Marquesas => Marquesas.from_utc_datetime(datetime),
            Self::Midway => Midway.from_utc_datetime(datetime),
            Self::Nauru => Nauru.from_utc_datetime(datetime),
            Self::Niue => Niue.from_utc_datetime(datetime),
            Self::Norfolk => Norfolk.from_utc_datetime(datetime),
            Self::Noumea => Noumea.from_utc_datetime(datetime),
            Self::PagoPago => Pago_Pago.from_utc_datetime(datetime),
            Self::Palau => Palau.from_utc_datetime(datetime),
            Self::Pitcairn => Pitcairn.from_utc_datetime(datetime),
            Self::Pohnpei => Pohnpei.from_utc_datetime(datetime),
            Self::Ponape => Ponape.from_utc_datetime(datetime),
            Self::PortMoresby => Port_Moresby.from_utc_datetime(datetime),
            Self::Rarotonga => Rarotonga.from_utc_datetime(datetime),
            Self::Saipan => Saipan.from_utc_datetime(datetime),
            Self::Samoa => Samoa.from_utc_datetime(datetime),
            Self::Tahiti => Tahiti.from_utc_datetime(datetime),
            Self::Tarawa => Tarawa.from_utc_datetime(datetime),
            Self::Tongatapu => Tongatapu.from_utc_datetime(datetime),
            Self::Truk => Truk.from_utc_datetime(datetime),
            Self::Wake => Wake.from_utc_datetime(datetime),
            Self::Wallis => Wallis.from_utc_datetime(datetime),
            Self::Yap => Yap.from_utc_datetime(datetime),
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
