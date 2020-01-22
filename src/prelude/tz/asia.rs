use crate::prelude::tz::*;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Asia {
    Aden,
    Almaty,
    Amman,
    Anadyr,
    Aqtau,
    Aqtobe,
    Ashgabat,
    Ashkhabad,
    Atyrau,
    Baghdad,
    Bahrain,
    Baku,
    Bangkok,
    Barnaul,
    Beirut,
    Bishkek,
    Brunei,
    Calcutta,
    Chita,
    Choibalsan,
    Chongqing,
    Chungking,
    Colombo,
    Dacca,
    Damascus,
    Dhaka,
    Dili,
    Dubai,
    Dushanbe,
    Famagusta,
    Gaza,
    Harbin,
    Hebron,
    #[strum(serialize = "Ho_Chi_Minh")]
    HoChiMinh,
    #[strum(serialize = "Hong_Kong")]
    HongKong,
    Hovd,
    Irkutsk,
    Istanbul,
    Jakarta,
    Jayapura,
    Jerusalem,
    Kabul,
    Kamchatka,
    Karachi,
    Kashgar,
    Kathmandu,
    Katmandu,
    Khandyga,
    Kolkata,
    Krasnoyarsk,
    #[strum(serialize = "Kuala_Lumpur")]
    KualaLumpur,
    Kuching,
    Kuwait,
    Macao,
    Macau,
    Magadan,
    Makassar,
    Manila,
    Muscat,
    Nicosia,
    Novokuznetsk,
    Novosibirsk,
    Omsk,
    Oral,
    #[strum(serialize = "Phnom_Penh")]
    PhnomPenh,
    Pontianak,
    Pyongyang,
    Qatar,
    Qostanay,
    Qyzylorda,
    Rangoon,
    Riyadh,
    Saigon,
    Sakhalin,
    Samarkand,
    Seoul,
    Shanghai,
    Singapore,
    Srednekolymsk,
    Taipei,
    Tashkent,
    Tbilisi,
    Tehran,
    #[strum(serialize = "Tel_Aviv")]
    TelAviv,
    Thimbu,
    Thimphu,
    Tokyo,
    Tomsk,
    #[strum(serialize = "Ujung_Pandang")]
    UjungPandang,
    Ulaanbaatar,
    #[strum(serialize = "Ulan_Bator")]
    UlanBator,
    Urumqi,
    #[strum(serialize = "Ust-Nera")]
    UstNera,
    Vientiane,
    Vladivostok,
    Yakutsk,
    Yangon,
    Yekaterinburg,
    Yerevan,
}

impl Asia {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        Asia::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
}
