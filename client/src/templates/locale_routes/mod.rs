use crate::i18n::Locale;
use leptos::*;
use leptos_i18n::{td_string, Locale as I18nLocale};
use leptos_meta::{Html, Title};
use leptos_router::Outlet;

#[component()]
pub fn LocaleRoutesLayout(locale: Locale) -> impl IntoView {
    let formatter = move |text: String| {
        let text = text.as_str();
        let title = match text {
            "home" => td_string!(locale, routes.home),
            "curation" => td_string!(locale, routes.curation),
            "settings" => td_string!(locale, routes.settings),
            "notifications" => td_string!(locale, routes.notifications),
            _ => td_string!(locale, routes.not_found),
        };

        format!("Agnus Daily - {title}")
    };

    view! {
        <Html
            lang=locale.as_str()
        />
        <Title formatter />
        <Outlet />
    }
}
