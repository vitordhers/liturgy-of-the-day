use crate::contexts::LocaleConfigCtx;
use crate::i18n::*;
use config::constants::AVAILABLE_LANGUAGES;
use enums::Timezone;
use leptos::{logging::log, *};
use strum::IntoEnumIterator;

#[component]
pub fn Settings() -> impl IntoView {
    let i18n = use_i18n();
    let LocaleConfigCtx { state, set_state } =
        use_context::<LocaleConfigCtx>().expect("locale config to have been provided");

    // log!("current state {:?}", state.get());

    let current_language = move || {
        let s = state.get();
        s.current_language
    };

    let current_tz = move || {
        let s = state.get();
        s.current_timezone.to_string()
    };

    view! {
        <div class="container mx-auto p-2">
            <div class="join join-vertical w-full">
                <div class="collapse collapse-arrow join-item border border-base-300">
                    <input type="radio" name="my-accordion-4" checked="checked" />
                    <div class="collapse-title text-xl font-medium">
                        {t!(i18n, settings.locale_settings)}
                    </div>
                    <div class="collapse-content">
                        <label class="form-control w-full max-w-xs">
                            <div class="label">
                                <span class="label-text">{t!(i18n, settings.select_language)}</span>
                            </div>
                            <select class="select select-bordered" on:change=move |ev| {
                                let new_value = event_target_value(&ev);
                                set_state.set((Some(new_value), None));
                            }>
                                {
                                    AVAILABLE_LANGUAGES.into_iter().map(|l| {
                                        view!{
                                            <option value=move || {l.to_string()} selected=move || current_language() == l.to_string()>
                                                {
                                                    match l {
                                                    &"pt_BR" => td!(Locale::pt_BR, set_locale),
                                                    _ => td!(Locale::en_US, set_locale)
                                                    }
                                                }
                                            </option>
                                        }
                                    }).collect_view()
                                }
                            </select>
                        </label>
                        <label class="form-control w-full max-w-xs">
                            <div class="label">
                                <span class="label-text">{t!(i18n, settings.select_timezone)}</span>
                            </div>
                            <select class="select select-bordered" on:change=move |ev| {
                                let new_value: String = event_target_value(&ev);
                                let new_value: Timezone = new_value.parse().unwrap_or(Timezone::UTC);
                                set_state.set((None, Some(new_value)));
                            }>
                                {
                                    Timezone::iter().map(|l| {
                                        view!{
                                            <option value=move || {l.to_string()} selected=move || current_tz() == l.to_string()>
                                                {l.to_string()}
                                            </option>
                                        }
                                    }).collect_view()
                                }
                            </select>
                        </label>
                    </div>
                </div>
                <div class="collapse collapse-arrow join-item border border-base-300">
                    <input type="radio" name="my-accordion-4" />
                    <div class="collapse-title text-xl font-medium">
                        Click to open this one and close others
                    </div>
                    <div class="collapse-content">
                        color settings
                    </div>
                </div>
            </div>
        </div>
    }
}
