use lazy_static::lazy_static;
use std::collections::HashMap;
use structs::routes::{Component, Layout, Locales, LocalizedRoute, PathsMap, RoutesMap};

pub const CONFIG_KEY: &'static str = "AGNUS_DAILY_CONFIG";

pub const LOCALES: Locales = &["en-US", "pt-BR"];

lazy_static! {
    pub static ref ROUTES_MAP: RoutesMap = {
        let mut map = HashMap::new();

        let en_us_index_routes = vec![
            LocalizedRoute::new_final_route("", Component::Home, "home"),
            LocalizedRoute::new_final_route("curation", Component::Curation, "curation"),
            LocalizedRoute::new_final_route("*any", Component::NotFound, "not_found"),
        ];

        map.insert(
            ("en-US", "index"),
            LocalizedRoute::new_middle_route("", Layout::Nav, en_us_index_routes),
        );

        let en_us_dashboard_routes = vec![
            LocalizedRoute::new_final_route("settings", Component::Settings, "settings"),
            LocalizedRoute::new_final_route(
                "notifications",
                Component::Notifications,
                "notifications",
            ),
            LocalizedRoute::new_final_route("*any", Component::NotFound, "not_found"),
        ];

        let pt_br_index_routes = vec![
            LocalizedRoute::new_final_route("", Component::Home, "home"),
            LocalizedRoute::new_final_route("curadoria", Component::Curation, "curation"),
            LocalizedRoute::new_final_route("*any", Component::NotFound, "not_found"),
        ];

        let pt_br_dashboard_routes = vec![
            LocalizedRoute::new_final_route("configuracoes", Component::Settings, "settings"),
            LocalizedRoute::new_final_route(
                "notificacoes",
                Component::Notifications,
                "notifications",
            ),
            LocalizedRoute::new_final_route("*any", Component::NotFound, "not_found"),
        ];

        map.insert(
            ("en-US", "dashboard"),
            LocalizedRoute::new_middle_route("dashboard", Layout::BackNav, en_us_dashboard_routes),
        );

        map.insert(
            ("pt-BR", "index"),
            LocalizedRoute::new_middle_route("", Layout::Nav, pt_br_index_routes),
        );
        map.insert(
            ("pt-BR", "dashboard"),
            LocalizedRoute::new_middle_route("painel", Layout::BackNav, pt_br_dashboard_routes),
        );

        map
    };
    
    pub static ref PATHS_MAP: PathsMap = {
        let mut map = HashMap::new();

        map.insert(("en-US", "index"), "/");
        map.insert(("en-US", "curation"), "/curation");
        map.insert(("en-US", "settings"), "/settings");
        map.insert(("en-US", "notifications"), "/notifications");

        map.insert(("pt-BR", "index"), "/pt-BR");
        map.insert(("pt-BR", "curation"), "/pt-BR/curadoria");
        map.insert(("pt-BR", "settings"), "/pt-BR/configuracoes");
        map.insert(("pt-BR", "notifications"), "/pt-BR/notificacoes");

        map
    };
}
