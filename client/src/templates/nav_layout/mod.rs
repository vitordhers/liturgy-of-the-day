use leptos::*;
use leptos_router::Outlet;

use crate::components::navbar::Navbar;

#[component]
pub fn NavLayout() -> impl IntoView {
    view! {
      <div class="drawer lg:drawer-open">
        <input id="nav-drawer" type="checkbox" class="drawer-toggle" />
        <div class="drawer-content flex flex-col">
          <Outlet />
        </div>
        <div class="drawer-side">
          <ul class="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
            <li><a>Sidebar Item 1</a></li>
            <li><a>Sidebar Item 2</a></li>
          </ul>

        </div>
      </div>

      <Navbar />
    }
}
