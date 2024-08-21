use config::Config;
use leptos::*;

pub fn create_server_config_ctx(state: RwSignal<Config>) -> ServerConfigCtx {
    let (config, update_config) = create_slice(
        state,
        |state| state.network.server.clone(),
        |state, server_url| state.network.server = server_url,
    );

    ServerConfigCtx {
        config,
        update_config,
    }
}

#[derive(Copy, Clone)]
pub struct ServerConfigCtx {
    pub config: Signal<String>,
    pub update_config: SignalSetter<String>,
}
