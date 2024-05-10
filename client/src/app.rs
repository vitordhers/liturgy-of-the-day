use crate::pages::home::Home;
use crate::pages::notifications::Notifications;
use crate::pages::settings::Settings;
use crate::pages::shop::Shop;
use crate::templates::nav_layout::NavLayout;
use crate::{i18n::provide_i18n_context, pages::locale_routes::LocaleRoutes};
use leptos::logging::log;
// use leptos::leptos_dom::logging::console_log;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// use leptos_router::*;

use config::{constants::AVAILABLE_LANGUAGES, Config};
// use leptos::leptos_dom::ev::SubmitEvent;
use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::contexts::{provide_config_context_slices, PlatformConfigCtx, ThemeCtx};
// use enums::LiturgicalColor;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_i18n_context();
    provide_config_context_slices(Config::default());

    // let current_locale = i18n.get_locale();

    // println!("current locale {:?}", current_locale);

    // let (name, set_name) = create_signal(String::new());
    // let (greet_msg, set_greet_msg) = create_signal(String::new());

    // let update_name = move |ev| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };

    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         let name = name.get_untracked();
    //         if name.is_empty() {
    //             return;
    //         }

    //         let args = to_value(&GreetArgs { name: &name }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };

    // let theme_ctx = expect_context::<ThemeCtx>().0;
    let theme_color = "purple";
    //  move || match theme_ctx.0.get() {
    //     LiturgicalColor::Green => "green",
    //     LiturgicalColor::Purple => "purple",
    //     LiturgicalColor::Rose => "rose",
    //     LiturgicalColor::Red => "red",
    //     LiturgicalColor::Gold => "gold",
    //     LiturgicalColor::Black => "black"
    // };
    // let platform_ctx = expect_context::<PlatformConfigCtx>().0;

    // let routes_view = view! {
    //
    // };
    // log!("FRAGMENT INTO VIEW {:?}", routes_view.clone().into_view().render_to_string());

    let single_route = view! {
        <Route path="" view=NavLayout ssr=SsrMode::PartiallyBlocked>
            <Html lang="en-US" />
            <Route path="shop" view=Shop />
            <Route path="" view=Home />
        </Route>
    };

    if cfg!(feature = "ssr") {
        log!("SINGLE ROUTE => {:?}", single_route.render_to_string());
    }

    view! {
        <Html
            attr:data-theme=theme_color
        />
        <Stylesheet id="leptos" href="assets/css/styles.css"/>

        <main class="relative">
            <Router>
                <Routes>
                    <LocaleRoutes locale=String::from("en_US") />
                    // <Route path="/" view=NavLayout ssr=SsrMode::PartiallyBlocked>
                    //     <Html lang="en-US" />
                    //     <Route path="shop" view=Shop />
                    //     <Route path="" view=Home />
                    // </Route>
                    // <Route path="/settings" view=Settings />
                    // <Route path="/notifications" view=Notifications />
                </Routes>
            </Router>
        </main>
    }
}
