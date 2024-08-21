use std::collections::HashMap;

pub type RoutesMap = HashMap<(&'static str, &'static str), LocalizedRoute>;
pub type PathsMap = HashMap<(&'static str, &'static str), &'static str>;
pub type Locales = &'static [&'static str];
pub type LocalizedId = (String, String);

#[derive(Debug, Clone)]
pub enum LocalizedRoute {
    Middle(MiddleRoute),
    Final(FinalRoute),
    Redirect(RedirectRoute),
}

impl LocalizedRoute {
    pub fn get_path(&self) -> &str {
        match self {
            Self::Middle(m) => m.path,
            Self::Final(f) => f.path,
            Self::Redirect(f) => f.path,
        }
    }
}

impl LocalizedRoute {
    pub fn new_final_route(path: &'static str, component: Component, title: &'static str) -> Self {
        Self::Final(FinalRoute::new(path, title, component))
    }

    pub fn new_middle_route(
        path: &'static str,
        layout: Layout,
        children: Vec<LocalizedRoute>,
    ) -> Self {
        Self::Middle(MiddleRoute::new(path, layout, children))
    }

    pub fn new_redirect_route(path: &'static str, redirect_to: &'static str) -> Self {
        Self::Redirect(RedirectRoute::new(path, redirect_to))
    }
}

#[derive(Debug, Clone)]
pub struct MiddleRoute {
    pub layout: Layout,
    pub path: &'static str,
    pub children: Box<Vec<LocalizedRoute>>,
}

impl MiddleRoute {
    pub fn new(path: &'static str, layout: Layout, children: Vec<LocalizedRoute>) -> Self {
        Self {
            path,
            layout,
            children: Box::new(children),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RedirectRoute {
    pub path: &'static str,
    pub redirect_to: &'static str,
}

impl RedirectRoute {
    pub fn new(path: &'static str, redirect_to: &'static str) -> Self {
        Self { path, redirect_to }
    }
}

#[derive(Debug, Clone)]
pub struct FinalRoute {
    pub path: &'static str,
    pub title: &'static str,
    pub component: Component,
}

impl FinalRoute {
    pub fn new(path: &'static str, title: &'static str, component: Component) -> Self {
        Self {
            path,
            title,
            component,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Layout {
    Nav,
    BackNav,
    #[default]
    Outlet,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Component {
    Home,
    Curation,
    Settings,
    Notifications,
    #[default]
    NotFound,
}

pub trait ToLocaleRoute {
    fn to_locale_route(&self, locale: &str, key: &str) -> Option<LocalizedRoute>;
    fn get_locale_routes(&self, locale: &str) -> Vec<LocalizedRoute>;
    fn get_locale_path(&self, locale: &str) -> &str;
    fn get_curation_path(&self, locale: &str) -> String;
    fn get_settings_path(&self, locale: &str) -> String;
    fn get_notifications_path(&self, locale: &str) -> String;
}

impl ToLocaleRoute for RoutesMap {
    fn get_locale_routes(&self, locale: &str) -> Vec<LocalizedRoute> {
        self.iter()
            .filter_map(|((map_locale, _), route)| {
                if *map_locale != locale {
                    None
                } else {
                    Some(route.clone())
                }
            })
            .collect()
    }

    fn to_locale_route(&self, locale: &str, key: &str) -> Option<LocalizedRoute> {
        match self.get(&(locale, key)) {
            Some(route) => Some(route.clone()),
            None => None,
        }
    }

    fn get_locale_path(&self, locale: &str) -> &str {
        match locale {
            "pt-BR" => "/pt-BR",
            _ => "/",
        }
    }

    fn get_curation_path(&self, locale: &str) -> String {
        let locale_path = self.get_locale_path(locale);
        let index_route = self.get(&(locale, "index")).expect("index route to exist");
        let index_routes = match index_route {
            LocalizedRoute::Middle(middle_route) => middle_route.children.clone(),
            _ => panic!("index route should not be final route"),
        };
        let curation_route = index_routes.get(1).expect("index routes to have index 0");
        format!("{}/{}", locale_path, curation_route.get_path())
    }

    fn get_settings_path(&self, locale: &str) -> String {
        let locale_path = self.get_locale_path(locale);
        let dashboard_route = self
            .get(&(locale, "dashboard"))
            .expect("dashboard route to exist");
        let dashboard_routes = match dashboard_route {
            LocalizedRoute::Middle(middle_route) => middle_route.children.clone(),
            _ => panic!("dashboard route should not be final route"),
        };
        let settings_route = dashboard_routes
            .get(0)
            .expect("dashboard routes to have index 0");
        format!(
            "{}/{}/{}",
            locale_path,
            dashboard_route.get_path(),
            settings_route.get_path()
        )
    }

    fn get_notifications_path(&self, locale: &str) -> String {
        let locale_path = self.get_locale_path(locale);
        let dashboard_route = self
            .get(&(locale, "dashboard"))
            .expect("dashboard route to exist");
        let dashboard_routes = match dashboard_route {
            LocalizedRoute::Middle(middle_route) => middle_route.children.clone(),
            _ => panic!("dashboard route should not be final route"),
        };
        let settings_route = dashboard_routes
            .get(1)
            .expect("dashboard routes to have index 1");
        format!(
            "{}/{}/{}",
            locale_path,
            dashboard_route.get_path(),
            settings_route.get_path()
        )
    }
}

pub trait LocalesIndexPaths {
    fn get_locales_index_paths(&self) -> Vec<String>;
    fn is_path_locale_index(&self, path: &String) -> bool;
}

impl LocalesIndexPaths for Locales {
    fn get_locales_index_paths(&self) -> Vec<String> {
        self.into_iter()
            .map(|locale| {
                if *locale == "en-US" {
                    return "/".to_string();
                } else {
                    return format!("/{}", locale).to_string();
                }
            })
            .collect()
    }

    fn is_path_locale_index(&self, path: &String) -> bool {
        self.get_locales_index_paths().contains(path)
    }
}
