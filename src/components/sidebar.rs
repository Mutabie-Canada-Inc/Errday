use dioxus::prelude::*;
use crate::routes::Route;

/// SIDEBAR COMPONENT: The main navigation menu for the application
#[component]
pub fn Sidebar() -> Element {
    // Keep track of whether the sidebar is slim (collapsed) or full-width
    let mut is_collapsed = use_signal(|| false);

    // Calculate styling based on whether we are collapsed or expanded
    let nav_width = if is_collapsed() { "w-20" } else { "w-64" };
    let text_visibility = if is_collapsed() { "hidden" } else { "block" };
    let align_items = if is_collapsed() { "items-center justify-center" } else { "items-center" };
    let px = if is_collapsed() { "px-2" } else { "px-4" };

    rsx! {
        nav { class: "{nav_width} transition-all duration-300 border-r border-space-700 bg-space-900/50 flex flex-col justify-between py-6 relative",
            
            // COLLAPSE TOGGLE: A small button that pops out to let users expand/shrink the menu
            button { 
                class: "absolute -right-3 top-8 w-6 h-6 bg-space-800 border border-space-700 rounded-full flex items-center justify-center hover:bg-space-700 hover:text-neon-cyan transition-colors z-50",
                onclick: move |_| is_collapsed.set(!is_collapsed()),
                if is_collapsed() {
                     svg { class: "w-3 h-3", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", 
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M9 5l7 7-7 7" }
                    }
                } else {
                    svg { class: "w-3 h-3", fill: "none", stroke: "currentColor", view_box: "0 0 24 24", 
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M15 19l-7-7 7-7" }
                    }
                }
            }

            // APP BRANDING: Shows "Errday Mission Control" or a small logo when collapsed
            div { class: "px-6 mb-8 whitespace-nowrap overflow-hidden transition-opacity duration-200 {text_visibility}",
                h1 { class: "text-2xl font-bold font-sans tracking-tight text-white mb-1", "Errday" }
                p { class: "text-xs text-neon-cyan font-mono tracking-widest uppercase", "Mission Control" }
            }
             if is_collapsed() {
                div { class: "mb-8 flex justify-center",
                    div { class: "w-8 h-8 rounded-full bg-neon-cyan/20 border border-neon-cyan flex items-center justify-center text-neon-cyan text-xs font-bold", "E" }
                }
            }

            // PRIMARY NAVIGATION: Links to the main dashboards
            div { class: "flex-1 {px} space-y-2",
                // 1. Brain Dump (Inbox)
                Link { to: Route::Inbox {}, class: "block p-3 rounded hover:bg-space-800 text-gray-400 hover:text-white transition-colors flex {align_items} gap-3 group relative",
                    active_class: "bg-space-800 text-neon-cyan border-l-2 border-neon-cyan",
                    span { class: "w-1.5 h-1.5 rounded-full bg-current opacity-50 group-hover:opacity-100 transition-opacity" }
                    span { class: "{text_visibility} whitespace-nowrap", "Brain Dump" }
                    if is_collapsed() {
                        div { class: "absolute left-full top-1/2 -translate-y-1/2 ml-2 bg-space-800 text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none border border-space-700 z-50", "Brain Dump" }
                    }
                }
                // 2. Eisenhower Matrix
                Link { to: Route::Matrix {}, class: "block p-3 rounded hover:bg-space-800 text-gray-400 hover:text-white transition-colors flex {align_items} gap-3 group relative",
                    active_class: "bg-space-800 text-neon-purple border-l-2 border-neon-purple",
                     span { class: "w-1.5 h-1.5 rounded-full bg-current opacity-50 group-hover:opacity-100 transition-opacity" }
                     span { class: "{text_visibility} whitespace-nowrap", "Matrix Protocol" }
                     if is_collapsed() {
                        div { class: "absolute left-full top-1/2 -translate-y-1/2 ml-2 bg-space-800 text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none border border-space-700 z-50", "Matrix Protocol" }
                    }
                }
                // 3. Time Blocking (Calendar)
                Link { to: Route::Calendar {}, class: "block p-3 rounded hover:bg-space-800 text-gray-400 hover:text-white transition-colors flex {align_items} gap-3 group relative",
                    active_class: "bg-space-800 text-neon-green border-l-2 border-neon-green",
                     span { class: "w-1.5 h-1.5 rounded-full bg-current opacity-50 group-hover:opacity-100 transition-opacity" }
                     span { class: "{text_visibility} whitespace-nowrap", "Time Blocking" }
                     if is_collapsed() {
                        div { class: "absolute left-full top-1/2 -translate-y-1/2 ml-2 bg-space-800 text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none border border-space-700 z-50", "Time Blocking" }
                    }
                }
            }
            
            // SYSTEM NAVIGATION: Secondary links for help and info
            div { class: "{px} mt-auto space-y-2 border-t border-space-800 pt-4",
                Link { to: Route::Tutorial {}, class: "block p-3 rounded hover:bg-space-800 text-gray-500 hover:text-white transition-colors text-sm flex {align_items} gap-3 group relative",
                    active_class: "text-white",
                    if is_collapsed() {  span { class: "text-xs", "?" } } else { "Tutorial" }
                     if is_collapsed() {
                        div { class: "absolute left-full top-1/2 -translate-y-1/2 ml-2 bg-space-800 text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none border border-space-700 z-50", "Flight Manual" }
                    }
                }
                Link { to: Route::Credits {}, class: "block p-3 rounded hover:bg-space-800 text-gray-500 hover:text-white transition-colors text-sm flex {align_items} gap-3 group relative",
                    active_class: "text-white",
                    if is_collapsed() {  span { class: "text-xs", "i" } } else { "System Info" }
                     if is_collapsed() {
                        div { class: "absolute left-full top-1/2 -translate-y-1/2 ml-2 bg-space-800 text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none border border-space-700 z-50", "System Info" }
                    }
                }
            }
        }
    }
}
