use crate::{contexts::state::NavigationStateCtx, i18n::Locale};
use leptos::*;
use leptos_i18n::Locale as I18nLocale;
use leptos_meta::Title;

#[component]
pub fn Curation(#[prop(default = Locale::en_US)] locale: Locale) -> impl IntoView {
    let navigation_ctx = expect_context::<NavigationStateCtx>();
    navigation_ctx.set_title.set(String::from("curation"));

    view! {
        <Title text="curation" />
        <h1>Curation page</h1>
    }
}
