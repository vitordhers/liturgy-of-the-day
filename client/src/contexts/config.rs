use enums::LiturgicalColors;
use leptos::*;

use config::{Config, UiConfig};

pub fn provide_config_context_slices(config: Config) {
    let config = create_rw_signal(config);
    create_effect(move |_| config.get()); // create_effect(move |_| config.get().save()); 

    let theme_slice = create_slice(
        config,
        |config| config.ui.theme.clone(),
        |config, theme| config.ui.theme = theme,
    );

    let ui_slice = create_slice(
        config,
        |config| config.ui.clone(),
        |config, ui| config.ui = ui,
    );

    let theme_ctx = ThemeCtx(theme_slice);
    let ui_ctx = UiConfigCtx(ui_slice);

    provide_context(config);
    provide_context(theme_ctx);
    provide_context(ui_ctx);
}

#[derive(Copy, Clone)]
pub struct ThemeCtx(pub (Signal<LiturgicalColors>, SignalSetter<LiturgicalColors>));

#[derive(Copy, Clone)]
pub struct UiConfigCtx(pub (Signal<UiConfig>, SignalSetter<UiConfig>));
