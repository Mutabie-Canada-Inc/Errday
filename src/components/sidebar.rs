use dioxus::prelude::*;
use crate::routes::Route;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div {
            class: "w-64 h-screen bg-space-800 border-r border-space-700 flex flex-col p-4",
            div {
                class: "mb-8 flex items-center gap-2",
                div { class: "w-8 h-8 bg-neon-cyan/20 rounded-full border border-neon-cyan flex items-center justify-center",
                    "🚀"
                }
                h1 { class: "text-xl font-bold tracking-wider text-gray-100", "ERRDAY" }
            }
            
            nav {
                class: "space-y-2",
                Link { to: Route::Inbox {}, class: "block px-4 py-2 rounded text-gray-400 hover:text-neon-cyan hover:bg-space-700 transition-colors", "Inbox" }
                Link { to: Route::Matrix {}, class: "block px-4 py-2 rounded text-gray-400 hover:text-neon-cyan hover:bg-space-700 transition-colors", "Matrix" }
                Link { to: Route::Calendar {}, class: "block px-4 py-2 rounded text-gray-400 hover:text-neon-cyan hover:bg-space-700 transition-colors", "Calendar" }
            }
            
            div { class: "mt-auto pt-4 border-t border-space-700 text-xs text-gray-500 text-center font-mono",
                "MISSION CONTROL"
            }
        }
    }
}
