use crate::i18n::Locale;
use config::{
    constants::{LOCALES, PATHS_MAP},
    Config,
};
use enums::{liturgy::LiturgicalCalendar, Timezone};
use leptos::*;
use leptos_i18n::Locale as I18nLocale;
use leptos_router::{use_navigate, use_router, NavigateOptions};
use std::borrow::Cow;
use structs::routes::LocalesIndexPaths;

pub fn create_locale_config_ctx(state: RwSignal<Config>) -> LocaleConfigCtx {
    let (timezone, update_timezone) = create_slice(
        state,
        |state| state.locale.timezone,
        |state, tz: Timezone| {
            let updated = state.locale.update_tz(tz);
            if updated.is_err() {
                return;
            }
            let _ = state.save();
        },
    );

    let (locale, update_locale) = create_slice(
        state,
        |state| state.locale.locale.clone(),
        |state, locale: Locale| {
            let locale_str = locale.as_str();
            let updated = state.locale.update_locale(locale_str);
            if updated.is_err() {
                return;
            }
            let _ = state.save();
            let locale_settings_path = PATHS_MAP
                .get(&(locale_str, "settings"))
                .expect("settings route to exist");
            let navigate = use_navigate();
            navigate(&locale_settings_path, NavigateOptions::default());
        },
    );

    let (calendar, update_calendar) = create_slice(
        state,
        |state| state.locale.calendar,
        |state, calendar: LiturgicalCalendar| {
            let updated = state.locale.update_calendar(calendar);
            if updated.is_err() {
                return;
            }
            let _ = state.save();
        },
    );

    let liturgy_fetch_data: Memo<(Timezone, String, String)> = create_memo(move |_| {
        (
            timezone.get(),
            locale.get().to_string(),
            calendar.get().to_string(),
        )
    });

    create_effect(move |_| {
        let current_locale_config = state.get_untracked().locale;
        let router = use_router();
        let initial_path = router.pathname().get_untracked();

        let is_path_locale_index = LOCALES.is_path_locale_index(&initial_path);

        // if path is locale index
        if is_path_locale_index && !initial_path.contains(&*current_locale_config.locale) {
            // check if initial path doesn't comply with set config

            // let locale: Locale =
            //     Locale::from_str(&current_locale_config.locale).expect("string to yield locale");
            let locale_index_path = PATHS_MAP
                .get(&(&current_locale_config.locale, "index"))
                .expect("settings route to exist");
            let navigate = use_navigate();
            navigate(locale_index_path, NavigateOptions::default());
        }
    });

    LocaleConfigCtx {
        locale,
        calendar,
        timezone,
        update_locale,
        update_calendar,
        update_timezone,
        liturgy_fetch_data,
    }
}

#[derive(Clone)]
pub struct LocaleConfigCtx {
    pub locale: Signal<Cow<'static, str>>,
    pub calendar: Signal<LiturgicalCalendar>,
    pub timezone: Signal<Timezone>,
    pub update_locale: SignalSetter<Locale>,
    pub update_calendar: SignalSetter<LiturgicalCalendar>,
    pub update_timezone: SignalSetter<Timezone>,
    pub liturgy_fetch_data: Memo<(Timezone, String, String)>,
}
