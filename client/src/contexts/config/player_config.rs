use config::{Config, PlayerConfig};
use leptos::*;

pub fn create_player_config_ctx(state: RwSignal<Config>) -> PlayerConfigCtx {
    let (config, update_config) = create_slice(
        state,
        |state| state.player.clone(),
        |state, volume| state.player = volume,
    );

    PlayerConfigCtx {
        config,
        update_config,
    }
}

#[derive(Copy, Clone)]
pub struct PlayerConfigCtx {
    pub config: Signal<PlayerConfig>,
    pub update_config: SignalSetter<PlayerConfig>,
}
