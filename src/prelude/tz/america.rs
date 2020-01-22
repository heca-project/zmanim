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
            other => Self::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string())),
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
            "New_Salem" => Ok(Self::NewSalem),
            other => {
                Self::from_str(other).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
            }
        }
    }
}
