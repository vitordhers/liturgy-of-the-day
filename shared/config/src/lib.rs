use liturgy_of_the_day_error::LiturgyOfTheDayError;
use enums::{LiturgicalColor, Timezone};
use gloo::storage::{LocalStorage, Storage};
use leptos::leptos_dom::{is_browser, is_server};
use serde::{Deserialize, Serialize};
pub mod constants;
use constants::LOCALE_KEY;
use serde_json::{from_str, to_string};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub ui: UiConfig,
    pub platform: PlatformConfig,
    pub routes: RoutesConfig,
    pub locale: LocaleConfig,
}

impl Default for Config {
    fn default() -> Self {
        let ui = UiConfig::default();
        let platform = PlatformConfig::default();
        let routes = RoutesConfig::default();
        let locale = LocaleConfig::default();

        Self {
            ui,
            platform,
            routes,
            locale,
        }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct UiConfig {
    pub theme: LiturgicalColor,
    pub font_scale: u8,
    pub ui_scale: u8,
}

impl Default for UiConfig {
    fn default() -> Self {
        let theme = LiturgicalColor::Green;
        let font_scale = 100u8;
        let ui_scale = 100u8;

        Self {
            theme,
            font_scale,
            ui_scale,
        }
    }
}

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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct NavigationRoute {
    pub locale: String,
    pub key: String,
    pub path: String,
}

impl NavigationRoute {
    fn new(locale: &str, key: &str, path: &str) -> Self {
        Self {
            locale: locale.into(),
            key: key.into(),
            path: path.into(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct RoutesConfig {
    pub available_routes: Vec<NavigationRoute>,
    pub current_routes: Vec<NavigationRoute>,
}

impl Default for RoutesConfig {
    fn default() -> Self {
        let english_us_routes = vec![
            NavigationRoute::new("en_US", "home", "/"),
            NavigationRoute::new("en_US", "curation", "/curation"),
            NavigationRoute::new("en_US", "settings", "/settings"),
            NavigationRoute::new("en_US", "notifications", "/notifications"),
        ];
        let portuguese_br_routes = vec![
            NavigationRoute::new("pt_BR", "home", "/"),
            NavigationRoute::new("pt_BR", "curation", "/curadoria"),
            NavigationRoute::new("pt_BR", "settings", "/configuracoes"),
            NavigationRoute::new("pt_BR", "notifications", "/notifications"),
        ];
        let available_routes = [english_us_routes.clone(), portuguese_br_routes].concat();
        let current_routes = english_us_routes;

        Self {
            available_routes,
            current_routes,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct LocaleConfig {
    pub current_timezone: Timezone,
    pub current_language: String,
}

impl Default for LocaleConfig {
    fn default() -> Self {
        let default_locale_config = Self {
            current_language: String::from("en_US"),
            current_timezone: Timezone::UTCm5,
        };
        // LocalStorage is not available in server
        if is_server() {
            return default_locale_config;
        }
        match LocalStorage::get::<String>(LOCALE_KEY) {
            Ok(config_str) => Self::from_json_str(&config_str).unwrap_or(default_locale_config),
            Err(_) => default_locale_config,
        }
    }
}

impl LocaleConfig {
    pub fn save(&self) -> Result<(), LiturgyOfTheDayError> {
        LocalStorage::set(LOCALE_KEY, self.to_json_str()?)?;
        Ok(())
    }

    pub fn update(
        &self,
        payload: (Option<String>, Option<Timezone>),
    ) -> Result<Self, LiturgyOfTheDayError> {
        let mut updated_locale_config = self.clone();
        if payload.0.is_some() {
            updated_locale_config.current_language = payload.0.unwrap();
            return Ok(updated_locale_config);
        }

        if payload.1.is_some() {
            updated_locale_config.current_timezone = payload.1.unwrap();
            return Ok(updated_locale_config);
        }

        Ok(updated_locale_config)
    }

    fn from_json_str(json_str: &str) -> Result<Self, LiturgyOfTheDayError> {
        from_str(json_str)?
    }

    fn to_json_str(&self) -> Result<String, LiturgyOfTheDayError> {
        Ok(to_string(self)?)
    }
}
