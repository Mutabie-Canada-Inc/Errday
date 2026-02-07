use dioxus::prelude::*;
use crate::routes::Route;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        nav { class: "w-64 border-r border-space-700 bg-space-900/50 flex flex-col justify-between py-6",
            div { class: "px-6 mb-8",
                h1 { class: "text-2xl font-bold font-sans tracking-tight text-white mb-1", "Errday" }
                p { class: "text-xs text-neon-cyan font-mono tracking-widest uppercase", "Mission Control" }
            }

            div { class: "flex-1 px-4 space-y-2",
                Link { to: Route::Inbox {}, class: "block p-3 rounded hover:bg-space-800 text-gray-400 hover:text-white transition-colors flex items-center gap-3 group",
                    active_class: "bg-space-800 text-neon-cyan border-l-2 border-neon-cyan",
                    span { class: "w-1.5 h-1.5 rounded-full bg-current opacity-50 group-hover:opacity-100 transition-opacity" }
                    "Brain Dump"
                }
                Link { to: Route::Matrix {}, class: "block p-3 rounded hover:bg-space-800 text-gray-400 hover:text-white transition-colors flex items-center gap-3 group",
                    active_class: "bg-space-800 text-neon-purple border-l-2 border-neon-purple",
                     span { class: "w-1.5 h-1.5 rounded-full bg-current opacity-50 group-hover:opacity-100 transition-opacity" }
                    "Matrix Protocol"
                }
                Link { to: Route::Calendar {}, class: "block p-3 rounded hover:bg-space-800 text-gray-400 hover:text-white transition-colors flex items-center gap-3 group",
                    active_class: "bg-space-800 text-neon-green border-l-2 border-neon-green",
                     span { class: "w-1.5 h-1.5 rounded-full bg-current opacity-50 group-hover:opacity-100 transition-opacity" }
                    "Flight Plan"
                }
            }
            
            div { class: "px-4 mt-auto space-y-2 border-t border-space-800 pt-4",
                Link { to: Route::Tutorial {}, class: "block p-3 rounded hover:bg-space-800 text-gray-500 hover:text-white transition-colors text-sm flex items-center gap-3",
                    active_class: "text-white",
                    "Flight Manual"
                }
                Link { to: Route::Credits {}, class: "block p-3 rounded hover:bg-space-800 text-gray-500 hover:text-white transition-colors text-sm flex items-center gap-3",
                    active_class: "text-white",
                    "System Info"
                }
            }
        }
    }
}
