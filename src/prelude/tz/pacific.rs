use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Pacific::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString)]
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
            Self::Apia => Apia.from_local_datetime(datetime).unwrap(),
            Self::Auckland => Auckland.from_local_datetime(datetime).unwrap(),
            Self::Bougainville => Bougainville.from_local_datetime(datetime).unwrap(),
            Self::Chatham => Chatham.from_local_datetime(datetime).unwrap(),
            Self::Chuuk => Chuuk.from_local_datetime(datetime).unwrap(),
            Self::Easter => Easter.from_local_datetime(datetime).unwrap(),
            Self::Efate => Efate.from_local_datetime(datetime).unwrap(),
            Self::Enderbury => Enderbury.from_local_datetime(datetime).unwrap(),
            Self::Fakaofo => Fakaofo.from_local_datetime(datetime).unwrap(),
            Self::Fiji => Fiji.from_local_datetime(datetime).unwrap(),
            Self::Funafuti => Funafuti.from_local_datetime(datetime).unwrap(),
            Self::Galapagos => Galapagos.from_local_datetime(datetime).unwrap(),
            Self::Gambier => Gambier.from_local_datetime(datetime).unwrap(),
            Self::Guadalcanal => Guadalcanal.from_local_datetime(datetime).unwrap(),
            Self::Guam => Guam.from_local_datetime(datetime).unwrap(),
            Self::Honolulu => Honolulu.from_local_datetime(datetime).unwrap(),
            Self::Johnston => Johnston.from_local_datetime(datetime).unwrap(),
            Self::Kiritimati => Kiritimati.from_local_datetime(datetime).unwrap(),
            Self::Kosrae => Kosrae.from_local_datetime(datetime).unwrap(),
            Self::Kwajalein => Kwajalein.from_local_datetime(datetime).unwrap(),
            Self::Majuro => Majuro.from_local_datetime(datetime).unwrap(),
            Self::Marquesas => Marquesas.from_local_datetime(datetime).unwrap(),
            Self::Midway => Midway.from_local_datetime(datetime).unwrap(),
            Self::Nauru => Nauru.from_local_datetime(datetime).unwrap(),
            Self::Niue => Niue.from_local_datetime(datetime).unwrap(),
            Self::Norfolk => Norfolk.from_local_datetime(datetime).unwrap(),
            Self::Noumea => Noumea.from_local_datetime(datetime).unwrap(),
            Self::PagoPago => Pago_Pago.from_local_datetime(datetime).unwrap(),
            Self::Palau => Palau.from_local_datetime(datetime).unwrap(),
            Self::Pitcairn => Pitcairn.from_local_datetime(datetime).unwrap(),
            Self::Pohnpei => Pohnpei.from_local_datetime(datetime).unwrap(),
            Self::Ponape => Ponape.from_local_datetime(datetime).unwrap(),
            Self::PortMoresby => Port_Moresby.from_local_datetime(datetime).unwrap(),
            Self::Rarotonga => Rarotonga.from_local_datetime(datetime).unwrap(),
            Self::Saipan => Saipan.from_local_datetime(datetime).unwrap(),
            Self::Samoa => Samoa.from_local_datetime(datetime).unwrap(),
            Self::Tahiti => Tahiti.from_local_datetime(datetime).unwrap(),
            Self::Tarawa => Tarawa.from_local_datetime(datetime).unwrap(),
            Self::Tongatapu => Tongatapu.from_local_datetime(datetime).unwrap(),
            Self::Truk => Truk.from_local_datetime(datetime).unwrap(),
            Self::Wake => Wake.from_local_datetime(datetime).unwrap(),
            Self::Wallis => Wallis.from_local_datetime(datetime).unwrap(),
            Self::Yap => Yap.from_local_datetime(datetime).unwrap(),
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
