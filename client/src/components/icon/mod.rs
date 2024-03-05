use enums::{FontSize, IconInput};
use leptos::*;

#[component]
pub fn Icon(
    icon_input: IconInput,
    #[prop(default = FontSize::Base)] size: FontSize,
) -> impl IntoView {
    let class_size: &str = size.into();
    view! {
        <svg xmlns="http://www.w3.org/2000/svg"
            class=(format!("icon h-{} w-{}", class_size, class_size))
            aria-hidden="true"
            focusable="false"
            viewBox="0 0 512 512">
            {
                move || match icon_input {
                    IconInput::Bible => {
                        view! {<path d="M96 0C43 0 0 43 0 96V416c0 53 43 96 96 96H384h32c17.7 0 32-14.3 32-32s-14.3-32-32-32V384c17.7 0 32-14.3 32-32V32c0-17.7-14.3-32-32-32H384 96zm0 384H352v64H96c-17.7 0-32-14.3-32-32s14.3-32 32-32zM208 80c0-8.8 7.2-16 16-16h32c8.8 0 16 7.2 16 16v48h48c8.8 0 16 7.2 16 16v32c0 8.8-7.2 16-16 16H272V304c0 8.8-7.2 16-16 16H224c-8.8 0-16-7.2-16-16V192H160c-8.8 0-16-7.2-16-16V144c0-8.8 7.2-16 16-16h48V80z"/>}
                    }
                    IconInput::Church => {
                        view! {<path d="M344 24c0-13.3-10.7-24-24-24s-24 10.7-24 24V48H264c-13.3 0-24 10.7-24 24s10.7 24 24 24h32v46.4L183.3 210c-14.5 8.7-23.3 24.3-23.3 41.2V512h96V416c0-35.3 28.7-64 64-64s64 28.7 64 64v96h96V251.2c0-16.9-8.8-32.5-23.3-41.2L344 142.4V96h32c13.3 0 24-10.7 24-24s-10.7-24-24-24H344V24zM24.9 330.3C9.5 338.8 0 354.9 0 372.4V464c0 26.5 21.5 48 48 48h80V273.6L24.9 330.3zM592 512c26.5 0 48-21.5 48-48V372.4c0-17.5-9.5-33.6-24.9-42.1L512 273.6V512h80z"/>}
                    }
                }
            }
        </svg>
    }
}
