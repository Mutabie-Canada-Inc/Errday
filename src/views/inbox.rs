use dioxus::prelude::*;
use crate::store::AppState;

#[component]
pub fn Inbox() -> Element {
    let app_state = use_context::<AppState>();
    let mut input_val = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex-1 h-full p-8 overflow-y-auto",
            h2 { class: "text-3xl font-bold mb-6 text-neon-cyan font-mono", "INBOX // CAPTURE" }
            
            div { class: "max-w-2xl mx-auto",
                div { class: "flex gap-4 mb-8",
                    input {
                        class: "flex-1 bg-space-800 border border-space-700 rounded p-4 text-white focus:border-neon-cyan focus:outline-none focus:ring-1 focus:ring-neon-cyan transition-all",
                        placeholder: "What needs to be done?",
                        value: "{input_val}",
                        oninput: move |evt| input_val.set(evt.value()),
                        onkeydown: move |evt| {
                            if evt.key() == Key::Enter && !input_val.read().trim().is_empty() {
                                app_state.add_task(input_val.read().clone());
                                input_val.set("".to_string());
                            }
                        }
                    }
                    button {
                        class: "bg-neon-cyan/20 text-neon-cyan border border-neon-cyan/50 px-6 rounded hover:bg-neon-cyan/30 transition-colors uppercase font-bold tracking-wide",
                        onclick: move |_| {
                            if !input_val.read().trim().is_empty() {
                                app_state.add_task(input_val.read().clone());
                                input_val.set("".to_string());
                            }
                        },
                        "Add"
                    }
                }

                div { class: "space-y-3",
                    for task in app_state.tasks.read().iter().filter(|t| matches!(t.quadrant, crate::models::Quadrant::Unsorted)).cloned() {
                        div { class: "glass-panel p-4 rounded flex justify-between items-center group",
                            span { class: "text-lg", "{task.title}" }
                            div { class: "opacity-0 group-hover:opacity-100 transition-opacity flex gap-2",
                                button { class: "text-xs px-2 py-1 border border-space-700 rounded hover:border-red-500 hover:text-red-500",
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
