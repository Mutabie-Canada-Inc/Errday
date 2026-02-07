use dioxus::prelude::*;
use crate::components::Sidebar;
use crate::routes::Route;

#[component]
pub fn SidebarLayout() -> Element {
    rsx! {
        div { class: "flex h-screen bg-space-900 text-gray-100 font-sans overflow-hidden",
            Sidebar {}
            div { class: "flex-1 overflow-hidden relative",
                // Background stars/grid effect could go here
                div { class: "absolute inset-0 bg-[url('/assets/grid.svg')] opacity-5 pointer-events-none" }
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-full text-center p-8",
            h1 { class: "text-4xl font-bold text-neon-pink mb-4", "404 - LOST IN SPACE" }
            p { class: "text-gray-400 mb-8", "The coordinates {route:?} do not exist." }
            Link { to: Route::Inbox {}, class: "btn-primary", "Return to Base" }
        }
    }
}
