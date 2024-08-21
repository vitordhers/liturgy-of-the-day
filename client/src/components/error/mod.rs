use agnus_daily_error::AgnusDailyError;
use leptos::*;

#[component]
pub fn AgnusDailyErrorCmp(error: AgnusDailyError) -> impl IntoView {
    view! {
        <div class="bg-inherit w-[70%] max-w-xl h-auto flex flex-col space-y-8 items-center p-2 text-base-content">
            // <img class="w-[80%]" src="ferris/wtf.svg"/>
            <h1 class="w-fit font-semibold text-xl">{error.title}</h1>
            <p class="w-fit font-normal text-base font-mono">{error.description}</p>
        </div>
    }
}
