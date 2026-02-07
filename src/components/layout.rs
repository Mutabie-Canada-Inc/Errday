use dioxus::prelude::*;
use crate::components::Sidebar;
use crate::routes::Route;

/// SHELL COMPONENT: This wraps every page with a consistent sidebar and background
#[component]
pub fn SidebarLayout() -> Element {
    rsx! {
        div { class: "flex h-screen bg-space-900 text-gray-100 font-sans overflow-hidden",
            // The persistent navigation menu on the left
            Sidebar {}
            
            // The main content area where different pages (Outlets) are loaded
            div { class: "flex-1 overflow-hidden relative",
                // Subtle space-themed grid background decoration
                div { class: "absolute inset-0 bg-[url('/assets/grid.svg')] opacity-5 pointer-events-none" }
                
                // This is where the specific page content (Inbox, Matrix, etc.) appears
                Outlet::<Route> {}
            }
        }
    }
}

/// ERROR COMPONENT: Shown when a user tries to go to a non-existent page
#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-full text-center p-8",
            h1 { class: "text-4xl font-bold text-neon-pink mb-4", "404 - LOST IN SPACE" }
            p { class: "text-gray-400 mb-8", "The coordinates {route:?} do not exist." }
            // Navigation link to bring the user back to safety
            Link { to: Route::Inbox {}, class: "btn-primary", "Return to Base" }
        }
    }
}
