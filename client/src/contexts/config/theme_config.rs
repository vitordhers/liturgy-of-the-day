use config::{Config, ThemeConfig};
use enums::theme::ThemeMode;
use leptos::*;

pub fn create_theme_config_ctx(state: RwSignal<Config>) -> ThemeConfigCtx {
    let (mode, set_mode) = create_slice(
        state,
        |state| state.theme.mode,
        |state, new_mode: ThemeMode| {
            state.theme.mode = new_mode;
            let _ = state.save();
        },
    );

    let (use_gold_instead_of_white, set_use_gold) = create_slice(
        state,
        |state| state.theme.use_gold_instead_of_white,
        |state, should_use_gold: bool| {
            state.theme.use_gold_instead_of_white = should_use_gold;

            let _ = state.save();
        },
    );

    let state = create_read_slice(state, |state| state.theme);

    ThemeConfigCtx {
        state,
        mode,
        set_mode,
        use_gold_instead_of_white,
        set_use_gold,
    }
}

#[derive(Clone)]
pub struct ThemeConfigCtx {
    pub state: Signal<ThemeConfig>,
    pub mode: Signal<ThemeMode>,
    pub set_mode: SignalSetter<ThemeMode>,
    pub use_gold_instead_of_white: Signal<bool>,
    pub set_use_gold: SignalSetter<bool>,
}
