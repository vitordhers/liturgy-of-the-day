use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq)]
pub enum LiturgicalColor {
    Green,
    Violet,
    Rose,
    Blue,
    WhiteOrGold,
    Red,
    Black,
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

pub enum IconInput {
    Bible,
    Church,
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
