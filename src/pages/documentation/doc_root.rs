use std::fmt::Display;

use leptonic::prelude::*;
use leptos::*;
use leptos_icons::BsIcon;
use leptos_router::*;

use crate::app::{AppLayoutContext, AppRoutes};

use crate::pages::documentation::installation::PageInstallation;

use crate::pages::documentation::overview::PageOverview;

use crate::pages::documentation::deployments::Deployments;

use crate::pages::documentation::trade_history::TradeHistory;
// use crate::APP_BAR_HEIGHT;
pub const APP_BAR_HEIGHT: Height = Height::Em(3.5);
#[derive(Debug, Copy, Clone)]
pub enum DocRoutes {
    // Getting started
    Overview,
    Installation,
    Usage,
    Themes,
    Changelog,

    // Layout
    Stack,
    Grid,
    Separator,
    Skeleton,
    AppBar,
    Drawer,
    Tab,
    Table,
    Collapsible,

    // Input
    Button,
    Input,
    TiptapEditor,
    DateTime,
    Slider,
    Select,
    Toggle,
    ColorPicker,

    // Feedback
    Alert,
    Toast,
    Modal,
    Progress,
    Popover,
    Chip,
    Kbd,

    // General
    Typography,
    Icon,
    Link,
    Anchor,
    Callback,

    // Animation
    Transition,

    // Technical
    NotFound,
}

impl DocRoutes {
    pub const fn route(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Installation => "installation",
            Self::Usage => "usage",
            Self::Themes => "themes",
            Self::Changelog => "changelog",

            Self::Stack => "stack",
            Self::Grid => "grid",
            Self::Separator => "separator",
            Self::Skeleton => "skeleton",
            Self::AppBar => "app-bar",
            Self::Drawer => "drawer",
            Self::Tab => "tabs",
            Self::Table => "table",
            Self::Collapsible => "collapsible",

            Self::Button => "button",
            Self::Input => "input",
            Self::TiptapEditor => "tiptap-editor",
            Self::DateTime => "date-time",
            Self::Slider => "slider",
            Self::Select => "select",
            Self::Toggle => "toggle",
            Self::ColorPicker => "color-picker",

            Self::Alert => "alert",
            Self::Toast => "toast",
            Self::Modal => "modal",
            Self::Progress => "progress",
            Self::Popover => "popover",
            Self::Chip => "chip",
            Self::Kbd => "kbd",

            Self::Typography => "typography",
            Self::Icon => "icon",
            Self::Link => "link",
            Self::Anchor => "anchor",
            Self::Callback => "callback",

            Self::Transition => "transition",
            Self::NotFound => "*", // Leptos requires this to be be named "*"!
        }
    }
}

/// Required so that `Routes` variants can be used in `<Route path=Routes::Foo ...>` definitions.
impl Display for DocRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.route())
    }
}

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for DocRoutes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || format!("/{}/{}", AppRoutes::Doc.route(), self.route()))
    }
}

// You can define other routes in their own component.
// Use a #[component(transparent)] that returns a <Route/>.
#[component(transparent)]
pub fn DocRoutes<P: Display>(path: P) -> impl IntoView {
    view! {
        <Route path=path view=|| view! { <DocLayout/>}>
            <Route path="" view=|| view! { <Redirect path=DocRoutes::Overview/> }/>
            <Route path=DocRoutes::Overview view=|| view! { <PageOverview/> }/>
            <Route path=DocRoutes::Installation view=|| view! { <PageInstallation/> }/>
            <Route path=DocRoutes::Usage view=|| view! { <TradeHistory/> }/>
            <Route path=DocRoutes::Themes view=|| view! { <Deployments/> }/>
        </Route>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn DocLayout() -> impl IntoView {
    let app_layout_context = expect_context::<AppLayoutContext>();

    let drawer_class = move || match app_layout_context.is_small.get() {
        true => "mobile",
        false => "",
    };

    let close_doc_drawer_on_mobile = move || {
        if app_layout_context.is_small.get_untracked() {
            app_layout_context.close_doc_drawer();
        }
    };

    let drawer_content = view! {
        <DrawerSection header=move || view! {
            <Icon icon=BsIcon::BsBook margin=Margin::Right(Size::Em(1.0))></Icon> "Uni Vizualizer"
        }>
            <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="link-stack">
                <Link href=DocRoutes::Overview class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Overview"</Link>
                <Link href=DocRoutes::Themes class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Deployments"</Link>
                <Link href=DocRoutes::Installation class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Uniswap Pools"</Link>
                <Link href=DocRoutes::Usage class="item" on:click=move |_| close_doc_drawer_on_mobile()>"Trade History"</Link>
            </Stack>
        </DrawerSection>
    };

    view! {
        <Box id="doc-layout">
            <Drawer
                side=DrawerSide::Left
                id="doc-drawer"
                shown=Signal::derive(move || !app_layout_context.doc_drawer_closed.get())
                class=drawer_class
                style=format!("top: {APP_BAR_HEIGHT}")
            >
                <Stack orientation=StackOrientation::Vertical spacing=Size::Zero class="menu">
                    { drawer_content }
                </Stack>
            </Drawer>

            <Box id="doc-content">
                // <Outlet/> will show nested child routes.
                <Outlet/>
            </Box>
        </Box>
    }
}

#[component]
pub fn DrawerSection<H, IV>(header: H, children: Children) -> impl IntoView
where
    H: Fn() -> IV + 'static,
    IV: IntoView + 'static,
{
    view! {
        <div class="drawer-section">
            <div class="section-header">
                { header() }
            </div>
            { children() }
        </div>
    }
}

#[component]
pub fn New() -> impl IntoView {
    view! {
        <Chip style="color: var(--primary-color); background-color: transparent; margin: 0; padding: 0;">
            "NEW"
        </Chip>
    }
}
