use std::collections::HashMap;

use chrono::{Datelike, Utc};
use enums::LiturgicalColor;
use serde::{Deserialize, Serialize};

mod constants;
mod functions;

use constants::{fixed_liturgy_days::LITURGY_FIXED_DAYS};
use functions::{calculate_easter_sunday, previous_weekday_before, next_weekday_after};

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

pub fn get_prefedined_dates(day: u32, month: u32, year: i32) -> (&'static str, LiturgicalColor) {
    let mut map: HashMap<(u32, u32), (&'static str, LiturgicalColor)> = HashMap::new();

    LITURGY_FIXED_DAYS.iter().for_each(|&(date, content)| {
        map.insert(date, content);
    });

    if map.contains_key(&(day, month)) {
        let result = map.get(&(day, month)).expect("map to has date tuple");
        return result.clone();
    }

    let easter_sunday = calculate_easter_sunday(year);

    if easter_sunday.day() == day && easter_sunday.month() == month {
        return (
            "Feast of the Resurrection of the Lord",
            LiturgicalColor::Gold,
        );
    }

    let good_friday = previous_weekday_before(easter_sunday, chrono::Weekday::Fri);

    if good_friday.day() == day && good_friday.month() == month {
        return (
            "Great and Holy Friday",
            LiturgicalColor::Red,
        );
    }

    (
        "Sunday and Ferias in Ordinary Time",
        LiturgicalColor::Green,
    )
}

pub fn get_liturgical_period() -> LiturgicalColor {
    let now = Utc::now();
    let current_year = now.year();
    let current_month = now.month();
    let current_day = now.day();

    LiturgicalColor::Green
}
