use crate::i18n::Locale;
use config::constants::PATHS_MAP;
use enums::IconInput;
use leptos::*;
use leptos_i18n::Locale as I18nLocale;
use leptos_router::A;
use logging::log;

use crate::components::icon::Icon;

#[component]
pub fn Navbar(#[prop(default = Locale::en_US)] locale: Locale) -> impl IntoView {
    let index_path = PATHS_MAP
        .get(&(locale.as_str(), "index"))
        .expect("index path to exist at Navbar cmp");
    let curation_path = PATHS_MAP
        .get(&(locale.as_str(), "curation"))
        .expect("curation path to exist at Navbar cmp");

    view! {
        <nav role="tablist" class="lg:hidden w-screen tabs tabs-boxed fixed bottom-0">
            <A
                exact=true
                href=move || index_path.to_string()
                class="tab"
                active_class="tab-active"
                >
                <Icon icon_input=IconInput::Bible />
            </A>
            <A
                exact=true
                href=move || curation_path.to_string()
                class="tab"
                active_class="tab-active"
                >
                <Icon icon_input=IconInput::Star />
            </A>
        </nav>
    }
}
