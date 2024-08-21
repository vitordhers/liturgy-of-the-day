use crate::components::back_navbar::BackNavbar;
use crate::components::icon::Icon;
use crate::i18n::*;
use config::constants::PATHS_MAP;
use enums::IconInput;
use leptos::*;
use leptos_router::*;
use leptos_i18n::Locale as I18nLocale;
use crate::components::navbar::Navbar;
use crate::i18n::Locale;

#[component]
pub fn NavLayout(
    #[prop(default = true)] use_bottom_nav: bool,
    #[prop(default = false)] use_top_back_nav: bool,
    #[prop(default = Locale::en_US)] locale: Locale
) -> impl IntoView {
    // let data: Option<Locale> = use_route_data();
    // let locale = data.unwrap_or_default();
    let index_path = PATHS_MAP.get(&(locale.as_str(), "index")).expect("index path to exist at NavLayout cmp");
    let curation_path = PATHS_MAP.get(&(locale.as_str(), "curation")).expect("curation path to exist at NavLayout cmp");
    let settings_path = PATHS_MAP.get(&(locale.as_str(), "settings")).expect("settings path to exist at NavLayout cmp");
    let notifications_path = PATHS_MAP.get(&(locale.as_str(), "notifications")).expect("notifications path to exist at NavLayout cmp");

    view! {
      <div class="drawer lg:drawer-open">
            <input id="nav-drawer" type="checkbox" class="drawer-toggle" />

            <div class="drawer-content flex flex-col">
              <Show when=move || use_top_back_nav>
                <BackNavbar locale />
              </Show>
              <Outlet />
              <Show when=move || use_bottom_nav>
                <Navbar locale />
              </Show>
            </div>
            
            <div class="drawer-side">
                <ul class="menu p-4 w-80 min-h-full bg-base-200 text-base-content">
                    <li>
                      <A href=move || index_path.to_string()>
                        <Icon icon_input=IconInput::Bible />
                        {td!(locale, routes.home)}
                      </A>
                    </li>
                    <li>
                      <A
                        href=move || curation_path.to_string()
                        active_class="active"
                        exact=true>
                        <Icon icon_input=IconInput::Star />
                        {td!(locale, routes.curation)}
                      </A>
                    </li>
                    <div class="divider"></div>
                    <li>
                      <A href=move || settings_path.to_string()
                        active_class="active"
                        exact=true>
                        <Icon icon_input=IconInput::Gear />
                        {td!(locale, routes.settings)}
                      </A>
                    </li>
                    <li>
                      <A href=move || notifications_path.to_string()
                        active_class="active"
                        exact=true>
                        <Icon icon_input=IconInput::Bell />
                        {td!(locale, routes.notifications)}
                      </A>
                    </li>
                </ul>
            </div>
        </div>
    }
}
