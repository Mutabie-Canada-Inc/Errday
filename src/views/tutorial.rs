use dioxus::prelude::*;
use crate::routes::Route;

#[component]
pub fn Tutorial() -> Element {
    rsx! {
        div { class: "flex-1 h-full p-12 overflow-y-auto",
            div { class: "max-w-4xl mx-auto space-y-12",
                // Header
                div { class: "border-b border-gray-800 pb-8 text-center",
                    h2 { class: "text-4xl font-bold mb-2 text-white font-sans tracking-tight", "Flight Manual" }
                    p { class: "text-gray-400 font-mono text-sm", "OPERATIONAL GUIDE // v1.0" }
                }

                div { class: "grid gap-8",
                    // Step 1: Capture
                    div { class: "glass-panel p-8 rounded-xl border-l-4 border-neon-cyan flex gap-6",
                        div { class: "text-4xl font-mono text-neon-cyan/50 font-bold", "01" }
                        div {
                            h3 { class: "text-xl font-bold text-white mb-2", "Brain Dump" }
                            p { class: "text-gray-400 leading-relaxed",
                                "Start by capturing everything on your mind in the 'Brain Dump' tab. Don't worry about sorting or organizing yet. Just get it all out of your head."
                            }
                        }
                    }

                    // Step 2: Sort
                    div { class: "glass-panel p-8 rounded-xl border-l-4 border-neon-purple flex gap-6",
                        div { class: "text-4xl font-mono text-neon-purple/50 font-bold", "02" }
                        div {
                            h3 { class: "text-xl font-bold text-white mb-2", "Eisenhower Matrix" }
                            p { class: "text-gray-400 leading-relaxed",
                                "Move to the 'Matrix' tab. Drag your tasks into one of the four quadrants based on Urgency and Importance. Decide what to Do First, Schedule, Delegate, or Delete."
                            }
                        }
                    }

                    // Step 3: Schedule
                    div { class: "glass-panel p-8 rounded-xl border-l-4 border-neon-green flex gap-6",
                        div { class: "text-4xl font-mono text-neon-green/50 font-bold", "03" }
                        div {
                            h3 { class: "text-xl font-bold text-white mb-2", "Time Blocking" }
                            p { class: "text-gray-400 leading-relaxed",
                                "Finally, go to the 'Calendar' tab. Drag your prioritized tasks onto specific days to create a realistic flight plan for your week."
                            }
                        }
                    }
                }

                div { class: "text-center pt-8",
                    Link { to: Route::Inbox {}, class: "btn-primary inline-flex items-center gap-2",
                        "Start Mission"
                        svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", view_box: "0 0 24 24",
                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M14 5l7 7m0 0l-7 7m7-7H3" }
                        }
                    }
                }
            }
        }
    }
}
