use invidious::{Format, Video};
use leptos::*;
use logging::log;
use utils::is_webkit;

use crate::contexts::{PlayerState, PlayerStyle};

#[component]
pub fn VideoStream(video: Video) -> impl IntoView {
    let state = expect_context::<PlayerState>();
    // let video_player_ref = create_node_ref();
    // state.set_video_player_ref(video_player_ref);
    let style = expect_context::<PlayerStyle>();
    let video_player_ref = state.video_player_ref;

    view! {
        <video
            data-fullwindow=move || style.full_window.get().to_string()
            data-fullscreen=move || style.fullscreen.get().to_string()
            on:waiting=move |_| {
                log!("waiting evt");
                let _ = state.set_video_ready(false);
            }

            on:loadedmetadata=move |_| {
                log!("loaded metadata event");

                if is_webkit() {
                    let _ = state.set_video_ready(true);
                }
            }

            on:canplay=move |_| {
                log!("can play event");
                let _ = state.set_video_ready(true);
            }

            class="w-100 max-w-max h-screen object-fit"
            node_ref=video_player_ref
            on:timeupdate=move |_| {
                let _ = state.update_time();
            }

            poster=&video.thumbnails.first().unwrap().url
            preload="auto"
            controls=false
            muted=true
            playsinline=true
        >
            <VideoSource/>
        </video>
    }
}

#[component]
pub fn VideoSource() -> impl IntoView {
    let format = expect_context::<RwSignal<Option<Format>>>();
    let source = move || format.get().map(|format| format.video_url()).flatten();

    move || view! { <source src=source/> }
}
