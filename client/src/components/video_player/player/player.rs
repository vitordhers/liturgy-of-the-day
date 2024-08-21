use std::time::Duration;

use crate::{
    components::video_player::{
        player::{audio::AudioStream, video::VideoStream},
        VideoPlayerControls,
    },
    contexts::{
        config::{PlayerConfigCtx, ServerConfigCtx},
        state::LiturgyStateCtx,
    },
    i18n::*,
};
use invidious::{get_format, Format, Formats, Video, VideoFormat};
use leptos::{leptos_dom::helpers::TimeoutHandle, *};
use logging::log;
use structs::serialized::LiturgicalDate;
use utils::is_webkit;

use crate::contexts::{PlaybackState, PlayerState, PlayerStyle};

pub const VIDEO_CLASSES: &'static str = "
relative flex flex-col transition-all object-contain items-center justify-center duration-300
data-[controls=false]:cursor-none
data-[fullwindow=false]:w-full
data-[fullwindow=false]:ease-out
data-[fullwindow=true]:fullwindow
data-[fullwindow=true]:object-cover
data-[fullwindow=true]:ease-in";

#[component]
pub fn VideoContainer(current_date: LiturgicalDate) -> impl IntoView {
    let server_config_ctx = expect_context::<ServerConfigCtx>();
    let liturgy_state_ctx = expect_context::<LiturgyStateCtx>();

    let (current_date, _) = create_signal(current_date.clone());

    let video_resource = create_resource(
        move || {
            (
                current_date.get(),
                server_config_ctx.config.get(),
                liturgy_state_ctx.current_commemoration_index.get(),
            )
        },
        |(current_date, server, commemoration_index)| async move {
            let commemoration = if commemoration_index.is_none() {
                current_date.get_precedent_commemoration()
            } else {
                let current_commemoration_index =
                    commemoration_index.expect("commemoration_index to be some");
                let commemoration = current_date.commemorations.get(current_commemoration_index);
                commemoration.expect("Commemoration to be Some")
            };

            let (first_video_asset, _) = commemoration
                .gospel
                .video_assets
                .split_first()
                .expect("expect first_split_video_assets to be Some");
            // let video_url = format!("{server}/api/v1/videos/{}", first_video_asset.yt_id);
            // log!("THIS IS URL {:?}", video_url);
            let video = Video::fetch_video(&server, &first_video_asset.yt_id).await;
            // log!("THIS IS VIDEO {:?}", video);

            video
        },
    );

    let video_player_view = move || {
        video_resource
            .and_then(|initial_video| view! {<VideoPlayer initial_video=initial_video.clone() />})
    };

    view! {
        <Suspense fallback=move || view! { <VideoPlaceholder/> }>
        <ErrorBoundary fallback=move |error| view! { <div>"VIDEO ERROR"</div> }>
        <div>
            {video_player_view}
        </div>
        </ErrorBoundary>
        </Suspense>
    }
}

#[component]
pub fn VideoPlayer(initial_video: Video) -> impl IntoView {
    log!("video player start");
    let state = expect_context::<PlayerState>();
    let style = expect_context::<PlayerStyle>();
    let player_config_ctx = expect_context::<PlayerConfigCtx>();

    log!("before formats");
    let formats = Formats::from((
        initial_video.adaptive_formats.clone(),
        initial_video.format_streams.clone(),
    ));
    provide_context(create_rw_signal(formats.clone()));

    log!("before format");
    let format = create_memo(move |_| {
        let config = player_config_ctx.config.get();
        get_format(
            &formats,
            config.default_audio_quality,
            config.default_video_quality,
            is_webkit(),
        )
        .ok()
    });

    provide_context::<RwSignal<Option<Format>>>(create_rw_signal(format.get_untracked()));

    log!("before handle store");
    let handle_store: RwSignal<Option<TimeoutHandle>> = create_rw_signal(None);
    log!("after handle store");
    let idle_detection = move |_| {
        style.controls_visible.set(true);
        if let Some(handle) = handle_store.get() {
            handle.clear();
        }
        let handle = set_timeout_with_handle(
            move || {
                style.controls_visible.set(cursor_visible());
            },
            Duration::from_secs(3),
        )
        .unwrap();
        handle_store.set(Some(handle));
    };

    let video_container_ref = state.video_container_ref;
    
    log!("after idle detection");
    create_effect(move |_| {
        set_interval(
            move || {
                log!("set interval run");
                let _ = state.sync();
            },
            Duration::from_secs(3),
        );
    });

    log!("after set interval");
    view! {
        <div
            data-controls=move || style.controls_visible.get().to_string()
            data-fullwindow=move || style.full_window.get().to_string()
            on:click=move |_| {
                if !controls_hovered() {
                    let _ = state.toggle_playback();
                }
            }
            on:dblclick=move |_| {
                if !controls_hovered() {
                    toggle_fullscreen()
                }
            }
            on:mouseover=idle_detection
            on:mousemove=idle_detection
            class=VIDEO_CLASSES
            node_ref=video_container_ref
        >
            <VideoStream video=initial_video.clone() />
            <AudioStream />
            <VideoPlayerControls />
            <LoadingCircle />
        </div>
    }
}

#[component]
pub fn VideoPlaceholder() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col justify-center items-center bg-base-300 rounded">
            <div class="w-full aspect-video bg-base-300 rounded animate-pulse"></div>
        </div>
    }
}

#[component]
pub fn VideoFormat() -> impl IntoView {
    let url = expect_context::<RwSignal<Option<VideoFormat>>>()
        .get()
        .map(|format| format.url);

    move || view! { <source src=url.clone()/> }
}

#[component]
pub fn LoadingCircle(#[prop(default = Locale::en_US)] locale: Locale) -> impl IntoView {
    let state = expect_context::<PlayerState>();
    let classes = move || match state.playback_state.get() == PlaybackState::Loading {
        true => "absolute -translate-x-1/2 -translate-y-1/2 top-2/4 left-1/2",
        false => "hidden",
    };

    view! {
        <div role="status" class=classes>
            <svg
                aria-hidden="true"
                class="animate-spin w-8 h-8 mr-2 fill-primary"
                viewBox="0 0 100 101"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path
                    d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
                    fill="currentColor"
                ></path>
                <path
                    d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
                    fill="currentFill"
                ></path>
            </svg>
            <span class="sr-only">{move || td!(locale, common.loading)}</span>
        </div>
    }
}

fn cursor_visible() -> bool {
    true
    // !document().fullscreen() || controls_hovered()
}

fn controls_hovered() -> bool {
    false
    // let hovered_elements = document().query_selector_all(":hover").ok();

    // match hovered_elements {
    //     Some(hovered_elements) => {
    //         let mut elements_vec: Vec<Element> = Vec::new();
    //         let mut index = 0;
    //         while let Some(node) = hovered_elements.item(index) {
    //             let element: Element = node.dyn_into().unwrap();
    //             elements_vec.push(element);
    //             index = index + 1;
    //         }
    //         elements_vec
    //             .iter()
    //             .find(|element| element.id().eq(VIDEO_CONTROLS_ID))
    //             .is_some()
    //     }
    //     None => false,
    // }
}

fn toggle_fullscreen() {
    // match document().fullscreen() {
    //     true => document().exit_fullscreen(),
    //     false => {
    //         let _ = get_element_by_id::<HtmlDivElement>(VIDEO_CONTAINER_ID)
    //             .unwrap()
    //             .request_fullscreen();
    //     }
    // }
}
