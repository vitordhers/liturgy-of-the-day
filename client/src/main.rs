mod app;

use app::*;
use leptos::*;

mod contexts;
mod components;

fn main() {
    mount_to_body(|| {
        view! { <App/> }
    })
}
