use config::Config;
use leptos::*;

pub fn create_volume_config_ctx(state: RwSignal<Config>) -> VolumeConfigCtx {
    let (config, update_config) = create_slice(
        state,
        |state| state.player.volume.clone(),
        |state, volume| state.player.volume = volume,
    );

    VolumeConfigCtx {
        config,
        update_config,
    }
}

#[derive(Copy, Clone)]
pub struct VolumeConfigCtx {
    pub config: Signal<f64>,
    pub update_config: SignalSetter<f64>,
}
