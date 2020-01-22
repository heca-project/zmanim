use crate::prelude::tz::*;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum America {
    #[strum(disabled = "true")]
    Argentina(Argentina),
    #[strum(disabled = "true")]
    Indiana(Indiana),
    #[strum(disabled = "true")]
    Kentucky(Kentuky),
    #[strum(disabled = "true")]
    NorthDakota(NorthDakota),
    Constants,
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
            "Kentucky" => Ok(Self::Kentucky(Kentuky::try_from_path(&p[1..])?)),
            "North_Dakota" => Ok(Self::NorthDakota(NorthDakota::try_from_path(&p[1..])?)),
            "Bahia_Banderas" => Ok(Self::BahiaBanderas),
            "Blanc-Sablon" => Ok(America::BlancSablon),
            "Boa_Vista" => Ok(America::BoaVista),
            "Buenos_Aires" => Ok(America::BuenosAires),
            "Cambridge_Bay" => Ok(America::CambridgeBay),
            "Campo_Grande" => Ok(America::CampoGrande),
            "Coral_Harbour" => Ok(America::CoralHarbour),
            "Thunder_Bay" => Ok(America::ThunderBay),
            "Swift_Current" => Ok(America::SwiftCurrent),
            "St_Vincent" => Ok(America::StVincent),
            "St_Thomas" => Ok(America::StThomas),
            "St_Lucia" => Ok(America::StLucia),
            "St_Kitts" => Ok(America::StKitts),
            "St_Johns" => Ok(America::StJohns),
            "St_Barthelemy" => Ok(America::StBarthelemy),
            "Sao_Paulo" => Ok(America::SaoPaulo),
            "Santo_Domingo" => Ok(America::SantoDomingo),
            "Santa_Isabel" => Ok(America::SantaIsabel),
            "Rio_Branco" => Ok(America::RioBranco),
            "Rankin_Inlet" => Ok(America::RankinInlet),
            "Rainy_River" => Ok(America::RainyRiver),
            "Punta_Arenas" => Ok(America::PuntaArenas),
            "Costa_Rica" => Ok(America::CostaRica),
            "Dawson_Creek" => Ok(America::DawsonCreek),
            "El_Salvador" => Ok(America::ElSalvador),
            "Fort_Nelson" => Ok(America::FortNelson),
            "Fort_Wayne" => Ok(America::FortWayne),
            "Glace_Bay" => Ok(America::GlaceBay),
            "Goose_Bay" => Ok(America::GooseBay),
            "Grand_Turk" => Ok(America::GrandTurk),
            "Knox_IN" => Ok(America::KnoxIN),
            "La_Paz" => Ok(America::LaPaz),
            "Los_Angeles" => Ok(America::LosAngeles),
            "Lower_Princes" => Ok(America::LowerPrinces),
            "Mexico_City" => Ok(America::MexicoCity),
            "New_York" => Ok(America::NewYork),
            "Port_of_Spain" => Ok(America::PortOfSpain),
            "Port-au-Prince" => Ok(America::PortAuPrince),
            "Porto_Acre" => Ok(America::PortoAcre),
            "Porto_Velho" => Ok(America::PortoVelho),
            "Puerto_Rico" => Ok(America::PuertoRico),
            other => America::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
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
            "Buenos_Aires" => Ok(Argentina::BuenosAires),
            "La_Rioja" => Ok(Argentina::LaRioja),
            "Rio_Gallegos" => Ok(Argentina::RioGallegos),
            "San_Juan" => Ok(Argentina::SanJuan),
            "San_Luis" => Ok(Argentina::SanLuis),
            other => Argentina::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
        }
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
            "Tell_City" => Ok(Indiana::TellCity),
            other => Indiana::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
        }
    }
}

#[derive(Debug, EnumString)]
pub enum Kentuky {
    Louisville,
    Monticello,
}

impl Kentuky {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        match p[0] {
            other => Kentuky::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
        }
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
            "New_Salem" => Ok(NorthDakota::NewSalem),
            other => {
                NorthDakota::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
            }
        }
    }
}
