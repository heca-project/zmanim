use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate};
use chrono_tz::America::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
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
                    Self::Adak => Adak.from_utc_datetime(datetime),
                    Self::Anchorage => Anchorage.from_utc_datetime(datetime),
                    Self::Anguilla => Anguilla.from_utc_datetime(datetime),
                    Self::Antigua => Antigua.from_utc_datetime(datetime),
                    Self::Araguaina => Araguaina.from_utc_datetime(datetime),
                    Self::Aruba => Aruba.from_utc_datetime(datetime),
                    Self::Asuncion => Asuncion.from_utc_datetime(datetime),
                    Self::Atikokan => Atikokan.from_utc_datetime(datetime),
                    Self::Atka => Atka.from_utc_datetime(datetime),
                    Self::Bahia => Bahia.from_utc_datetime(datetime),
                    Self::BahiaBanderas => Bahia_Banderas.from_utc_datetime(datetime),
                    Self::Barbados => Barbados.from_utc_datetime(datetime),
                    Self::Belem => Belem.from_utc_datetime(datetime),
                    Self::Belize => Belize.from_utc_datetime(datetime),
                    Self::BlancSablon => BlancSablon.from_utc_datetime(datetime),
                    Self::BoaVista => Boa_Vista.from_utc_datetime(datetime),
                    Self::Bogota => Bogota.from_utc_datetime(datetime),
                    Self::Boise => Boise.from_utc_datetime(datetime),
                    Self::BuenosAires => Buenos_Aires.from_utc_datetime(datetime),
                    Self::CambridgeBay => Cambridge_Bay.from_utc_datetime(datetime),
                    Self::CampoGrande => Campo_Grande.from_utc_datetime(datetime),
                    Self::Cancun => Cancun.from_utc_datetime(datetime),
                    Self::Caracas => Caracas.from_utc_datetime(datetime),
                    Self::Catamarca => Catamarca.from_utc_datetime(datetime),
                    Self::Cayenne => Cayenne.from_utc_datetime(datetime),
                    Self::Cayman => Cayman.from_utc_datetime(datetime),
                    Self::Chicago => Chicago.from_utc_datetime(datetime),
                    Self::Chihuahua => Chihuahua.from_utc_datetime(datetime),
                    Self::CoralHarbour => Coral_Harbour.from_utc_datetime(datetime),
                    Self::Cordoba => Cordoba.from_utc_datetime(datetime),
                    Self::CostaRica => Costa_Rica.from_utc_datetime(datetime),
                    Self::Creston => Creston.from_utc_datetime(datetime),
                    Self::Cuiaba => Cuiaba.from_utc_datetime(datetime),
                    Self::Curacao => Curacao.from_utc_datetime(datetime),
                    Self::Danmarkshavn => Danmarkshavn.from_utc_datetime(datetime),
                    Self::Dawson => Dawson.from_utc_datetime(datetime),
                    Self::DawsonCreek => Dawson_Creek.from_utc_datetime(datetime),
                    Self::Denver => Denver.from_utc_datetime(datetime),
                    Self::Detroit => Detroit.from_utc_datetime(datetime),
                    Self::Dominica => Dominica.from_utc_datetime(datetime),
                    Self::Edmonton => Edmonton.from_utc_datetime(datetime),
                    Self::Eirunepe => Eirunepe.from_utc_datetime(datetime),
                    Self::ElSalvador => El_Salvador.from_utc_datetime(datetime),
                    Self::Ensenada => Ensenada.from_utc_datetime(datetime),
                    Self::FortNelson => Fort_Nelson.from_utc_datetime(datetime),
                    Self::FortWayne => Fort_Wayne.from_utc_datetime(datetime),
                    Self::Fortaleza => Fortaleza.from_utc_datetime(datetime),
                    Self::GlaceBay => Glace_Bay.from_utc_datetime(datetime),
                    Self::Godthab => Godthab.from_utc_datetime(datetime),
                    Self::GooseBay => Goose_Bay.from_utc_datetime(datetime),
                    Self::GrandTurk => Grand_Turk.from_utc_datetime(datetime),
                    Self::Grenada => Grenada.from_utc_datetime(datetime),
                    Self::Guadeloupe => Guadeloupe.from_utc_datetime(datetime),
                    Self::Guatemala => Guatemala.from_utc_datetime(datetime),
                    Self::Guayaquil => Guayaquil.from_utc_datetime(datetime),
                    Self::Guyana => Guyana.from_utc_datetime(datetime),
                    Self::Halifax => Halifax.from_utc_datetime(datetime),
                    Self::Havana => Havana.from_utc_datetime(datetime),
                    Self::Hermosillo => Hermosillo.from_utc_datetime(datetime),
                    Self::Indianapolis => Indianapolis.from_utc_datetime(datetime),
                    Self::Inuvik => Inuvik.from_utc_datetime(datetime),
                    Self::Iqaluit => Iqaluit.from_utc_datetime(datetime),
                    Self::Jamaica => Jamaica.from_utc_datetime(datetime),
                    Self::Jujuy => Jujuy.from_utc_datetime(datetime),
                    Self::Juneau => Juneau.from_utc_datetime(datetime),
                    Self::KnoxIN => Knox_IN.from_utc_datetime(datetime),
                    Self::Kralendijk => Kralendijk.from_utc_datetime(datetime),
                    Self::LaPaz => La_Paz.from_utc_datetime(datetime),
                    Self::Lima => Lima.from_utc_datetime(datetime),
                    Self::LosAngeles => Los_Angeles.from_utc_datetime(datetime),
                    Self::Louisville => Louisville.from_utc_datetime(datetime),
                    Self::LowerPrinces => Lower_Princes.from_utc_datetime(datetime),
                    Self::Maceio => Maceio.from_utc_datetime(datetime),
                    Self::Managua => Managua.from_utc_datetime(datetime),
                    Self::Manaus => Manaus.from_utc_datetime(datetime),
                    Self::Marigot => Marigot.from_utc_datetime(datetime),
                    Self::Martinique => Martinique.from_utc_datetime(datetime),
                    Self::Matamoros => Matamoros.from_utc_datetime(datetime),
                    Self::Mazatlan => Mazatlan.from_utc_datetime(datetime),
                    Self::Mendoza => Mendoza.from_utc_datetime(datetime),
                    Self::Menominee => Menominee.from_utc_datetime(datetime),
                    Self::Merida => Merida.from_utc_datetime(datetime),
                    Self::Metlakatla => Metlakatla.from_utc_datetime(datetime),
                    Self::MexicoCity => Mexico_City.from_utc_datetime(datetime),
                    Self::Miquelon => Miquelon.from_utc_datetime(datetime),
                    Self::Moncton => Moncton.from_utc_datetime(datetime),
                    Self::Monterrey => Monterrey.from_utc_datetime(datetime),
                    Self::Montevideo => Montevideo.from_utc_datetime(datetime),
                    Self::Montreal => Montreal.from_utc_datetime(datetime),
                    Self::Montserrat => Montserrat.from_utc_datetime(datetime),
                    Self::Nassau => Nassau.from_utc_datetime(datetime),
                    Self::NewYork => New_York.from_utc_datetime(datetime),
                    Self::Nipigon => Nipigon.from_utc_datetime(datetime),
                    Self::Nome => Nome.from_utc_datetime(datetime),
                    Self::Noronha => Noronha.from_utc_datetime(datetime),
                    Self::Ojinaga => Ojinaga.from_utc_datetime(datetime),
                    Self::Panama => Panama.from_utc_datetime(datetime),
                    Self::Pangnirtung => Pangnirtung.from_utc_datetime(datetime),
                    Self::Paramaribo => Paramaribo.from_utc_datetime(datetime),
                    Self::Phoenix => Phoenix.from_utc_datetime(datetime),
                    Self::PortOfSpain => Port_of_Spain.from_utc_datetime(datetime),
                    Self::PortAuPrince => PortauPrince.from_utc_datetime(datetime),
                    Self::PortoAcre => Porto_Acre.from_utc_datetime(datetime),
                    Self::PortoVelho => Porto_Velho.from_utc_datetime(datetime),
                    Self::PuertoRico => Puerto_Rico.from_utc_datetime(datetime),
                    Self::PuntaArenas => Punta_Arenas.from_utc_datetime(datetime),
                    Self::RainyRiver => Rainy_River.from_utc_datetime(datetime),
                    Self::RankinInlet => Rankin_Inlet.from_utc_datetime(datetime),
                    Self::Recife => Recife.from_utc_datetime(datetime),
                    Self::Regina => Regina.from_utc_datetime(datetime),
                    Self::Resolute => Resolute.from_utc_datetime(datetime),
                    Self::RioBranco => Rio_Branco.from_utc_datetime(datetime),
                    Self::Rosario => Rosario.from_utc_datetime(datetime),
                    Self::SantaIsabel => Santa_Isabel.from_utc_datetime(datetime),
                    Self::Santarem => Santarem.from_utc_datetime(datetime),
                    Self::Santiago => Santiago.from_utc_datetime(datetime),
                    Self::SantoDomingo => Santo_Domingo.from_utc_datetime(datetime),
                    Self::SaoPaulo => Sao_Paulo.from_utc_datetime(datetime),
                    Self::Scoresbysund => Scoresbysund.from_utc_datetime(datetime),
                    Self::Shiprock => Shiprock.from_utc_datetime(datetime),
                    Self::Sitka => Sitka.from_utc_datetime(datetime),
                    Self::StBarthelemy => St_Barthelemy.from_utc_datetime(datetime),
                    Self::StJohns => St_Johns.from_utc_datetime(datetime),
                    Self::StKitts => St_Kitts.from_utc_datetime(datetime),
                    Self::StLucia => St_Lucia.from_utc_datetime(datetime),
                    Self::StThomas => St_Thomas.from_utc_datetime(datetime),
                    Self::StVincent => St_Vincent.from_utc_datetime(datetime),
                    Self::SwiftCurrent => Swift_Current.from_utc_datetime(datetime),
                    Self::Tegucigalpa => Tegucigalpa.from_utc_datetime(datetime),
                    Self::Thule => Thule.from_utc_datetime(datetime),
                    Self::ThunderBay => Thunder_Bay.from_utc_datetime(datetime),
                    Self::Tijuana => Tijuana.from_utc_datetime(datetime),
                    Self::Toronto => Toronto.from_utc_datetime(datetime),
                    Self::Tortola => Tortola.from_utc_datetime(datetime),
                    Self::Vancouver => Vancouver.from_utc_datetime(datetime),
                    Self::Virgin => Virgin.from_utc_datetime(datetime),
                    Self::Whitehorse => Whitehorse.from_utc_datetime(datetime),
                    Self::Winnipeg => Winnipeg.from_utc_datetime(datetime),
                    Self::Yakutat => Yakutat.from_utc_datetime(datetime),
                    Self::Yellowknife => Yellowknife.from_utc_datetime(datetime),
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
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
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
            Self::BuenosAires => Buenos_Aires.from_utc_datetime(datetime),
            Self::Catamarca => Catamarca.from_utc_datetime(datetime),
            Self::ComodRivadavia => ComodRivadavia.from_utc_datetime(datetime),
            Self::Cordoba => Cordoba.from_utc_datetime(datetime),
            Self::Jujuy => Jujuy.from_utc_datetime(datetime),
            Self::LaRioja => La_Rioja.from_utc_datetime(datetime),
            Self::Mendoza => Mendoza.from_utc_datetime(datetime),
            Self::RioGallegos => Rio_Gallegos.from_utc_datetime(datetime),
            Self::Salta => Salta.from_utc_datetime(datetime),
            Self::SanJuan => San_Juan.from_utc_datetime(datetime),
            Self::SanLuis => San_Luis.from_utc_datetime(datetime),
            Self::Tucuman => Tucuman.from_utc_datetime(datetime),
            Self::Ushuaia => Ushuaia.from_utc_datetime(datetime),
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
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
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
            Self::Indianapolis => Indianapolis.from_utc_datetime(datetime),
            Self::Knox => Knox.from_utc_datetime(datetime),
            Self::Marengo => Marengo.from_utc_datetime(datetime),
            Self::Petersburg => Petersburg.from_utc_datetime(datetime),
            Self::TellCity => Tell_City.from_utc_datetime(datetime),
            Self::Vevay => Vevay.from_utc_datetime(datetime),
            Self::Vincennes => Vincennes.from_utc_datetime(datetime),
            Self::Winamac => Winamac.from_utc_datetime(datetime),
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
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
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
            Self::Louisville => Louisville.from_utc_datetime(datetime),
            Self::Monticello => Monticello.from_utc_datetime(datetime),
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
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
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
            Self::Beulah => Beulah.from_utc_datetime(datetime),
            Self::Center => Center.from_utc_datetime(datetime),
            Self::NewSalem => New_Salem.from_utc_datetime(datetime),
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
