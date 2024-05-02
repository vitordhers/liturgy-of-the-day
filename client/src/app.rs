use config::Config;
use enums::{IconInput, LiturgicalColors};
// use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use crate::components::icon::Icon;
use crate::contexts::{provide_config_context_slices, ThemeCtx};

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
    provide_config_context_slices(Config::default());
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

    let (active_tab_index, set_active_tab_index) = create_signal(0);

    let theme_ctx = expect_context::<ThemeCtx>().0;
    let theme_color = move || match theme_ctx.0.get() {
        LiturgicalColors::Green => "green",
        LiturgicalColors::Purple => "purple",
        LiturgicalColors::Rose => "rose",
        LiturgicalColors::Red => "red",
        LiturgicalColors::Gold => "gold",
        LiturgicalColors::Black => "black",
        LiturgicalColors::Blue => "blue",
    };

    view! {
        <Html
            attr:data-theme=theme_color
        />

        <main class="relative">
            <div role="tablist" class="w-screen tabs tabs-boxed fixed bottom-0">
                <a role="tab" class="tab"
                class=("tab-active", move || active_tab_index.get() == 0)
                 on:click=move |_| {
                    set_active_tab_index.set(0)
                }>
                    <Icon icon_input=IconInput::Bible />
                </a>
                <a role="tab" class="tab"
                class=("tab-active", move || active_tab_index.get() == 1)
                 on:click=move |_| {
                    set_active_tab_index.set(1)
                }>
                <Icon icon_input=IconInput::Church size=enums::FontSize::Large />
                </a>
            </div>
        </main>

    }
}
