use agnus_daily_error::AgnusDailyError;
use enums::{liturgy::LiturgicalCalendar, theme::ThemeMode, Timezone};
use gloo::storage::{LocalStorage, Storage};
use leptos::{
    leptos_dom::{is_browser, is_server},
    logging::log,
};
use serde::{Deserialize, Serialize};
pub mod constants;
use constants::CONFIG_KEY;
use invidious::{AudioQuality, QualityLabel};
use serde_json::{from_str, to_string};
use std::borrow::Cow;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Config<'a> {
    pub locale: LocaleConfig<'a>,
    pub player: PlayerConfig,
    pub network: NetworkConfig,
    pub theme: ThemeConfig,
}

impl Default for Config<'_> {
    fn default() -> Self {
        let locale = LocaleConfig::default();
        let network = NetworkConfig::default();
        let player = PlayerConfig::default();
        let theme = ThemeConfig::default();

        Self {
            locale,
            network,
            player,
            theme,
        }
    }
}

impl Config<'_> {
    pub fn save(&self) -> Result<(), AgnusDailyError> {
        LocalStorage::set(CONFIG_KEY, self.to_json_str()?)?;
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), AgnusDailyError> {
        if is_server() {
            return Ok(());
        }
        match Self::from_json_str(&LocalStorage::get::<String>(CONFIG_KEY)?) {
            Ok(loaded) => {
                *self = loaded;
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn from_json_str(json_str: &str) -> Result<Self, AgnusDailyError> {
        Ok(from_str(json_str)?)
    }

    fn to_json_str(&self) -> Result<String, AgnusDailyError> {
        Ok(to_string(self)?)
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ThemeConfig {
    pub mode: ThemeMode,
    pub use_gold_instead_of_white: bool,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        let mode = ThemeMode::Light;
        let use_gold_instead_of_white = false;

        Self {
            mode,
            use_gold_instead_of_white,
        }
    }
}

// impl ThemeConfig {
//     pub fn get_color_theme(&self) -> &str {
//         let mut color_mode = match self.theme_mode {
//             ThemeMode::Light => "",
//             ThemeMode::Dark => "dark_",
//         };
//         let color_theme = match self.color {
//             LiturgicalColor::Red => "red",
//             LiturgicalColor::Violet => "purle",
//             LiturgicalColor::WhiteOrGold(is_gold) => {
//                 if *is_gold {
//                     "gold"
//                 } else {
//                     "white"
//                 }
//             }
//             LiturgicalColor::Green => "green",
//             LiturgicalColor::Black => "black",
//         };

//         format!("{}{}", color_mode, color_theme).as_str()
//     }
// }

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct PlatformConfig {
    pub is_server: bool,
    pub is_browser: bool,
    pub is_csr: bool,
    pub is_ssr: bool,
    pub is_hydrate: bool,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            is_server: is_server(),
            is_browser: is_browser(),
            is_csr: cfg!(feature = "csr"),
            is_ssr: cfg!(feature = "ssr"),
            is_hydrate: cfg!(feature = "hydrate"),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct LocaleConfig<'a> {
    pub timezone: Timezone,
    pub locale: Cow<'a, str>,
    pub calendar: LiturgicalCalendar,
}

impl Default for LocaleConfig<'_> {
    fn default() -> Self {
        Self {
            locale: std::borrow::Cow::Borrowed("en-US"),
            timezone: Timezone::UTCm5,
            calendar: LiturgicalCalendar::GeneralRoman,
        }
    }
}

impl LocaleConfig<'_> {
    pub fn update_locale(&mut self, locale: &'static str) -> Result<(), AgnusDailyError> {
        match locale {
            "en-US" => {
                self.update_calendar(LiturgicalCalendar::UnitedStatesOfAmerica)?;
                self.update_tz(Timezone::UTCm5)?;
            }
            "pt-BR" => {
                self.update_calendar(LiturgicalCalendar::Brazil)?;
                self.update_tz(Timezone::UTCm3)?;
            }
            _ => {
                self.update_calendar(LiturgicalCalendar::GeneralRoman)?;
                self.update_tz(Timezone::UTC)?;
            }
        }
        log!("updated locale {}", locale);
        self.locale = std::borrow::Cow::Borrowed(locale);

        Ok(())
    }

    pub fn update_tz(&mut self, tz: Timezone) -> Result<(), AgnusDailyError> {
        self.timezone = tz;
        Ok(())
    }

    pub fn update_calendar(&mut self, calendar: LiturgicalCalendar) -> Result<(), AgnusDailyError> {
        self.calendar = calendar;
        Ok(())
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PlayerConfig {
    pub auto_play: bool,
    pub fast_forward_interval: u8,
    pub default_video_quality: QualityLabel,
    pub default_audio_quality: AudioQuality,
    pub volume: f64,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        let auto_play = true;
        let fast_forward_interval = 10u8;
        let default_video_quality = QualityLabel::_1080p;
        let default_audio_quality = AudioQuality::Medium;
        let volume = 0.5f64;

        Self {
            auto_play,
            volume,
            fast_forward_interval,
            default_video_quality,
            default_audio_quality,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct NetworkConfig {
    pub server: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            server: String::from("invidious.incogniweb.net"),
        }
    }
}
