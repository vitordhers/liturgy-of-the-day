use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
pub mod liturgy;
pub mod theme;
use chrono::FixedOffset;

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq, EnumString, EnumIter, Display)]
pub enum Timezone {
    #[serde(rename = "GMT-12")]
    #[strum(serialize = "GMT-12")]
    UTCm12,
    #[serde(rename = "GMT-11")]
    #[strum(serialize = "GMT-11")]
    UTCm11,
    #[serde(rename = "GMT-10")]
    #[strum(serialize = "GMT-10")]
    UTCm10,
    #[serde(rename = "GMT-9")]
    #[strum(serialize = "GMT-9")]
    UTCm9,
    #[serde(rename = "GMT-8")]
    #[strum(serialize = "GMT-8")]
    UTCm8,
    #[serde(rename = "GMT-7")]
    #[strum(serialize = "GMT-7")]
    UTCm7,
    #[serde(rename = "GMT-6")]
    #[strum(serialize = "GMT-6")]
    UTCm6,
    #[serde(rename = "GMT-5")]
    #[strum(serialize = "GMT-5")]
    UTCm5,
    #[serde(rename = "GMT-4")]
    #[strum(serialize = "GMT-4")]
    UTCm4,
    #[serde(rename = "GMT-3")]
    #[strum(serialize = "GMT-3")]
    UTCm3,
    #[serde(rename = "GMT-2")]
    #[strum(serialize = "GMT-2")]
    UTCm2,
    #[serde(rename = "GMT-1")]
    #[strum(serialize = "GMT-1")]
    UTCm1,
    #[serde(rename = "UTC")]
    #[strum(serialize = "UTC")]
    UTC,
    #[serde(rename = "GMT+1")]
    #[strum(serialize = "GMT+1")]
    UTCp1,
    #[serde(rename = "GMT+2")]
    #[strum(serialize = "GMT+2")]
    UTCp2,
    #[serde(rename = "GMT+3")]
    #[strum(serialize = "GMT+3")]
    UTCp3,
    #[serde(rename = "GMT+4")]
    #[strum(serialize = "GMT+4")]
    UTCp4,
    #[serde(rename = "GMT+5")]
    #[strum(serialize = "GMT+5")]
    UTCp5,
    #[serde(rename = "GMT+6")]
    #[strum(serialize = "GMT+6")]
    UTCp6,
    #[serde(rename = "GMT+7")]
    #[strum(serialize = "GMT+7")]
    UTCp7,
    #[serde(rename = "GMT+8")]
    #[strum(serialize = "GMT+8")]
    UTCp8,
    #[serde(rename = "GMT+9")]
    #[strum(serialize = "GMT+9")]
    UTCp9,
    #[serde(rename = "GMT+10")]
    #[strum(serialize = "GMT+10")]
    UTCp10,
    #[serde(rename = "GMT+11")]
    #[strum(serialize = "GMT+11")]
    UTCp11,
    #[serde(rename = "GMT+12")]
    #[strum(serialize = "GMT+12")]
    UTCp12,
}

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

impl Into<FixedOffset> for Timezone {
    fn into(self) -> FixedOffset {
        match self {
            Self::UTC => FixedOffset::west_opt(0).expect("0 seconds to be within bounds"),
            Self::UTCm1 => FixedOffset::west_opt(1 * 3600).expect("3600 seconds to be within bounds"),
            Self::UTCm2 => FixedOffset::west_opt(2 * 3600).expect("2 * 3600 seconds to be within bounds"),
            Self::UTCm3 => FixedOffset::west_opt(3 * 3600).expect("3 * 3600 seconds to be within bounds"),
            Self::UTCm4 => FixedOffset::west_opt(4 * 3600).expect("4 * 3600 seconds to be within bounds"),
            Self::UTCm5 => FixedOffset::west_opt(5 * 3600).expect("5 * 3600 seconds to be within bounds"),
            Self::UTCm6 => FixedOffset::west_opt(6 * 3600).expect("6 * 3600 seconds to be within bounds"),
            Self::UTCm7 => FixedOffset::west_opt(7 * 3600).expect("7 * 3600 seconds to be within bounds"),
            Self::UTCm8 => FixedOffset::west_opt(8 * 3600).expect("8 * 3600 seconds to be within bounds"),
            Self::UTCm9 => FixedOffset::west_opt(9 * 3600).expect("9 * 3600 seconds to be within bounds"),
            Self::UTCm10 => FixedOffset::west_opt(10 * 3600).expect("10 * 3600 seconds to be within bounds"),
            Self::UTCm11 => FixedOffset::west_opt(11 * 3600).expect("11 * 3600 seconds to be within bounds"),
            Self::UTCm12 => FixedOffset::west_opt(12 * 3600).expect("12 * 3600 seconds to be within bounds"),
            Self::UTCp1 => FixedOffset::east_opt(1 * 3600).expect("3600 seconds to be within bounds"),
            Self::UTCp2 => FixedOffset::east_opt(2 * 3600).expect("2 * 3600 seconds to be within bounds"),
            Self::UTCp3 => FixedOffset::east_opt(3 * 3600).expect("3 * 3600 seconds to be within bounds"),
            Self::UTCp4 => FixedOffset::east_opt(4 * 3600).expect("4 * 3600 seconds to be within bounds"),
            Self::UTCp5 => FixedOffset::east_opt(5 * 3600).expect("5 * 3600 seconds to be within bounds"),
            Self::UTCp6 => FixedOffset::east_opt(6 * 3600).expect("6 * 3600 seconds to be within bounds"),
            Self::UTCp7 => FixedOffset::east_opt(7 * 3600).expect("7 * 3600 seconds to be within bounds"),
            Self::UTCp8 => FixedOffset::east_opt(8 * 3600).expect("8 * 3600 seconds to be within bounds"),
            Self::UTCp9 => FixedOffset::east_opt(9 * 3600).expect("9 * 3600 seconds to be within bounds"),
            Self::UTCp10 => FixedOffset::east_opt(10 * 3600).expect("10 * 3600 seconds to be within bounds"),
            Self::UTCp11 => FixedOffset::east_opt(11 * 3600).expect("11 * 3600 seconds to be within bounds"),
            Self::UTCp12 => FixedOffset::east_opt(12 * 3600).expect("12 * 3600 seconds to be within bounds"),
        }
    }
}

pub enum IconInput {
    ArrowLeft,
    Bell,
    Bible,
    Church,
    Expand,
    ExpandWide,
    Gear,
    Pause,
    Play,
    Shop,
    Star,
    VolumeHigh,
    VolumeLow,
    VolumeX
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
