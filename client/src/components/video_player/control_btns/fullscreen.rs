use enums::IconInput;
use leptos::*;

use crate::{
    components::icon::Icon,
    contexts::{PlayerState, PlayerStyle},
};

#[component]
pub fn FullScreenBtn() -> impl IntoView {
    let style = expect_context::<PlayerStyle>();
    let state = expect_context::<PlayerState>();

    let fullscreen = move |_| 
    if cfg!(feature="ssr") {
        return;
    };

    match document().fullscreen() {
        true => document().exit_fullscreen(),
        false => {
            let fullscreen = state.video_container_ref
                .get()
                .unwrap()
                .request_fullscreen();
            if fullscreen.is_ok() {
                style.fullscreen.set(true);
            }
        }
    };

    view! {
        <button on:click=fullscreen class="btn btn-ghost btn-xs">
            <Icon icon_input=IconInput::Expand />
        </button>
    }
}
