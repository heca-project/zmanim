use crate::prelude::tz::*;
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
}
