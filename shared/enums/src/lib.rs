use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq)]
pub enum LiturgicalColors {
    Green,
    Purple,
    Rose,
    Gold,
    Red,
    Black,
    Blue,
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
    ExtraExtraLarge
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