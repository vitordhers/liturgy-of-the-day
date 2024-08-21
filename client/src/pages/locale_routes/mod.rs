// use crate::{
//     i18n::Locale,
//     pages::{
//         curation::Curation, home::Home, not_found::NotFound, notifications::Notifications,
//         settings::Settings,
//     },
//     templates::nav_layout::NavLayout,
// };
// use agnus_daily_error::AgnusDailyError;
// use config::constants::ROUTES_MAP;
// use leptos::*;
// use leptos_i18n::td_string;
// use leptos_meta::{Html, Title};
// use leptos_router::{
//     use_route_data, NavigateOptions, Outlet, Redirect, RedirectProps, Route, RouteProps,
//     TrailingSlash,
// };
// use std::str::FromStr;

// #[component(transparent)]
// pub fn LocaleRoutes(#[prop(default = "en-US")] locale: &'static str) -> impl IntoView {
//     let locale_routes = ROUTES_MAP.get_locale_routes(locale);

//     let i18n_locale: Locale = locale.into();

//     let path = i18n_locale.get_path();

//     Route(
//         RouteProps::builder()
//             .path(path)
//             .view(move || view! { <LocaleRoutesComponent locale=locale.to_string() />})
//             .trailing_slash(TrailingSlash::Drop)
//             .children(ToChildren::to_children(move || {
//                 Fragment::lazy(|| {
//                     locale_routes
//                         .into_iter()
//                         .map(|r| view! {<NestedRoute localized_route=r locale=i18n_locale />})
//                         .collect()
//                 })
//             }))
//             .build(),
//     )
// }

// #[component(transparent)]
// pub fn RedirectWrapper() -> impl IntoView {
//     let data: Option<&'static str> = use_route_data();
//     let redirect_to = data.expect("redirect_to to exist");

//     view! {<Route path="" view=move || view! {<Redirect path=redirect_to /> } />}
// }

// #[component(transparent)]
// pub fn NestedRoute(localized_route: LocalizedRoute, locale: Locale) -> impl IntoView {
//     let result = match localized_route {
//         LocalizedRoute::Final(route) => Route(
//             RouteProps::builder()
//                 .path(route.path)
//                 .data(move || locale)
//                 .view(match route.component {
//                     Component::Home => || view! {<Home />},
//                     Component::Curation => || view! {<Shop />},
//                     Component::Settings => || view! {<Settings />},
//                     Component::Notifications => || view! {<Notifications />},
//                     _ => || view! {<NotFound />},
//                 })
//                 .build(),
//         ),
//         LocalizedRoute::Middle(route) => Route(
//             RouteProps::builder()
//                 .path(route.path)
//                 .trailing_slash(TrailingSlash::Drop)
//                 .view(match route.layout {
//                     Layout::Nav => || view! {<NavLayout />},
//                     Layout::BackNav => {
//                         || view! {<NavLayout use_bottom_nav=false use_top_back_nav=true />}
//                     }
//                     Layout::Outlet => || view! {<Outlet />},
//                 })
//                 .data(move || locale)
//                 .children(ToChildren::to_children(move || {
//                     Fragment::lazy(|| {
//                         route
//                             .children
//                             .into_iter()
//                             .map(|r| view! {<NestedRoute localized_route=r locale=locale />})
//                             .collect()
//                     })
//                 }))
//                 .build(),
//         ),
//         LocalizedRoute::Redirect(route) => Route(
//             RouteProps::builder()
//                 .path(route.path)
//                 .data(move || route.redirect_to)
//                 .view(match route.redirect_to {
//                     redirect_to => || view! {<RedirectWrapper />},
//                     _ => || view! {<NotFound />},
//                 })
//                 .build(),
//         ),
//     };

//     result
// }

// #[component()]
// fn LocaleRoutesComponent(locale: String) -> impl IntoView {
//     let parsed_locale = Locale::from_str(&locale).expect("locale to be parseable");

//     let formatter = move |text: String| {
//         let text = text.as_str();
//         let title = match text {
//             "home" => td_string!(parsed_locale, routes.home),
//             "curation" => td_string!(parsed_locale, routes.curation),
//             "settings" => td_string!(parsed_locale, routes.settings),
//             "notifications" => td_string!(parsed_locale, routes.notifications),
//             _ => td_string!(parsed_locale, routes.not_found),
//         };

//         format!("Agnus Daily - {title}")
//     };

//     view! {
//         <Html
//             lang=locale
//         />
//         <Title formatter />
//         <Outlet />
//     }
// }

// impl Locale {
//     pub fn get_path(&self) -> &'static str {
//         match self {
//             Locale::en_US => "",
//             Locale::pt_BR => "/pt-BR",
//         }
//     }
// }

// impl ToString for Locale {
//     fn to_string(&self) -> String {
//         match self {
//             Locale::en_US => String::from("en-US"),
//             Locale::pt_BR => String::from("pt-BR"),
//         }
//     }
// }

// impl FromStr for Locale {
//     type Err = AgnusDailyError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "pt-BR" => Ok(Locale::pt_BR),
//             _ => Ok(Locale::en_US),
//         }
//     }
// }

// impl Into<Locale> for &str {
//     fn into(self) -> Locale {
//         match self {
//             "pt-BR" => Locale::pt_BR,
//             _ => Locale::en_US,
//         }
//     }
// }

// impl Into<&str> for Locale {
//     fn into(self) -> &'static str {
//         match self {
//             Self::en_US => "en-US",
//             Self::pt_BR => "pt-BR",
//         }
//     }
// }
