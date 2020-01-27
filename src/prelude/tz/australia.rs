use crate::prelude::tz::*;
use chrono::offset::{Offset, TimeZone};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use chrono_tz::Australia::*;
use std::str::FromStr;
use strum_macros::EnumString;
#[derive(Debug, EnumString, Clone)]
#[non_exhaustive]
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
            Self::ACT => ACT.from_utc_datetime(datetime),
            Self::Adelaide => Adelaide.from_utc_datetime(datetime),
            Self::Brisbane => Brisbane.from_utc_datetime(datetime),
            Self::BrokenHill => Broken_Hill.from_utc_datetime(datetime),
            Self::Canberra => Canberra.from_utc_datetime(datetime),
            Self::Currie => Currie.from_utc_datetime(datetime),
            Self::Darwin => Darwin.from_utc_datetime(datetime),
            Self::Eucla => Eucla.from_utc_datetime(datetime),
            Self::Hobart => Hobart.from_utc_datetime(datetime),
            Self::LHI => LHI.from_utc_datetime(datetime),
            Self::Lindeman => Lindeman.from_utc_datetime(datetime),
            Self::LordHowe => Lord_Howe.from_utc_datetime(datetime),
            Self::Melbourne => Melbourne.from_utc_datetime(datetime),
            Self::NSW => NSW.from_utc_datetime(datetime),
            Self::North => North.from_utc_datetime(datetime),
            Self::Perth => Perth.from_utc_datetime(datetime),
            Self::Queensland => Queensland.from_utc_datetime(datetime),
            Self::South => South.from_utc_datetime(datetime),
            Self::Sydney => Sydney.from_utc_datetime(datetime),
            Self::Tasmania => Tasmania.from_utc_datetime(datetime),
            Self::Victoria => Victoria.from_utc_datetime(datetime),
            Self::West => West.from_utc_datetime(datetime),
            Self::Yancowinna => Yancowinna.from_utc_datetime(datetime),
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
