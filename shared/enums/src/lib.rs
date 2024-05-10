use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString, Display };

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq)]
pub enum LiturgicalColor {
    Green,
    Violet,
    Blue,
    WhiteOrGold,
    Red,
    Black,
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq, EnumString, EnumIter, Display)]
pub enum Timezone {
    #[strum(serialize = "GMT-12")]
    UTCm12,
    #[strum(serialize = "GMT-11")]
    UTCm11,
    #[strum(serialize = "GMT-10")]
    UTCm10,
    #[strum(serialize = "GMT-9")]
    UTCm9,
    #[strum(serialize = "GMT-8")]
    UTCm8,
    #[strum(serialize = "GMT-7")]
    UTCm7,
    #[strum(serialize = "GMT-6")]
    UTCm6,
    #[strum(serialize = "GMT-5")]
    UTCm5,
    #[strum(serialize = "GMT-4")]
    UTCm4,
    #[strum(serialize = "GMT-3")]
    UTCm3,
    #[strum(serialize = "GMT-2")]
    UTCm2,
    #[strum(serialize = "GMT-1")]
    UTCm1,
    #[strum(serialize = "UTC")]
    UTC,
    #[strum(serialize = "GMT+1")]
    UTCp1,
    #[strum(serialize = "GMT+2")]
    UTCp2,
    #[strum(serialize = "GMT+3")]
    UTCp3,
    #[strum(serialize = "GMT+4")]
    UTCp4,
    #[strum(serialize = "GMT+5")]
    UTCp5,
    #[strum(serialize = "GMT+6")]
    UTCp6,
    #[strum(serialize = "GMT+7")]
    UTCp7,
    #[strum(serialize = "GMT+8")]
    UTCp8,
    #[strum(serialize = "GMT+9")]
    UTCp9,
    #[strum(serialize = "GMT+10")]
    UTCp10,
    #[strum(serialize = "GMT+11")]
    UTCp11,
    #[strum(serialize = "GMT+12")]
    UTCp12,
}

// impl Display for Timezone {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         match self {
//             Timezone::UTCm12 => write!(f, "GMT-12"),
//             Timezone::UTCm11 => write!(f, "GMT-11"),
//             Timezone::UTCm10 => write!(f, "GMT-10"),
//             Timezone::UTCm9 => write!(f, "GMT-9"),
//             Timezone::UTCm8 => write!(f, "GMT-8"),
//             Timezone::UTCm7 => write!(f, "GMT-7"),
//             Timezone::UTCm6 => write!(f, "GMT-6"),
//             Timezone::UTCm5 => write!(f, "GMT-5"),
//             Timezone::UTCm4 => write!(f, "GMT-4"),
//             Timezone::UTCm3 => write!(f, "GMT-3"),
//             Timezone::UTCm2 => write!(f, "GMT-2"),
//             Timezone::UTCm1 => write!(f, "GMT-1"),
//             Timezone::UTC => write!(f, "UTC"),
//             Timezone::UTCp1 => write!(f, "GMT+1"),
//             Timezone::UTCp2 => write!(f, "GMT+2"),
//             Timezone::UTCp3 => write!(f, "GMT+3"),
//             Timezone::UTCp4 => write!(f, "GMT+4"),
//             Timezone::UTCp5 => write!(f, "GMT+5"),
//             Timezone::UTCp6 => write!(f, "GMT+6"),
//             Timezone::UTCp7 => write!(f, "GMT+7"),
//             Timezone::UTCp8 => write!(f, "GMT+8"),
//             Timezone::UTCp9 => write!(f, "GMT+9"),
//             Timezone::UTCp10 => write!(f, "GMT+10"),
//             Timezone::UTCp11 => write!(f, "GMT+11"),
//             Timezone::UTCp12 => write!(f, "GMT+12"),
//         }
//     }
// }

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Into<u32> for Month {
    fn into(self) -> u32 {
        match self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
}

impl TryFrom<u32> for Month {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(()), // Return an error for values outside the range
        }
    }
}

pub enum IconInput {
    Bible,
    Church,
    Gear,
    Bell,
    Shop,
}

pub enum FontSize {
    Small,
    Base,
    Large,
    ExtraLarge,
    ExtraExtraLarge,
}

impl Into<&'static str> for FontSize {
    fn into(self) -> &'static str {
        match self {
            FontSize::Small => "sm",
            FontSize::Base => "base",
            FontSize::Large => "lg",
            FontSize::ExtraLarge => "xl",
            FontSize::ExtraExtraLarge => "2xl",
        }
    }
}
