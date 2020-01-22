use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::prelude::*;
use chrono::{FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Africa::*;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Africa {
    Abidjan,
    Accra,
    AddisAbaba,
    Algiers,
    Asmara,
    Asmera,
    Bamako,
    Bangui,
    Banjul,
    Bissau,
    Blantyre,
    Brazzaville,
    Bujumbura,
    Cairo,
    Casablanca,
    Ceuta,
    Conakry,
    Dakar,
    DarEsSalaam,
    Djibouti,
    Douala,
    ElAaiun,
    Freetown,
    Gaborone,
    Harare,
    Johannesburg,
    Juba,
    Kampala,
    Khartoum,
    Kigali,
    Kinshasa,
    Lagos,
    Libreville,
    Lome,
    Luanda,
    Lubumbashi,
    Lusaka,
    Malabo,
    Maputo,
    Maseru,
    Mbabane,
    Mogadishu,
    Monrovia,
    Nairobi,
    Ndjamena,
    Niamey,
    Nouakchott,
    Ouagadougou,
    PortoNovo,
    SaoTome,
    Timbuktu,
    Tripoli,
    Tunis,
    Windhoek,
}

impl Africa {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        match p[0] {
            "Addis_Ababa" => Ok(Self::AddisAbaba),
            "Dar_es_Salaam" => Ok(Self::DarEsSalaam),
            "El_Aaiun" => Ok(Self::ElAaiun),
            "Porto-Novo" => Ok(Self::PortoNovo),
            "Sao_Tome" => Ok(Self::SaoTome),
            other => Self::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
        }
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Self::Abidjan => Abidjan.from_local_datetime(datetime).unwrap(),
            Self::Windhoek => Windhoek.from_local_datetime(datetime).unwrap(),
            Self::Accra => Accra.from_local_datetime(datetime).unwrap(),
            Self::AddisAbaba => Addis_Ababa.from_local_datetime(datetime).unwrap(),
            Self::Algiers => Algiers.from_local_datetime(datetime).unwrap(),
            Self::Asmara => Asmara.from_local_datetime(datetime).unwrap(),
            Self::Asmera => Asmera.from_local_datetime(datetime).unwrap(),
            Self::Bamako => Bamako.from_local_datetime(datetime).unwrap(),
            Self::Bangui => Bangui.from_local_datetime(datetime).unwrap(),
            Self::Banjul => Banjul.from_local_datetime(datetime).unwrap(),
            Self::Bissau => Bissau.from_local_datetime(datetime).unwrap(),
            Self::Blantyre => Blantyre.from_local_datetime(datetime).unwrap(),
            Self::Brazzaville => Brazzaville.from_local_datetime(datetime).unwrap(),
            Self::Bujumbura => Bujumbura.from_local_datetime(datetime).unwrap(),
            Self::Cairo => Cairo.from_local_datetime(datetime).unwrap(),
            Self::Casablanca => Casablanca.from_local_datetime(datetime).unwrap(),
            Self::Ceuta => Ceuta.from_local_datetime(datetime).unwrap(),
            Self::Conakry => Conakry.from_local_datetime(datetime).unwrap(),
            Self::Dakar => Dakar.from_local_datetime(datetime).unwrap(),
            Self::DarEsSalaam => Dar_es_Salaam.from_local_datetime(datetime).unwrap(),
            Self::Djibouti => Djibouti.from_local_datetime(datetime).unwrap(),
            Self::Douala => Douala.from_local_datetime(datetime).unwrap(),
            Self::ElAaiun => El_Aaiun.from_local_datetime(datetime).unwrap(),
            Self::Freetown => Freetown.from_local_datetime(datetime).unwrap(),
            Self::Gaborone => Gaborone.from_local_datetime(datetime).unwrap(),
            Self::Harare => Harare.from_local_datetime(datetime).unwrap(),
            Self::Johannesburg => Johannesburg.from_local_datetime(datetime).unwrap(),
            Self::Juba => Juba.from_local_datetime(datetime).unwrap(),
            Self::Kampala => Kampala.from_local_datetime(datetime).unwrap(),
            Self::Khartoum => Khartoum.from_local_datetime(datetime).unwrap(),
            Self::Kigali => Kigali.from_local_datetime(datetime).unwrap(),
            Self::Kinshasa => Kinshasa.from_local_datetime(datetime).unwrap(),
            Self::Lagos => Lagos.from_local_datetime(datetime).unwrap(),
            Self::Libreville => Libreville.from_local_datetime(datetime).unwrap(),
            Self::Lome => Lome.from_local_datetime(datetime).unwrap(),
            Self::Luanda => Luanda.from_local_datetime(datetime).unwrap(),
            Self::Lubumbashi => Lubumbashi.from_local_datetime(datetime).unwrap(),
            Self::Lusaka => Lusaka.from_local_datetime(datetime).unwrap(),
            Self::Malabo => Malabo.from_local_datetime(datetime).unwrap(),
            Self::Maputo => Maputo.from_local_datetime(datetime).unwrap(),
            Self::Maseru => Maseru.from_local_datetime(datetime).unwrap(),
            Self::Mbabane => Mbabane.from_local_datetime(datetime).unwrap(),
            Self::Mogadishu => Mogadishu.from_local_datetime(datetime).unwrap(),
            Self::Monrovia => Monrovia.from_local_datetime(datetime).unwrap(),
            Self::Nairobi => Nairobi.from_local_datetime(datetime).unwrap(),
            Self::Ndjamena => Ndjamena.from_local_datetime(datetime).unwrap(),
            Self::Niamey => Niamey.from_local_datetime(datetime).unwrap(),
            Self::Nouakchott => Nouakchott.from_local_datetime(datetime).unwrap(),
            Self::Ouagadougou => Ouagadougou.from_local_datetime(datetime).unwrap(),
            Self::PortoNovo => PortoNovo.from_local_datetime(datetime).unwrap(),
            Self::SaoTome => Sao_Tome.from_local_datetime(datetime).unwrap(),
            Self::Timbuktu => Timbuktu.from_local_datetime(datetime).unwrap(),
            Self::Tripoli => Tripoli.from_local_datetime(datetime).unwrap(),
            Self::Tunis => Tunis.from_local_datetime(datetime).unwrap(),
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
