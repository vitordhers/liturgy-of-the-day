use enums::IconInput;
use leptos::*;

use crate::{
    components::icon::Icon,
    contexts::{PlaybackState, PlayerState},
};

#[component]
pub fn PauseBtn() -> impl IntoView {
    let state = expect_context::<PlayerState>();

    let btn_view = move || match state.playback_state.get() {
        PlaybackState::Playing | PlaybackState::Loading => {
            view! { <Icon icon_input=IconInput::Expand /> }
        }
        PlaybackState::Initial | PlaybackState::Paused => {
            view! { <Icon icon_input=IconInput::Play /> }
        }
    };

    view! {
        <button
            on:click=move |_| {
                let _ = state.toggle_playback();
            }

            class="btn btn-ghost btn-xs"
        >
            {btn_view}
        </button>
    }
}
