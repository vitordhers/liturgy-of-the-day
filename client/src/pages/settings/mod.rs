use crate::{contexts::{config::{LocaleConfigCtx, ThemeConfigCtx}, state::NavigationStateCtx}, i18n::*};
use config::constants::LOCALES;
use enums::liturgy::LiturgicalCalendar;
use enums::theme::ThemeMode;
use enums::Timezone;
use leptos::*;
use leptos_i18n::Locale as I18nLocale;
use leptos_meta::Title;
use serde_json::{from_str, to_string};
use strum::IntoEnumIterator;

#[component]
pub fn Settings(#[prop(default = Locale::en_US)] locale: Locale) -> impl IntoView {
    let LocaleConfigCtx {
        timezone,
        calendar,
        locale: current_locale,
        update_locale,
        update_calendar,
        update_timezone,
        liturgy_fetch_data: _,
    } = use_context::<LocaleConfigCtx>().expect("LocaleConfigCtx to be provided");
    let ThemeConfigCtx {
        mode,
        set_mode,
        state: _,
        use_gold_instead_of_white,
        set_use_gold,
    } = use_context().expect("ThemeConfigCtx to be provided");

    let navigation_state_ctx = expect_context::<NavigationStateCtx>();
    navigation_state_ctx.set_title.set(String::from("settings"));

    let theme_label = create_memo(move |_| {
        if mode.get() == ThemeMode::Light {
            td_string!(locale, settings.use_dark_mode)
        } else {
            td_string!(locale, settings.use_light_mode)
        }
    });

    let use_white_label = create_memo(move |_| {
        if use_gold_instead_of_white.get() {
            td_string!(locale, settings.use_white_instead_of_gold)
        } else {
            td_string!(locale, settings.use_gold_instead_of_white)
        }
    });

    view! {
        <Title text="settings" />
        <div class="container mx-auto p-2">
            <div class="join join-vertical w-full">
                <div class="collapse collapse-arrow join-item border border-base-300">
                    <input type="radio" name="settings-radio" checked="checked" />
                    <div class="collapse-title text-xl font-medium">
                        {td!(locale, settings.locale_settings)}
                    </div>
                    <div class="collapse-content">
                        <label class="form-control w-full max-w-xs">
                            <div class="label">
                                <span class="label-text">{td!(locale, settings.select_language)}</span>
                            </div>
                            <select class="select select-bordered" on:change=move |ev| {
                                let updated_locale = event_target_value(&ev);
                                match Locale::from_str(&updated_locale) {
                                    Some(updated_locale) => update_locale.set(updated_locale),
                                    _ => {}
                                };
                            }>
                                {
                                    LOCALES.into_iter().map(|l| {
                                        view!{
                                            <option value=move || {l.to_string()} selected=move || current_locale.with(|cl| cl == l)>
                                                {
                                                    match l {
                                                    &"pt-BR" => td!(Locale::pt_BR, set_locale),
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
                                <span class="label-text">{td!(locale, settings.select_liturgical_calendar)}</span>
                            </div>
                            <select class="select select-bordered" on:change=move |ev| {
                                let new_value: String = event_target_value(&ev);
                                match from_str(new_value.as_str()) {
                                    Ok(updated_calendar) => update_calendar.set(updated_calendar),
                                    _ => {}
                                };
                            }>
                                {
                                    LiturgicalCalendar::iter().map(|cl| {
                                        view!{
                                            <option value=move || {to_string(&cl).expect("calendar to be JSON parseable")}
                                                selected=move || calendar.with(|ccl| to_string(ccl).expect("calendar to be JSON parseable") == to_string(&cl).expect("calendar to be JSON parseable")) >
                                                {move ||{
                                                    let calendar_key = to_string(&cl).expect("calendar to be JSON parseable").replace("\"", "");
                                                    match calendar_key.as_str() {
                                                        "br" => td_string!(locale, settings.calendars.br),
                                                        "us" => td_string!(locale, settings.calendars.us),
                                                        _ => td_string!(locale, settings.calendars.roman)
                                                    }
                                                }}
                                            </option>
                                        }
                                    }).collect_view()
                                }
                            </select>
                        </label>
                        <label class="form-control w-full max-w-xs">
                            <div class="label">
                                <span class="label-text">{td!(locale, settings.select_timezone)}</span>
                            </div>
                            <select class="select select-bordered" on:change=move |ev| {
                                let new_value: String = event_target_value(&ev);
                                match new_value.parse::<Timezone>() {
                                    Ok(updated_tz) => update_timezone.set(updated_tz),
                                    _ => {}
                                };
                            }>
                                {
                                    Timezone::iter().map(|tz| {
                                        view!{
                                            <option value=move || {tz.to_string()} selected=move || timezone.with(|ctz| ctz.to_string() == tz.to_string()) >
                                                {tz.to_string()}
                                            </option>
                                        }
                                    }).collect_view()
                                }
                            </select>
                        </label>
                    </div>
                </div>
                <div class="collapse collapse-arrow join-item border border-base-300">
                    <input type="radio" name="settings-radio" />
                    <div class="collapse-title text-xl font-medium">
                    {td!(locale, settings.theme_settings)}
                    </div>
                    <div class="collapse-content">
                        <div>
                            <div class="label">
                                <span class="label-text">
                                {
                                    move || theme_label.get()
                                }
                                </span>
                            </div>
                            <label class="swap swap-rotate">
                                <input type="checkbox" class="theme-controller"
                                    checked=move || mode.get() == ThemeMode::Dark
                                    on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    if checked {
                                        set_mode.set(ThemeMode::Dark);
                                    } else {
                                        set_mode.set(ThemeMode::Light);
                                    }
                                } />
                                <svg class="swap-on fill-current w-10 h-10" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z"/></svg>
                                <svg class="swap-off fill-current w-10 h-10" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z"/></svg>
                            </label>
                        </div>
                        <div>
                            <div class="label">
                                <span class="label-text">
                                {
                                    move || use_white_label.get()
                                }
                                </span>
                            </div>
                            <input type="checkbox"
                                class="toggle theme-controller row-start-1 col-start-1 col-span-2"
                                checked=move || use_gold_instead_of_white.get()
                                on:change=move |ev| {
                                    let checked = event_target_checked(&ev);
                                    if checked {
                                        set_use_gold.set(true);
                                    } else {
                                        set_use_gold.set(false);
                                    }
                                }
                                attr:data-theme= move ||{
                                    if mode.get() == ThemeMode::Dark {
                                        if use_gold_instead_of_white.get() {
                                            "white-dark"
                                        } else {
                                            "gold-dark"
                                        }
                                    } else {
                                        if use_gold_instead_of_white.get() {
                                            "white"
                                        } else {
                                            "gold"
                                        }
                                    }
                                }
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
