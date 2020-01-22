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
            "Addis_Ababa" => Ok(Africa::AddisAbaba),
            "Dar_es_Salaam" => Ok(Africa::DarEsSalaam),
            "El_Aaiun" => Ok(Africa::ElAaiun),
            "Porto-Novo" => Ok(Africa::PortoNovo),
            "Sao_Tome" => Ok(Africa::SaoTome),
            other => Africa::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
        }
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Africa::Abidjan => Abidjan.from_local_datetime(datetime).unwrap(),
            Africa::Windhoek => Windhoek.from_local_datetime(datetime).unwrap(),
            Africa::Accra => Accra.from_local_datetime(datetime).unwrap(),
            Africa::AddisAbaba => Addis_Ababa.from_local_datetime(datetime).unwrap(),
            Africa::Algiers => Algiers.from_local_datetime(datetime).unwrap(),
            Africa::Asmara => Asmara.from_local_datetime(datetime).unwrap(),
            Africa::Asmera => Asmera.from_local_datetime(datetime).unwrap(),
            Africa::Bamako => Bamako.from_local_datetime(datetime).unwrap(),
            Africa::Bangui => Bangui.from_local_datetime(datetime).unwrap(),
            Africa::Banjul => Banjul.from_local_datetime(datetime).unwrap(),
            Africa::Bissau => Bissau.from_local_datetime(datetime).unwrap(),
            Africa::Blantyre => Blantyre.from_local_datetime(datetime).unwrap(),
            Africa::Brazzaville => Brazzaville.from_local_datetime(datetime).unwrap(),
            Africa::Bujumbura => Bujumbura.from_local_datetime(datetime).unwrap(),
            Africa::Cairo => Cairo.from_local_datetime(datetime).unwrap(),
            Africa::Casablanca => Casablanca.from_local_datetime(datetime).unwrap(),
            Africa::Ceuta => Ceuta.from_local_datetime(datetime).unwrap(),
            Africa::Conakry => Conakry.from_local_datetime(datetime).unwrap(),
            Africa::Dakar => Dakar.from_local_datetime(datetime).unwrap(),
            Africa::DarEsSalaam => Dar_es_Salaam.from_local_datetime(datetime).unwrap(),
            Africa::Djibouti => Djibouti.from_local_datetime(datetime).unwrap(),
            Africa::Douala => Douala.from_local_datetime(datetime).unwrap(),
            Africa::ElAaiun => El_Aaiun.from_local_datetime(datetime).unwrap(),
            Africa::Freetown => Freetown.from_local_datetime(datetime).unwrap(),
            Africa::Gaborone => Gaborone.from_local_datetime(datetime).unwrap(),
            Africa::Harare => Harare.from_local_datetime(datetime).unwrap(),
            Africa::Johannesburg => Johannesburg.from_local_datetime(datetime).unwrap(),
            Africa::Juba => Juba.from_local_datetime(datetime).unwrap(),
            Africa::Kampala => Kampala.from_local_datetime(datetime).unwrap(),
            Africa::Khartoum => Khartoum.from_local_datetime(datetime).unwrap(),
            Africa::Kigali => Kigali.from_local_datetime(datetime).unwrap(),
            Africa::Kinshasa => Kinshasa.from_local_datetime(datetime).unwrap(),
            Africa::Lagos => Lagos.from_local_datetime(datetime).unwrap(),
            Africa::Libreville => Libreville.from_local_datetime(datetime).unwrap(),
            Africa::Lome => Lome.from_local_datetime(datetime).unwrap(),
            Africa::Luanda => Luanda.from_local_datetime(datetime).unwrap(),
            Africa::Lubumbashi => Lubumbashi.from_local_datetime(datetime).unwrap(),
            Africa::Lusaka => Lusaka.from_local_datetime(datetime).unwrap(),
            Africa::Malabo => Malabo.from_local_datetime(datetime).unwrap(),
            Africa::Maputo => Maputo.from_local_datetime(datetime).unwrap(),
            Africa::Maseru => Maseru.from_local_datetime(datetime).unwrap(),
            Africa::Mbabane => Mbabane.from_local_datetime(datetime).unwrap(),
            Africa::Mogadishu => Mogadishu.from_local_datetime(datetime).unwrap(),
            Africa::Monrovia => Monrovia.from_local_datetime(datetime).unwrap(),
            Africa::Nairobi => Nairobi.from_local_datetime(datetime).unwrap(),
            Africa::Ndjamena => Ndjamena.from_local_datetime(datetime).unwrap(),
            Africa::Niamey => Niamey.from_local_datetime(datetime).unwrap(),
            Africa::Nouakchott => Nouakchott.from_local_datetime(datetime).unwrap(),
            Africa::Ouagadougou => Ouagadougou.from_local_datetime(datetime).unwrap(),
            Africa::PortoNovo => PortoNovo.from_local_datetime(datetime).unwrap(),
            Africa::SaoTome => Sao_Tome.from_local_datetime(datetime).unwrap(),
            Africa::Timbuktu => Timbuktu.from_local_datetime(datetime).unwrap(),
            Africa::Tripoli => Tripoli.from_local_datetime(datetime).unwrap(),
            Africa::Tunis => Tunis.from_local_datetime(datetime).unwrap(),
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
