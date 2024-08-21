use crate::components::icon::Icon;
use crate::components::video_player::VideoContainer;
use crate::contexts::state::{LiturgyStateCtx, NavigationStateCtx};
use crate::contexts::{PlayerState, PlayerStyle};
use crate::i18n::Locale;
use config::constants::PATHS_MAP;
use enums::IconInput;
use leptos::*;
use leptos_i18n::Locale as I18nLocale;
use leptos_meta::Title;
use leptos_router::A;
use logging::log;

#[component]
pub fn Home(#[prop(default = Locale::en_US)] locale: Locale) -> impl IntoView {
    let settings_path = PATHS_MAP
        .get(&(locale.as_str(), "settings"))
        .expect("settings path to exist at Home cmp");
    let notifications_path = PATHS_MAP
        .get(&(locale.as_str(), "notifications"))
        .expect("notifications path to exist at Home cmp");
    let navigation_ctx = expect_context::<NavigationStateCtx>();
    navigation_ctx.set_title.set(String::from("home"));
    let liturgy_state_ctx = expect_context::<LiturgyStateCtx>();

    let state = PlayerState::init();
	let style = PlayerStyle::init();
	provide_context(state);
	provide_context(style);
    
    let container_view = move || {
        liturgy_state_ctx
            .current_liturgical_date
            .and_then(|current_date| view! {<VideoContainer current_date=current_date.clone() />})
    };

    view! {
        <Title text="home" />
        <div class="min-h-screen relative flex justify-end">
            <div class="menu menu-horizontal absolute gap-0.5 z-10">
                <A
                    href=move || settings_path.to_string()
                    class="btn btn-sm btn-circle">
                    <Icon icon_input=IconInput::Gear />
                </A>
                <A
                    href=move ||notifications_path.to_string()
                    class="btn btn-sm btn-circle">
                    <Icon icon_input=IconInput::Bell />
                </A>
            </div>
            <Suspense
                fallback=move || view! { <p>"Loading..."</p> }
            >
                <ErrorBoundary fallback=move |_| view! { <div>"CONTAINER ERROR"</div> }>
                {container_view}
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
