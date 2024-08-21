use enums::IconInput;
use leptos::*;
use leptos_i18n::td_string;
use leptos_router::{use_route, A};

use crate::{components::icon::Icon, contexts::state::NavigationStateCtx, i18n::Locale};

#[component]
pub fn BackNavbar(#[prop(default = Locale::en_US)] locale: Locale) -> impl IntoView {
    let navigation_ctx = expect_context::<NavigationStateCtx>();

    let title_memo = create_memo(move |_| {
        let title = navigation_ctx.title.get();
        match title.as_str() {
            "home" => td_string!(locale, routes.home),
            "curation" => td_string!(locale, routes.curation),
            "settings" => td_string!(locale, routes.settings),
            "notifications" => td_string!(locale, routes.notifications),
            _ => td_string!(locale, routes.not_found),
        }
    });
    view! {
        <nav class="navbar bg-base-100 w-screen lg:hidden">
            <div class="navbar-start">
                <A
                    class="btn btn-circle"
                    exact=true
                    href=|| {
                        let route = use_route();
                        route.path()
                    }
                >
                    <Icon icon_input=IconInput::ArrowLeft />
                </A>
            </div>
            <div class="navbar-center">
                {move || title_memo.get()}
            </div>
            <div class="navbar-end">
            </div>
        </nav>
    }
}
