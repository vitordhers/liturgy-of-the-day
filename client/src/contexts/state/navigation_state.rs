use leptos::*;

pub fn create_navigation_state_ctx() -> NavigationStateCtx {
    let (title, set_title) = create_signal(String::default());

    NavigationStateCtx { title, set_title }
}

#[derive(Copy, Clone)]
pub struct NavigationStateCtx {
    pub title: ReadSignal<String>,
    pub set_title: WriteSignal<String>,
}
