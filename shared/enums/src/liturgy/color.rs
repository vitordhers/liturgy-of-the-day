use crate::theme::ThemeMode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LiturgicalColor {
    #[default]
    Green,
    Black,
    WhiteOrGold,
    Purple,
    Red,
}

impl LiturgicalColor {
    pub fn get_color_theme(&self, mode: ThemeMode, use_gold_instead_of_white: bool) -> String {
        let mode = if mode == ThemeMode::Light {
            ""
        } else {
            "-dark"
        };
        let color = match self {
            Self::Red => "red",
            Self::Purple => "purple",
            Self::WhiteOrGold => {
                if use_gold_instead_of_white {
                    "gold"
                } else {
                    "white"
                }
            }
            Self::Black => "black",
            Self::Green => "green",
        };
        format!("{}{}", color, mode)
    }
}
