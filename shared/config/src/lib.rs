use enums::LiturgicalColors;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub ui: UiConfig,
}

impl Default for Config {
    fn default() -> Self {
        let ui = UiConfig::default();

        Self { ui }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct UiConfig {
    pub theme: LiturgicalColors,
    pub font_scale: u8,
    pub ui_scale: u8,
}

impl Default for UiConfig {
    fn default() -> Self {
        // TODO: adds logic to get the liturgial color based in the current day
        let theme = LiturgicalColors::Green;
        let font_scale = 100u8;
        let ui_scale = 100u8;

        Self {
            theme,
            font_scale,
            ui_scale,
        }
    }
}
