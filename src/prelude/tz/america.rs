use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate};
use chrono_tz::America::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString)]
pub enum America {
    #[strum(disabled = "true")]
    Argentina(Argentina),
    #[strum(disabled = "true")]
    Indiana(Indiana),
    #[strum(disabled = "true")]
    Kentucky(Kentucky),
    #[strum(disabled = "true")]
    NorthDakota(NorthDakota),
    Adak,
    Anchorage,
    Anguilla,
    Antigua,
    Araguaina,
    Aruba,
    Asuncion,
    Atikokan,
    Atka,
    Bahia,
    BahiaBanderas,
    Barbados,
    Belem,
    Belize,
    BlancSablon,
    BoaVista,
    Bogota,
    Boise,
    BuenosAires,
    CambridgeBay,
    CampoGrande,
    Cancun,
    Caracas,
    Catamarca,
    Cayenne,
    Cayman,
    Chicago,
    Chihuahua,
    CoralHarbour,
    Cordoba,
    CostaRica,
    Creston,
    Cuiaba,
    Curacao,
    Danmarkshavn,
    Dawson,
    DawsonCreek,
    Denver,
    Detroit,
    Dominica,
    Edmonton,
    Eirunepe,
    ElSalvador,
    Ensenada,
    FortNelson,
    FortWayne,
    Fortaleza,
    GlaceBay,
    Godthab,
    GooseBay,
    GrandTurk,
    Grenada,
    Guadeloupe,
    Guatemala,
    Guayaquil,
    Guyana,
    Halifax,
    Havana,
    Hermosillo,
    Indianapolis,
    Inuvik,
    Iqaluit,
    Jamaica,
    Jujuy,
    Juneau,
    KnoxIN,
    Kralendijk,
    LaPaz,
    Lima,
    LosAngeles,
    Louisville,
    LowerPrinces,
    Maceio,
    Managua,
    Manaus,
    Marigot,
    Martinique,
    Matamoros,
    Mazatlan,
    Mendoza,
    Menominee,
    Merida,
    Metlakatla,
    MexicoCity,
    Miquelon,
    Moncton,
    Monterrey,
    Montevideo,
    Montreal,
    Montserrat,
    Nassau,
    NewYork,
    Nipigon,
    Nome,
    Noronha,
    Ojinaga,
    Panama,
    Pangnirtung,
    Paramaribo,
    Phoenix,
    PortOfSpain,
    PortAuPrince,
    PortoAcre,
    PortoVelho,
    PuertoRico,
    PuntaArenas,
    RainyRiver,
    RankinInlet,
    Recife,
    Regina,
    Resolute,
    RioBranco,
    Rosario,
    SantaIsabel,
    Santarem,
    Santiago,
    SantoDomingo,
    SaoPaulo,
    Scoresbysund,
    Shiprock,
    Sitka,
    StBarthelemy,
    StJohns,
    StKitts,
    StLucia,
    StThomas,
    StVincent,
    SwiftCurrent,
    Tegucigalpa,
    Thule,
    ThunderBay,
    Tijuana,
    Toronto,
    Tortola,
    Vancouver,
    Virgin,
    Whitehorse,
    Winnipeg,
    Yakutat,
    Yellowknife,
}
impl America {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 && p.len() != 2 {
            return Err(Error::TooManyElements(p.len()));
        }
        match p[0] {
            "Argentina" => Ok(Self::Argentina(Argentina::try_from_path(&p[1..])?)),
            "Indiana" => Ok(Self::Indiana(Indiana::try_from_path(&p[1..])?)),
            "Kentucky" => Ok(Self::Kentucky(Kentucky::try_from_path(&p[1..])?)),
            "North_Dakota" => Ok(Self::NorthDakota(NorthDakota::try_from_path(&p[1..])?)),
            "Bahia_Banderas" => Ok(Self::BahiaBanderas),
            "Blanc-Sablon" => Ok(Self::BlancSablon),
            "Boa_Vista" => Ok(Self::BoaVista),
            "Buenos_Aires" => Ok(Self::BuenosAires),
            "Cambridge_Bay" => Ok(Self::CambridgeBay),
            "Campo_Grande" => Ok(Self::CampoGrande),
            "Coral_Harbour" => Ok(Self::CoralHarbour),
            "Thunder_Bay" => Ok(Self::ThunderBay),
            "Swift_Current" => Ok(Self::SwiftCurrent),
            "St_Vincent" => Ok(Self::StVincent),
            "St_Thomas" => Ok(Self::StThomas),
            "St_Lucia" => Ok(Self::StLucia),
            "St_Kitts" => Ok(Self::StKitts),
            "St_Johns" => Ok(Self::StJohns),
            "St_Barthelemy" => Ok(Self::StBarthelemy),
            "Sao_Paulo" => Ok(Self::SaoPaulo),
            "Santo_Domingo" => Ok(Self::SantoDomingo),
            "Santa_Isabel" => Ok(Self::SantaIsabel),
            "Rio_Branco" => Ok(Self::RioBranco),
            "Rankin_Inlet" => Ok(Self::RankinInlet),
            "Rainy_River" => Ok(Self::RainyRiver),
            "Punta_Arenas" => Ok(Self::PuntaArenas),
            "Costa_Rica" => Ok(Self::CostaRica),
            "Dawson_Creek" => Ok(Self::DawsonCreek),
            "El_Salvador" => Ok(Self::ElSalvador),
            "Fort_Nelson" => Ok(Self::FortNelson),
            "Fort_Wayne" => Ok(Self::FortWayne),
            "Glace_Bay" => Ok(Self::GlaceBay),
            "Goose_Bay" => Ok(Self::GooseBay),
            "Grand_Turk" => Ok(Self::GrandTurk),
            "Knox_IN" => Ok(Self::KnoxIN),
            "La_Paz" => Ok(Self::LaPaz),
            "Los_Angeles" => Ok(Self::LosAngeles),
            "Lower_Princes" => Ok(Self::LowerPrinces),
            "Mexico_City" => Ok(Self::MexicoCity),
            "New_York" => Ok(Self::NewYork),
            "Port_of_Spain" => Ok(Self::PortOfSpain),
            "Port-au-Prince" => Ok(Self::PortAuPrince),
            "Porto_Acre" => Ok(Self::PortoAcre),
            "Porto_Velho" => Ok(Self::PortoVelho),
            "Puerto_Rico" => Ok(Self::PuertoRico),
            other => Self::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
        }
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        match self {
            Self::Argentina(argentina) => argentina.get_tz(datetime),
            Self::Indiana(indiana) => indiana.get_tz(datetime),
            Self::Kentucky(kentucky) => kentucky.get_tz(datetime),
            Self::NorthDakota(north_dakota) => north_dakota.get_tz(datetime),
            left => {
                let p = match left {
                    Self::Adak => Adak.from_local_datetime(datetime).unwrap(),
                    Self::Anchorage => Anchorage.from_local_datetime(datetime).unwrap(),
                    Self::Anguilla => Anguilla.from_local_datetime(datetime).unwrap(),
                    Self::Antigua => Antigua.from_local_datetime(datetime).unwrap(),
                    Self::Araguaina => Araguaina.from_local_datetime(datetime).unwrap(),
                    Self::Aruba => Aruba.from_local_datetime(datetime).unwrap(),
                    Self::Asuncion => Asuncion.from_local_datetime(datetime).unwrap(),
                    Self::Atikokan => Atikokan.from_local_datetime(datetime).unwrap(),
                    Self::Atka => Atka.from_local_datetime(datetime).unwrap(),
                    Self::Bahia => Bahia.from_local_datetime(datetime).unwrap(),
                    Self::BahiaBanderas => Bahia_Banderas.from_local_datetime(datetime).unwrap(),
                    Self::Barbados => Barbados.from_local_datetime(datetime).unwrap(),
                    Self::Belem => Belem.from_local_datetime(datetime).unwrap(),
                    Self::Belize => Belize.from_local_datetime(datetime).unwrap(),
                    Self::BlancSablon => BlancSablon.from_local_datetime(datetime).unwrap(),
                    Self::BoaVista => Boa_Vista.from_local_datetime(datetime).unwrap(),
                    Self::Bogota => Bogota.from_local_datetime(datetime).unwrap(),
                    Self::Boise => Boise.from_local_datetime(datetime).unwrap(),
                    Self::BuenosAires => Buenos_Aires.from_local_datetime(datetime).unwrap(),
                    Self::CambridgeBay => Cambridge_Bay.from_local_datetime(datetime).unwrap(),
                    Self::CampoGrande => Campo_Grande.from_local_datetime(datetime).unwrap(),
                    Self::Cancun => Cancun.from_local_datetime(datetime).unwrap(),
                    Self::Caracas => Caracas.from_local_datetime(datetime).unwrap(),
                    Self::Catamarca => Catamarca.from_local_datetime(datetime).unwrap(),
                    Self::Cayenne => Cayenne.from_local_datetime(datetime).unwrap(),
                    Self::Cayman => Cayman.from_local_datetime(datetime).unwrap(),
                    Self::Chicago => Chicago.from_local_datetime(datetime).unwrap(),
                    Self::Chihuahua => Chihuahua.from_local_datetime(datetime).unwrap(),
                    Self::CoralHarbour => Coral_Harbour.from_local_datetime(datetime).unwrap(),
                    Self::Cordoba => Cordoba.from_local_datetime(datetime).unwrap(),
                    Self::CostaRica => Costa_Rica.from_local_datetime(datetime).unwrap(),
                    Self::Creston => Creston.from_local_datetime(datetime).unwrap(),
                    Self::Cuiaba => Cuiaba.from_local_datetime(datetime).unwrap(),
                    Self::Curacao => Curacao.from_local_datetime(datetime).unwrap(),
                    Self::Danmarkshavn => Danmarkshavn.from_local_datetime(datetime).unwrap(),
                    Self::Dawson => Dawson.from_local_datetime(datetime).unwrap(),
                    Self::DawsonCreek => Dawson_Creek.from_local_datetime(datetime).unwrap(),
                    Self::Denver => Denver.from_local_datetime(datetime).unwrap(),
                    Self::Detroit => Detroit.from_local_datetime(datetime).unwrap(),
                    Self::Dominica => Dominica.from_local_datetime(datetime).unwrap(),
                    Self::Edmonton => Edmonton.from_local_datetime(datetime).unwrap(),
                    Self::Eirunepe => Eirunepe.from_local_datetime(datetime).unwrap(),
                    Self::ElSalvador => El_Salvador.from_local_datetime(datetime).unwrap(),
                    Self::Ensenada => Ensenada.from_local_datetime(datetime).unwrap(),
                    Self::FortNelson => Fort_Nelson.from_local_datetime(datetime).unwrap(),
                    Self::FortWayne => Fort_Wayne.from_local_datetime(datetime).unwrap(),
                    Self::Fortaleza => Fortaleza.from_local_datetime(datetime).unwrap(),
                    Self::GlaceBay => Glace_Bay.from_local_datetime(datetime).unwrap(),
                    Self::Godthab => Godthab.from_local_datetime(datetime).unwrap(),
                    Self::GooseBay => Goose_Bay.from_local_datetime(datetime).unwrap(),
                    Self::GrandTurk => Grand_Turk.from_local_datetime(datetime).unwrap(),
                    Self::Grenada => Grenada.from_local_datetime(datetime).unwrap(),
                    Self::Guadeloupe => Guadeloupe.from_local_datetime(datetime).unwrap(),
                    Self::Guatemala => Guatemala.from_local_datetime(datetime).unwrap(),
                    Self::Guayaquil => Guayaquil.from_local_datetime(datetime).unwrap(),
                    Self::Guyana => Guyana.from_local_datetime(datetime).unwrap(),
                    Self::Halifax => Halifax.from_local_datetime(datetime).unwrap(),
                    Self::Havana => Havana.from_local_datetime(datetime).unwrap(),
                    Self::Hermosillo => Hermosillo.from_local_datetime(datetime).unwrap(),
                    Self::Indianapolis => Indianapolis.from_local_datetime(datetime).unwrap(),
                    Self::Inuvik => Inuvik.from_local_datetime(datetime).unwrap(),
                    Self::Iqaluit => Iqaluit.from_local_datetime(datetime).unwrap(),
                    Self::Jamaica => Jamaica.from_local_datetime(datetime).unwrap(),
                    Self::Jujuy => Jujuy.from_local_datetime(datetime).unwrap(),
                    Self::Juneau => Juneau.from_local_datetime(datetime).unwrap(),
                    Self::KnoxIN => Knox_IN.from_local_datetime(datetime).unwrap(),
                    Self::Kralendijk => Kralendijk.from_local_datetime(datetime).unwrap(),
                    Self::LaPaz => La_Paz.from_local_datetime(datetime).unwrap(),
                    Self::Lima => Lima.from_local_datetime(datetime).unwrap(),
                    Self::LosAngeles => Los_Angeles.from_local_datetime(datetime).unwrap(),
                    Self::Louisville => Louisville.from_local_datetime(datetime).unwrap(),
                    Self::LowerPrinces => Lower_Princes.from_local_datetime(datetime).unwrap(),
                    Self::Maceio => Maceio.from_local_datetime(datetime).unwrap(),
                    Self::Managua => Managua.from_local_datetime(datetime).unwrap(),
                    Self::Manaus => Manaus.from_local_datetime(datetime).unwrap(),
                    Self::Marigot => Marigot.from_local_datetime(datetime).unwrap(),
                    Self::Martinique => Martinique.from_local_datetime(datetime).unwrap(),
                    Self::Matamoros => Matamoros.from_local_datetime(datetime).unwrap(),
                    Self::Mazatlan => Mazatlan.from_local_datetime(datetime).unwrap(),
                    Self::Mendoza => Mendoza.from_local_datetime(datetime).unwrap(),
                    Self::Menominee => Menominee.from_local_datetime(datetime).unwrap(),
                    Self::Merida => Merida.from_local_datetime(datetime).unwrap(),
                    Self::Metlakatla => Metlakatla.from_local_datetime(datetime).unwrap(),
                    Self::MexicoCity => Mexico_City.from_local_datetime(datetime).unwrap(),
                    Self::Miquelon => Miquelon.from_local_datetime(datetime).unwrap(),
                    Self::Moncton => Moncton.from_local_datetime(datetime).unwrap(),
                    Self::Monterrey => Monterrey.from_local_datetime(datetime).unwrap(),
                    Self::Montevideo => Montevideo.from_local_datetime(datetime).unwrap(),
                    Self::Montreal => Montreal.from_local_datetime(datetime).unwrap(),
                    Self::Montserrat => Montserrat.from_local_datetime(datetime).unwrap(),
                    Self::Nassau => Nassau.from_local_datetime(datetime).unwrap(),
                    Self::NewYork => New_York.from_local_datetime(datetime).unwrap(),
                    Self::Nipigon => Nipigon.from_local_datetime(datetime).unwrap(),
                    Self::Nome => Nome.from_local_datetime(datetime).unwrap(),
                    Self::Noronha => Noronha.from_local_datetime(datetime).unwrap(),
                    Self::Ojinaga => Ojinaga.from_local_datetime(datetime).unwrap(),
                    Self::Panama => Panama.from_local_datetime(datetime).unwrap(),
                    Self::Pangnirtung => Pangnirtung.from_local_datetime(datetime).unwrap(),
                    Self::Paramaribo => Paramaribo.from_local_datetime(datetime).unwrap(),
                    Self::Phoenix => Phoenix.from_local_datetime(datetime).unwrap(),
                    Self::PortOfSpain => Port_of_Spain.from_local_datetime(datetime).unwrap(),
                    Self::PortAuPrince => PortauPrince.from_local_datetime(datetime).unwrap(),
                    Self::PortoAcre => Porto_Acre.from_local_datetime(datetime).unwrap(),
                    Self::PortoVelho => Porto_Velho.from_local_datetime(datetime).unwrap(),
                    Self::PuertoRico => Puerto_Rico.from_local_datetime(datetime).unwrap(),
                    Self::PuntaArenas => Punta_Arenas.from_local_datetime(datetime).unwrap(),
                    Self::RainyRiver => Rainy_River.from_local_datetime(datetime).unwrap(),
                    Self::RankinInlet => Rankin_Inlet.from_local_datetime(datetime).unwrap(),
                    Self::Recife => Recife.from_local_datetime(datetime).unwrap(),
                    Self::Regina => Regina.from_local_datetime(datetime).unwrap(),
                    Self::Resolute => Resolute.from_local_datetime(datetime).unwrap(),
                    Self::RioBranco => Rio_Branco.from_local_datetime(datetime).unwrap(),
                    Self::Rosario => Rosario.from_local_datetime(datetime).unwrap(),
                    Self::SantaIsabel => Santa_Isabel.from_local_datetime(datetime).unwrap(),
                    Self::Santarem => Santarem.from_local_datetime(datetime).unwrap(),
                    Self::Santiago => Santiago.from_local_datetime(datetime).unwrap(),
                    Self::SantoDomingo => Santo_Domingo.from_local_datetime(datetime).unwrap(),
                    Self::SaoPaulo => Sao_Paulo.from_local_datetime(datetime).unwrap(),
                    Self::Scoresbysund => Scoresbysund.from_local_datetime(datetime).unwrap(),
                    Self::Shiprock => Shiprock.from_local_datetime(datetime).unwrap(),
                    Self::Sitka => Sitka.from_local_datetime(datetime).unwrap(),
                    Self::StBarthelemy => St_Barthelemy.from_local_datetime(datetime).unwrap(),
                    Self::StJohns => St_Johns.from_local_datetime(datetime).unwrap(),
                    Self::StKitts => St_Kitts.from_local_datetime(datetime).unwrap(),
                    Self::StLucia => St_Lucia.from_local_datetime(datetime).unwrap(),
                    Self::StThomas => St_Thomas.from_local_datetime(datetime).unwrap(),
                    Self::StVincent => St_Vincent.from_local_datetime(datetime).unwrap(),
                    Self::SwiftCurrent => Swift_Current.from_local_datetime(datetime).unwrap(),
                    Self::Tegucigalpa => Tegucigalpa.from_local_datetime(datetime).unwrap(),
                    Self::Thule => Thule.from_local_datetime(datetime).unwrap(),
                    Self::ThunderBay => Thunder_Bay.from_local_datetime(datetime).unwrap(),
                    Self::Tijuana => Tijuana.from_local_datetime(datetime).unwrap(),
                    Self::Toronto => Toronto.from_local_datetime(datetime).unwrap(),
                    Self::Tortola => Tortola.from_local_datetime(datetime).unwrap(),
                    Self::Vancouver => Vancouver.from_local_datetime(datetime).unwrap(),
                    Self::Virgin => Virgin.from_local_datetime(datetime).unwrap(),
                    Self::Whitehorse => Whitehorse.from_local_datetime(datetime).unwrap(),
                    Self::Winnipeg => Winnipeg.from_local_datetime(datetime).unwrap(),
                    Self::Yakutat => Yakutat.from_local_datetime(datetime).unwrap(),
                    Self::Yellowknife => Yellowknife.from_local_datetime(datetime).unwrap(),
                    Self::Argentina(_)
                    | Self::Indiana(_)
                    | Self::Kentucky(_)
                    | Self::NorthDakota(_) => unreachable!(),
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
    }
}
#[derive(Debug, EnumString)]
pub enum Argentina {
    BuenosAires,
    Catamarca,
    ComodRivadavia,
    Cordoba,
    Jujuy,
    LaRioja,
    Mendoza,
    RioGallegos,
    Salta,
    SanJuan,
    SanLuis,
    Tucuman,
    Ushuaia,
}
impl Argentina {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        match p[0] {
            "Buenos_Aires" => Ok(Self::BuenosAires),
            "La_Rioja" => Ok(Self::LaRioja),
            "Rio_Gallegos" => Ok(Self::RioGallegos),
            "San_Juan" => Ok(Self::SanJuan),
            "San_Luis" => Ok(Self::SanLuis),
            other => Self::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
        }
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        use chrono_tz::America::Argentina::*;
        let p = match self {
            Self::BuenosAires => Buenos_Aires.from_local_datetime(datetime).unwrap(),
            Self::Catamarca => Catamarca.from_local_datetime(datetime).unwrap(),
            Self::ComodRivadavia => ComodRivadavia.from_local_datetime(datetime).unwrap(),
            Self::Cordoba => Cordoba.from_local_datetime(datetime).unwrap(),
            Self::Jujuy => Jujuy.from_local_datetime(datetime).unwrap(),
            Self::LaRioja => La_Rioja.from_local_datetime(datetime).unwrap(),
            Self::Mendoza => Mendoza.from_local_datetime(datetime).unwrap(),
            Self::RioGallegos => Rio_Gallegos.from_local_datetime(datetime).unwrap(),
            Self::Salta => Salta.from_local_datetime(datetime).unwrap(),
            Self::SanJuan => San_Juan.from_local_datetime(datetime).unwrap(),
            Self::SanLuis => San_Luis.from_local_datetime(datetime).unwrap(),
            Self::Tucuman => Tucuman.from_local_datetime(datetime).unwrap(),
            Self::Ushuaia => Ushuaia.from_local_datetime(datetime).unwrap(),
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
#[derive(Debug, EnumString)]
pub enum Indiana {
    Indianapolis,
    Knox,
    Marengo,
    Petersburg,
    TellCity,
    Vevay,
    Vincennes,
    Winamac,
}
impl Indiana {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        match p[0] {
            "Tell_City" => Ok(Self::TellCity),
            other => Self::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
        }
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        use chrono_tz::America::Indiana::*;
        let p = match self {
            Self::Indianapolis => Indianapolis.from_local_datetime(datetime).unwrap(),
            Self::Knox => Knox.from_local_datetime(datetime).unwrap(),
            Self::Marengo => Marengo.from_local_datetime(datetime).unwrap(),
            Self::Petersburg => Petersburg.from_local_datetime(datetime).unwrap(),
            Self::TellCity => Tell_City.from_local_datetime(datetime).unwrap(),
            Self::Vevay => Vevay.from_local_datetime(datetime).unwrap(),
            Self::Vincennes => Vincennes.from_local_datetime(datetime).unwrap(),
            Self::Winamac => Winamac.from_local_datetime(datetime).unwrap(),
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
#[derive(Debug, EnumString)]
pub enum Kentucky {
    Louisville,
    Monticello,
}
impl Kentucky {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        match p[0] {
            other => Self::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
        }
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        use chrono_tz::America::Kentucky::*;
        let p = match self {
            Self::Louisville => Louisville.from_local_datetime(datetime).unwrap(),
            Self::Monticello => Monticello.from_local_datetime(datetime).unwrap(),
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
#[derive(Debug, EnumString)]
pub enum NorthDakota {
    Beulah,
    Center,
    NewSalem,
}
impl NorthDakota {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        match p[0] {
            "New_Salem" => Ok(Self::NewSalem),
            other => Self::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
        }
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        use chrono_tz::America::North_Dakota::*;
        let p = match self {
            Self::Beulah => Beulah.from_local_datetime(datetime).unwrap(),
            Self::Center => Center.from_local_datetime(datetime).unwrap(),
            Self::NewSalem => New_Salem.from_local_datetime(datetime).unwrap(),
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
