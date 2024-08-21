use super::LiturgyStateCtx;
use crate::contexts::config::ThemeConfigCtx;
use leptos::{logging::log, *};

pub fn create_theme_state_ctx() -> ThemeStateCtx {
    let theme_config_ctx = expect_context::<ThemeConfigCtx>();
    let liturgy_state_ctx = expect_context::<LiturgyStateCtx>();
    let (current_theme, set_current_theme) = create_signal(String::from("white"));

    create_effect(move |_| {
        let current_date = liturgy_state_ctx.current_liturgical_date.get();
        let commemoration_index = liturgy_state_ctx.current_commemoration_index.get();

        if current_date.is_none() {
            return;
        }
        let current_date = current_date.expect("current date to be Some");
        if current_date.is_err() {
            return;
        }
        let current_date = current_date.expect("current date to be Ok");

        let current_commemoration = if commemoration_index.is_none() {
            current_date.get_precedent_commemoration()
        } else {
            let current_commemoration_index =
                commemoration_index.expect("commemoration_index to be some");
            let commemoration = current_date.commemorations.get(current_commemoration_index);
            commemoration.expect("commemoration to be Some")
        };

        let theme_color = current_commemoration
            .color
            .get_color_theme(
                theme_config_ctx.mode.get(),
                theme_config_ctx.use_gold_instead_of_white.get(),
            )
            .to_string();
        log!("LOADED color {:?}", &theme_color);
        set_current_theme.set(theme_color);
    });

    ThemeStateCtx { current_theme }
}

#[derive(Copy, Clone)]
pub struct ThemeStateCtx {
    pub current_theme: ReadSignal<String>,
}
