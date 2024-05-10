use crate::{
    pages::{
        home::Home, not_found::NotFound, notifications::Notifications, settings::Settings,
        shop::Shop,
    },
    templates::nav_layout::NavLayout,
};
use config::constants::ROUTES_MAP;
use leptos::{logging::log, *};
use leptos_meta::Html;
use leptos_router::{Outlet, Route};
use structs::{Component, Layout, LocalizedRoute};

#[component(transparent)]
pub fn LocaleRoutes(#[prop(default = String::from("en_US"))] locale: String) -> impl IntoView {
    let locale_routes = ROUTES_MAP.get(&locale.clone());
    let locale_routes = locale_routes.expect("locale routes to have key");

    let current_locale = locale.clone();

    let path = match current_locale.as_str() {
        "en_US" => "/",
        other => other,
    };

    let locale_views = locale_routes
        .into_iter()
        .map(|c| view! {<NestedRoute route=c.clone() />})
        .collect_view();

    // log!("IS SSR {}", cfg!(feature = "ssr"));
    // log!("IS CSR {}", cfg!(feature = "csr"));
    // log!("IS HYDRATE {}", cfg!(feature = "hydrate"));

    if cfg!(feature = "ssr") {
        log!("TEST TRANSPARENT {:?}", view! {<TestTransparent />}.render_to_string());
        log!("LOCALE VIEWS => {:?}", locale_views.render_to_string());
    }

    let views = view! {
        <Route path=path view=move || view! {<LocaleRoutesComponent locale=locale.clone() />}>
            {locale_routes.into_iter().map(|c| view! {<NestedRoute route=c.clone() />}).collect_view()}
        </Route>
    };

    if cfg!(feature = "ssr") {
        log!("VIEWS => {:?}", views.clone().render_to_string());
    }

    views
}

#[component(transparent)]
fn TestTransparent() -> impl IntoView {
    view! {
        <Route path="/" view=|| view! {<div> <p>this is useless</p> <Outlet /> </div>}>
            <Route path="test" view =|| view! {<span>TEST CHILD #2</span>} />
            <Route path="" view =|| view! {<span>TEST CHILD #1</span>} />
        </Route>
    }
}


#[component(transparent)]
fn LocaleRoutesComponent(locale: String) -> impl IntoView {
    view! {
        <Html
            lang=locale
        />
        <Outlet />
    }
}

#[component(transparent)]
pub fn NestedRoute(route: LocalizedRoute) -> impl IntoView {
    match route {
        LocalizedRoute::Final(route) => {
            let route_component = match route.component {
                Component::Home => || view! {<Home />},
                Component::Curation => || view! { <Shop />},
                Component::Settings => || view! {<Settings />},
                Component::Notifications => || view! { <Notifications />},
                _ => || view! { <NotFound />},
            };
            
            let final_route = view! {<Route path=route.path view=route_component.clone() />};
            if cfg!(feature = "ssr") {
                // log!(
                //     "PATH {} \nROUTE COMPONENT => {:?} \nFINAL ROUTE => {:?}",
                //     route.path,
                //     route_component().render_to_string(),
                //     final_route.clone().render_to_string(),
                // );
            }
            final_route
        }
        LocalizedRoute::Middle(route) => {
            let layout_component = match route.layout {
                Layout::Nav => move || view! {<NavLayout />},
                Layout::Outlet => move || view! {<Outlet />},
            };
            let middle_route = view! {
                <Route path=route.path view=layout_component>
                    {route.children.into_iter().map(|c| view! {<NestedRoute route=c />}).collect_view()}
                </Route>
            };
            if cfg!(feature = "ssr") {
                // log!(
                //     "MIDDLE ROUTE => {:?}",
                //     middle_route.clone().render_to_string()
                // );
            }
            middle_route
        }
    }
}
