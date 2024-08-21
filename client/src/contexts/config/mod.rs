use config::{Config, PlatformConfig};
use leptos::*;

mod locale_config;
pub use locale_config::*;
mod player_config;
pub use player_config::*;
mod server_config;
pub use server_config::*;
mod theme_config;
pub use theme_config::*;
mod volume_config;
pub use volume_config::*;

pub fn provide_config_context() {
    let mut config = Config::default();
    let _ = config.load();
    let config = create_rw_signal(config);

    let locale_config_ctx = create_locale_config_ctx(config);
    provide_context(locale_config_ctx);
    let player_config_ctx = create_player_config_ctx(config);
    provide_context(player_config_ctx);
    let server_config_ctx = create_server_config_ctx(config);
    provide_context(server_config_ctx);
    let theme_config_ctx = create_theme_config_ctx(config);
    provide_context(theme_config_ctx);
    let volume_config_ctx = create_volume_config_ctx(config);
    provide_context(volume_config_ctx);
    // provide_context(platform_ctx);
}

#[derive(Copy, Clone)]
pub struct PlatformConfigCtx(pub (Signal<PlatformConfig>, SignalSetter<PlatformConfig>));
