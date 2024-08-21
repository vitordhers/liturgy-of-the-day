use leptos::*;
mod liturgy_state;
pub use liturgy_state::*;
mod navigation_state;
pub use navigation_state::*;
mod theme_state;
pub use theme_state::*;

pub fn provide_state_context() {
    let liturgy_state_ctx = create_liturgy_state_ctx();
    provide_context(liturgy_state_ctx);
    let theme_state_ctx = create_theme_state_ctx();
    provide_context(theme_state_ctx);
    let navigation_state_ctx = create_navigation_state_ctx();
    provide_context(navigation_state_ctx);
}
