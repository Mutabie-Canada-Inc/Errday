use dioxus::prelude::*;
use crate::store::AppState;

/// INBOX VIEW: The "Brain Dump" where users capture new tasks before sorting them
#[component]
pub fn Inbox() -> Element {
    // Access the global state (The Brain) and set up local state for the input field
    let app_state = use_context::<AppState>();
    let mut input_val = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex-1 h-full p-8 overflow-y-auto",
            div { class: "w-full max-w-6xl mx-auto space-y-8",
                // PAGE HEADER: Displays the mission title
                div { class: "border-b border-gray-800 pb-6",
                    h1 { class: "text-4xl font-bold mb-2 text-white font-sans tracking-tight", "MISSION CONTROL" }
                    h2 { class: "text-xl font-mono text-neon-cyan/80 tracking-widest", "BRAIN DUMP // CAPTURE" }
                }
                
                // DATA CAPTURE AREA: Where users type in new tasks
                div { class: "flex gap-4 items-end",
                    div { class: "flex-1 relative group",
                        // A subtle glowing effect when hovering over the input
                        div { class: "absolute -inset-0.5 bg-gradient-to-r from-neon-cyan/50 to-purple-600/50 rounded-lg blur opacity-0 group-hover:opacity-100 transition duration-500" }
                        input {
                            class: "relative w-full bg-space-900 border border-space-700 rounded-lg p-3 text-lg text-white placeholder-gray-600 focus:border-neon-cyan focus:outline-none focus:ring-1 focus:ring-neon-cyan/50 transition-all font-sans",
                            placeholder: "What's on your mind?",
                            value: "{input_val}",
                            // Update the local state as the user types
                            oninput: move |evt| input_val.set(evt.value()),
                            // Handle the 'Enter' key to quickly save tasks
                            onkeydown: move |evt| {
                                if evt.key() == Key::Enter && !input_val.read().trim().is_empty() {
                                    app_state.add_task(input_val.read().clone());
                                    input_val.set("".to_string());
                                }
                            }
                        }
                    }
                    button {
                        class: "btn-primary h-[54px] flex items-center justify-center whitespace-nowrap",
                        onclick: move |_| {
                            if !input_val.read().trim().is_empty() {
                                app_state.add_task(input_val.read().clone());
                                input_val.set("".to_string());
                            }
                        },
                        "Capture Task"
                    }
                }

                // THE INBOX LIST: Shows all tasks that haven't been sorted into the matrix yet
                if !app_state.tasks.read().iter().any(|t| matches!(t.quadrant, crate::models::Quadrant::Unsorted)) {
                    // Displayed when the system is clear
                    div { class: "text-center py-20 text-gray-600 font-mono border border-dashed border-gray-800 rounded-xl bg-space-800/20",
                        "// SYSTEM CLEAR - NO PENDING TASKS"
                    }
                } else {
                    div { class: "glass-panel rounded-xl overflow-hidden",
                        table { class: "data-table",
                            thead {
                                tr {
                                    th { "Task Description" }
                                    th { class: "w-32 text-right", "Status" }
                                    th { class: "w-32 text-right", "Actions" }
                                }
                            }
                            tbody {
                                // Filter for unsorted tasks and render a row for each
                                for task in app_state.tasks.read().iter().filter(|t| matches!(t.quadrant, crate::models::Quadrant::Unsorted)).cloned() {
                                    tr {
                                        key: "{task.id}",
                                        td { 
                                            class: "text-lg font-medium",
                                            span { class: "text-neon-cyan mr-3 font-mono", "::" }
                                            "{task.title}" 
                                        }
                                        td { class: "text-right font-mono text-xs text-gray-500", "UNSORTED" }
                                        td { class: "text-right",
                                            button { class: "text-xs hover:text-red-500 text-gray-600 transition-colors uppercase tracking-wider font-bold",
                                                onclick: move |_| {
                                                    app_state.delete_task(task.id);
                                                },
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
