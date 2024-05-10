use enums::IconInput;
use leptos::*;
use leptos_router::A;

use crate::components::icon::Icon;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav role="tablist" class="lg:hidden w-screen tabs tabs-boxed fixed bottom-0">
            <A
                href="/"
                class="tab"
                active_class="tab-active"
                >
                <Icon icon_input=IconInput::Bible />
            </A>
            <A
                href="/shop"
                class="tab"
                active_class="tab-active"
                >
                <Icon icon_input=IconInput::Shop size=enums::FontSize::Large />
            </A>
        </nav>
    }
}
