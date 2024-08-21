use leptos::*;
use leptos_i18n::td;
use leptos_router::use_route_data;

use crate::{contexts::state::NavigationStateCtx, i18n::Locale};

#[component()]
pub fn NotFound() -> impl IntoView {
    let data: Option<Locale> = use_route_data();
    let locale = data.unwrap_or_default();
    let navigation_ctx = expect_context::<NavigationStateCtx>();
    navigation_ctx.set_title.set(String::from(""));

    view! {
        <div>
            {td!(locale, common.not_found)}
        </div>
    }
}
