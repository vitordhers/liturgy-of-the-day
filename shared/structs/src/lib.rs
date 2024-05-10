// pub struct LocalizedRoute {
//     pub view: ViewType,
//     pub path: &'static str,
//     pub locale: &'static str,
//     pub children: Box<Vec<LocalizedRoute>>,
// }

#[derive(Debug, Clone)]
pub enum LocalizedRoute {
    Middle(MiddleRoute),
    Final(FinalRoute),
}

impl LocalizedRoute {
    pub fn new(
        path: &'static str,
        component: Option<Component>,
        layout: Option<Layout>,
        children: Vec<LocalizedRoute>,
    ) -> Self {
        if children.len() != 0 && layout.is_some() {
            Self::Middle(MiddleRoute::new(path, layout, children))
        } else {
            Self::Final(FinalRoute::new(path, component))
        }
    }
}

#[derive(Debug, Clone)]
pub struct MiddleRoute {
    pub layout: Layout,
    pub path: &'static str,
    pub children: Box<Vec<LocalizedRoute>>,
}

impl MiddleRoute {
    pub fn new(path: &'static str, layout: Option<Layout>, children: Vec<LocalizedRoute>) -> Self {
        Self {
            path,
            layout: layout.unwrap_or_default(),
            children: Box::new(children),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FinalRoute {
    pub component: Component,
    pub path: &'static str,
}

impl FinalRoute {
    pub fn new(path: &'static str, component: Option<Component>) -> Self {
        Self {
            path,
            component: component.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Layout {
    Nav,
    #[default]
    Outlet,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Component {
    Home,
    Curation,
    Settings,
    Notifications,
    #[default]
    NotFound,
}
