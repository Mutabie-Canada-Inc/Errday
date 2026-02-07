use dioxus::prelude::*;
use crate::models::Quadrant;
use crate::store::AppState;

#[component]
pub fn Matrix() -> Element {
    let _app_state = use_context::<AppState>();
    let dragged_id = use_signal(|| None::<uuid::Uuid>);

    rsx! {
        div { class: "flex h-full",
            // Backlog Sidebar
            div { class: "w-80 bg-space-900 border-r border-space-700 p-4 flex flex-col h-full",
                div { class: "mb-4 pb-4 border-b border-space-800 flex justify-between items-center group",
                    div {
                        h2 { class: "text-lg font-bold text-white tracking-tight", "Backlog" }
                        p { class: "text-xs text-gray-500 font-mono", "UNSORTED MINDS" }
                    }
                    div { class: "relative group-hover:block hidden",
                         InfoBubble { 
                             text: "Tasks captured in Brain Dump appear here. Drag them into a quadrant.",
                             pos_class: "left-0 top-full mt-2"
                         }
                    }
                }
                
                div { class: "flex-1 overflow-y-auto space-y-2",
                   QuadrantBox { 
                       title: "", 
                       quadrant: Quadrant::Unsorted, 
                       color: "border-transparent", 
                       dragged_id: dragged_id,
                       is_backlog: true
                   }
                }
            }

            // Matrix Grid
            div { class: "flex-1 p-16 grid grid-cols-2 grid-rows-2 gap-10",
                QuadrantBox { 
                    title: "DO FIRST", 
                    subtitle: "URGENT & IMPORTANT", 
                    quadrant: Quadrant::DoFirst, 
                    color: "border-neon-pink text-neon-pink", 
                    dragged_id: dragged_id,
                    info: "Crises, deadlines, and problems. Do these NOW.",
                    bubble_pos: "left-0 top-full mt-3" 
                }
                QuadrantBox { 
                    title: "SCHEDULE", 
                    subtitle: "IMPORTANT, NOT URGENT", 
                    quadrant: Quadrant::Schedule, 
                    color: "border-neon-cyan text-neon-cyan", 
                    dragged_id: dragged_id,
                    info: "Planning, prevention, and improvement. Schedule a time for these.",
                    bubble_pos: "left-0 top-full mt-3" 
                }
                QuadrantBox { 
                    title: "DELEGATE", 
                    subtitle: "URGENT, NOT IMPORTANT",
                    quadrant: Quadrant::Delegate, 
                    color: "border-neon-amber text-neon-amber", 
                    dragged_id: dragged_id,
                    info: "Interruptions, some calls/meetings. Delegate if possible.",
                    bubble_pos: "left-0 bottom-full mb-3" 
                }
                QuadrantBox { 
                    title: "DELETE", 
                    subtitle: "NEITHER",
                    quadrant: Quadrant::Delete, 
                    color: "border-space-700 text-gray-400", 
                    dragged_id: dragged_id, 
                    info: "Time wasters and busy work. Elimination is the goal.",
                    bubble_pos: "left-0 bottom-full mb-3" 
                }
            }
        }
    }
}

#[component]
fn InfoBubble(text: &'static str, pos_class: Option<&'static str>) -> Element {
    // Default to displaying below and aligned to the left edge of the icon
    let placement = pos_class.unwrap_or("left-0 top-full mt-3");
    
    rsx! {
        div { class: "relative group cursor-help ml-2 z-[60]",
            div { class: "w-8 h-8 shrink-0 rounded-full border border-gray-500 text-gray-400 flex items-center justify-center text-lg font-mono font-bold hover:border-white hover:text-white transition-colors bg-space-900 shadow-lg",
                "i"
            }
            div { class: "absolute {placement} w-64 bg-space-800 text-gray-200 text-xs p-4 rounded-xl shadow-2xl border border-space-600 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none text-left backdrop-blur-xl z-[9999] tracking-wide leading-relaxed",
                "{text}"
            }
        }
    }
}

#[component]
fn QuadrantBox(
    title: &'static str, 
    subtitle: Option<&'static str>,
    quadrant: Quadrant, 
    color: &'static str, 
    dragged_id: Signal<Option<uuid::Uuid>>,
    info: Option<&'static str>,
    is_backlog: Option<bool>,
    bubble_pos: Option<&'static str>
) -> Element {
    let app_state = use_context::<AppState>();
    
    let is_drop_target = use_signal(|| false);

    let drop_target_class = if is_drop_target() { "bg-space-700/50" } else { "" };
    
    let tasks: Vec<crate::models::Task> = app_state.tasks.read().iter()
        .filter(|t| t.quadrant == quadrant)
        .cloned()
        .collect();

    // Conditional styling for backlog vs matrix boxes
    let container_class = if is_backlog.unwrap_or(false) {
        format!("flex flex-col h-full transition-colors {}", drop_target_class)
    } else {
        // Added hover:z-50 to bring the whole box to front when interacting (helps bubbles overlap neighbors)
        format!("glass-panel rounded-xl p-10 flex flex-col h-full border-2 transition-all duration-300 hover:bg-space-800/80 hover:shadow-2xl hover:z-50 relative overflow-visible {} {}", color, drop_target_class)
    };

    rsx! {
        div {
            class: "{container_class}",
            ondragover: move |evt| {
                evt.prevent_default();
                let mut is_drop_target = is_drop_target;
                is_drop_target.set(true);
            },
            ondragleave: move |_| {
                let mut is_drop_target = is_drop_target;
                is_drop_target.set(false);
            },
            ondrop: move |_| {
                let mut is_drop_target = is_drop_target;
                is_drop_target.set(false);
                let id = *dragged_id.read();
                if let Some(id) = id {
                    app_state.update_task_quadrant(id, quadrant.clone());
                    let mut dragged_id = dragged_id;
                    dragged_id.set(None);
                }
            },
            
            if !is_backlog.unwrap_or(false) {
                div { class: "flex justify-between items-start mb-6 border-b border-white/5 pb-4 relative z-20",
                    div {
                        h3 { class: "text-2xl font-bold tracking-widest leading-none mb-1", "{title}" }
                        if let Some(sub) = subtitle {
                            p { class: "text-xs font-mono opacity-70 uppercase tracking-wider", "{sub}" }
                        }
                    }
                    if let Some(info_text) = info {
                        InfoBubble { text: info_text, pos_class: bubble_pos }
                    }
                }
            }
            
            div { class: "flex-1 overflow-y-auto space-y-3 min-h-0 z-10",
                if tasks.is_empty() {
                    div { class: "h-full flex items-center justify-center text-gray-700 text-sm font-mono italic",
                        "Empty Sector"
                    }
                }
                for task in tasks {
                    div {
                        key: "{task.id.to_string()}",
                        class: "bg-space-900 border border-space-700 p-4 rounded-lg group cursor-move hover:border-current transition-all shadow-sm relative hover:translate-x-1",
                        draggable: true,
                        ondragstart: move |_| {
                            let mut dragged_id = dragged_id;
                            dragged_id.set(Some(task.id));
                        },
                        
                        div { class: "flex justify-between items-start gap-3",
                            span { class: "text-sm font-medium text-gray-200 leading-snug", "{task.title}" }
                            div { class: "opacity-0 group-hover:opacity-100 transition-opacity",
                                button { 
                                    class: "text-gray-500 hover:text-red-500 w-5 h-5 flex items-center justify-center rounded hover:bg-space-800 transition-colors",
                                    onclick: move |_| {
                                        app_state.delete_task(task.id);
                                    },
                                    "×" 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
