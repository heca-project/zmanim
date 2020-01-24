use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Australia::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString)]
pub enum Australia {
    ACT,
    Adelaide,
    Brisbane,
    #[strum(serialize = "Broken_Hill")]
    BrokenHill,
    Canberra,
    Currie,
    Darwin,
    Eucla,
    Hobart,
    LHI,
    Lindeman,
    #[strum(serialize = "Lord_Howe")]
    LordHowe,
    Melbourne,
    NSW,
    North,
    Perth,
    Queensland,
    South,
    Sydney,
    Tasmania,
    Victoria,
    West,
    Yancowinna,
}
impl Australia {
    pub(crate) fn try_from_path(p: &[&str]) -> Result<Self, Error> {
        if p.len() != 1 {
            return Err(Error::TooManyElements(p.len()));
        }
        Self::from_str(p[0]).map_err(|_| Error::WrongTimeZone(p[0].to_string()))
    }
    pub(crate) fn get_tz(&self, datetime: &NaiveDateTime) -> FixedOffset {
        let p = match self {
            Self::ACT => ACT.from_local_datetime(datetime).unwrap(),
            Self::Adelaide => Adelaide.from_local_datetime(datetime).unwrap(),
            Self::Brisbane => Brisbane.from_local_datetime(datetime).unwrap(),
            Self::BrokenHill => Broken_Hill.from_local_datetime(datetime).unwrap(),
            Self::Canberra => Canberra.from_local_datetime(datetime).unwrap(),
            Self::Currie => Currie.from_local_datetime(datetime).unwrap(),
            Self::Darwin => Darwin.from_local_datetime(datetime).unwrap(),
            Self::Eucla => Eucla.from_local_datetime(datetime).unwrap(),
            Self::Hobart => Hobart.from_local_datetime(datetime).unwrap(),
            Self::LHI => LHI.from_local_datetime(datetime).unwrap(),
            Self::Lindeman => Lindeman.from_local_datetime(datetime).unwrap(),
            Self::LordHowe => Lord_Howe.from_local_datetime(datetime).unwrap(),
            Self::Melbourne => Melbourne.from_local_datetime(datetime).unwrap(),
            Self::NSW => NSW.from_local_datetime(datetime).unwrap(),
            Self::North => North.from_local_datetime(datetime).unwrap(),
            Self::Perth => Perth.from_local_datetime(datetime).unwrap(),
            Self::Queensland => Queensland.from_local_datetime(datetime).unwrap(),
            Self::South => South.from_local_datetime(datetime).unwrap(),
            Self::Sydney => Sydney.from_local_datetime(datetime).unwrap(),
            Self::Tasmania => Tasmania.from_local_datetime(datetime).unwrap(),
            Self::Victoria => Victoria.from_local_datetime(datetime).unwrap(),
            Self::West => West.from_local_datetime(datetime).unwrap(),
            Self::Yancowinna => Yancowinna.from_local_datetime(datetime).unwrap(),
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
