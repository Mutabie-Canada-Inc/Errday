use dioxus::prelude::*;
use crate::routes::Route;

/// TUTORIAL VIEW: A simple step-by-step guide to help new users understand the workflow
#[component]
pub fn Tutorial() -> Element {
    rsx! {
        div { class: "flex-1 h-full p-12 overflow-y-auto",
            div { class: "max-w-4xl mx-auto space-y-12",
                // PAGE HEADER: Mission identification
                div { class: "border-b border-gray-800 pb-8 text-center",
                    h2 { class: "text-4xl font-bold mb-2 text-white font-sans tracking-tight", "Tutorial" }
                    p { class: "text-gray-400 font-mono text-sm", "OPERATIONAL GUIDE // v1.0" }
                }

                div { class: "grid gap-8",
                    // Stage 1: The 'Brain Dump' phase
                    div { class: "glass-panel p-8 rounded-xl border-l-4 border-neon-cyan flex gap-6",
                        div { class: "text-4xl font-mono text-neon-cyan/50 font-bold", "01" }
                        div {
                            h3 { class: "text-xl font-bold text-white mb-2", "Brain Dump" }
                            p { class: "text-gray-400 leading-relaxed",
                                "Start by capturing everything on your mind in the 'Brain Dump' tab. Don't worry about sorting or organizing yet. Just get it all out of your head."
                            }
                        }
                    }

                    // Stage 2: The 'Prioritization' phase
                    div { class: "glass-panel p-8 rounded-xl border-l-4 border-neon-purple flex gap-6",
                        div { class: "text-4xl font-mono text-neon-purple/50 font-bold", "02" }
                        div {
                            h3 { class: "text-xl font-bold text-white mb-2", "Eisenhower Matrix" }
                            p { class: "text-gray-400 leading-relaxed",
                                "Move to the 'Matrix' tab. Drag your tasks into one of the four quadrants based on Urgency and Importance. Decide what to Do First, Schedule, Delegate, or Delete."
                            }
                        }
                    }

                    // Stage 3: The 'Scheduling' phase
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

                // Eisenhower Matrix Definitions Grid
                div { class: "mt-16 mb-8",
                    h3 { class: "text-2xl font-bold text-white mb-8 text-center font-sans tracking-tight", "Eisenhower Matrix Reference" }
                    div { class: "grid grid-cols-2 gap-8",
                        // Do First
                        div { class: "glass-panel p-8 rounded-xl border-t-2 border-neon-pink bg-space-900",
                            h4 { class: "text-lg font-bold text-neon-pink mb-3 tracking-widest", "DO FIRST" }
                            p { class: "text-xs font-mono opacity-70 uppercase tracking-wider mb-4", "URGENT & IMPORTANT" }
                            p { class: "text-gray-400 text-sm leading-relaxed", "Crises, deadlines, and problems. Do these NOW." }
                        }
                        // Schedule
                        div { class: "glass-panel p-8 rounded-xl border-t-2 border-neon-cyan bg-space-900",
                            h4 { class: "text-lg font-bold text-neon-cyan mb-3 tracking-widest", "SCHEDULE" }
                            p { class: "text-xs font-mono opacity-70 uppercase tracking-wider mb-4", "IMPORTANT, NOT URGENT" }
                            p { class: "text-gray-400 text-sm leading-relaxed", "Planning, prevention, and improvement. Schedule a time for these." }
                        }
                        // Delegate
                        div { class: "glass-panel p-8 rounded-xl border-t-2 border-neon-amber bg-space-900",
                            h4 { class: "text-lg font-bold text-neon-amber mb-3 tracking-widest", "DELEGATE" }
                            p { class: "text-xs font-mono opacity-70 uppercase tracking-wider mb-4", "URGENT, NOT IMPORTANT" }
                            p { class: "text-gray-400 text-sm leading-relaxed", "Interruptions, some calls/meetings. Delegate if possible." }
                        }
                        // Delete
                        div { class: "glass-panel p-8 rounded-xl border-t-2 border-space-700 bg-space-900",
                            h4 { class: "text-lg font-bold text-gray-400 mb-3 tracking-widest", "DELETE" }
                            p { class: "text-xs font-mono opacity-70 uppercase tracking-wider mb-4", "NEITHER" }
                            p { class: "text-gray-400 text-sm leading-relaxed", "Time wasters and busy work. Elimination is the goal." }
                        }
                    }
                }

                // CTA: Call to action to jump back to the main dash
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
