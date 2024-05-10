use enums::Timezone;
use lazy_static::lazy_static;
use std::collections::HashMap;
use structs::{Component, Layout, LocalizedRoute};

pub const LOCALE_KEY: &'static str = "LITURGY_OF_THE_DAY_LOCALE";

pub struct LabeledTimezone {
    pub label: &'static str,
    pub tz: Timezone,
}

pub const AVAILABLE_LANGUAGES: &'static [&'static str] = &["en_US", "pt_BR"];

pub const ROUTES: &'static [LocalizedRoute] = &[];

lazy_static! {
    pub static ref ROUTES_MAP: HashMap<String, Vec<LocalizedRoute>> = {
        let en_us_nav_routes = vec![
            LocalizedRoute::new("curation", Some(Component::Curation), None, vec![]),
            LocalizedRoute::new("", Some(Component::Home), None, vec![]),
        ];
        let en_us_routes = vec![
            LocalizedRoute::new("/", None, Some(Layout::Nav), en_us_nav_routes),
            LocalizedRoute::new("/settings", Some(Component::Settings), None, vec![]),
            LocalizedRoute::new("/notifications", Some(Component::Notifications), None, vec![])
        ];
        let pt_br_nav_routes = vec![
            LocalizedRoute::new("curadoria", Some(Component::Curation), None, vec![]),
            LocalizedRoute::new("", Some(Component::Home), None, vec![]),
        ];
        let pt_br_routes = vec![
            LocalizedRoute::new("/", None, Some(Layout::Nav), pt_br_nav_routes),
            LocalizedRoute::new("/configuracoes", Some(Component::Settings), None, vec![]),
            LocalizedRoute::new("/notificacoes", Some(Component::Notifications), None, vec![])
        ];
        let mut map = HashMap::new();
        map.insert(String::from("en_US"), en_us_routes);
        map.insert(String::from("pt_BR"), pt_br_routes);
        map
    };
}
