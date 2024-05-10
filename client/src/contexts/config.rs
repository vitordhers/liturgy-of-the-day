use enums::{LiturgicalColor, Timezone};
use leptos::{logging, *};

use config::{Config, LocaleConfig, PlatformConfig, RoutesConfig, UiConfig};

use crate::i18n::provide_i18n_context;

pub fn provide_config_context_slices(config: Config) {
    let config = create_rw_signal(config);
    let i18n = provide_i18n_context();
    let current_locale = i18n.get_locale();

    create_effect(move |_| {
        let cfg = config.get();
        match cfg.locale.save() {
            Ok(_) => logging::log!("locale settings stored"),
            Err(_) => logging::log!("locale settings storing error"),
        }
    });

    create_effect(move |_| {
        logging::log!("config {:?}", config.get());
        logging::log!("locale: {:?}", current_locale);
    });

    let theme_slice = create_slice(
        config,
        |config| config.ui.theme.clone(),
        |config, theme| config.ui.theme = theme,
    );

    let ui_slice = create_slice(config, |config| config.ui, |config, ui| config.ui = ui);

    let platform_slice = create_slice(
        config,
        |config| config.platform.clone(),
        |config, platform| config.platform = platform,
    );

    let routes_slice = create_slice(
        config,
        |config| config.routes.clone(),
        |config, locale| {
            let available_routes = config.routes.available_routes.clone();
            config.routes = RoutesConfig {
                available_routes: available_routes.clone(),
                current_routes: available_routes
                    .into_iter()
                    .filter(|r| r.locale == locale)
                    .collect(),
            }
        },
    );

    let (locale_config, set_locale_config) = create_slice(
        config,
        |config| config.locale.clone(),
        |config, payload: (Option<String>, Option<Timezone>)| {
            let updated_locale = config.locale.update(payload);
            match updated_locale {
                Ok(updated_locale) => config.locale = updated_locale,
                Err(_) => {}
            }
        },
    );

    let theme_ctx = ThemeCtx(theme_slice);
    let ui_ctx = UiConfigCtx(ui_slice);
    let platform_ctx = PlatformConfigCtx(platform_slice);
    let routes_ctx = RoutesConfigCtx(routes_slice);
    let locale_ctx = LocaleConfigCtx {
        state: locale_config,
        set_state: set_locale_config,
    };

    provide_context(config);
    provide_context(theme_ctx);
    provide_context(ui_ctx);
    provide_context(platform_ctx);
    provide_context(routes_ctx);
    provide_context(locale_ctx);
}

#[derive(Copy, Clone)]
pub struct ThemeCtx(pub (Signal<LiturgicalColor>, SignalSetter<LiturgicalColor>));

#[derive(Copy, Clone)]
pub struct UiConfigCtx(pub (Signal<UiConfig>, SignalSetter<UiConfig>));

#[derive(Copy, Clone)]
pub struct PlatformConfigCtx(pub (Signal<PlatformConfig>, SignalSetter<PlatformConfig>));
#[derive(Clone)]
pub struct RoutesConfigCtx(pub (Signal<RoutesConfig>, SignalSetter<String>));

#[derive(Clone)]
pub struct LocaleConfigCtx {
    pub state: Signal<LocaleConfig>,
    pub set_state: SignalSetter<(Option<String>, Option<Timezone>)>,
}
