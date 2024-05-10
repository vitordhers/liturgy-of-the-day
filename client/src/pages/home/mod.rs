use enums::IconInput;
use leptos::*;
use leptos_router::A;

use crate::components::icon::Icon;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen flex justify-end gap-0.5">
            <A
                href="/settings"
                class="btn btn-sm btn-circle">
                <Icon icon_input=IconInput::Gear size=enums::FontSize::Base />
            </A>
            <A
                href="/notifications"
                class="btn btn-sm btn-circle">
                <Icon icon_input=IconInput::Bell size=enums::FontSize::Base />
            </A>
        </div>
    }
}
