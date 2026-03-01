use dioxus::prelude::*;
use crate::models::Quadrant;
use crate::store::AppState;

/// THE MATRIX DASHBOARD: Shows the four-quadrant Eisenhower Matrix for task prioritization
#[component]
pub fn Matrix() -> Element {
    // Stage 1: Connect to global app data and prepare a signal to track which task is being dragged
    let _app_state = use_context::<AppState>();
    let dragged_id = use_signal(|| None::<uuid::Uuid>);

    rsx! {
        div { class: "flex h-full",
            // THE BACKLOG SIDEBAR: Contains all "Unsorted" tasks waiting to be placed in the matrix
            div { class: "w-80 bg-space-900 border-r border-space-700 p-8 flex flex-col h-full",
                div { class: "mb-6 pb-6 border-b border-space-800 flex justify-between items-center group",
                    div {
                        h2 { class: "text-lg font-bold text-white tracking-tight", "Backlog" }
                        p { class: "text-xs font-mono text-neon-cyan/80 tracking-widest", "UNSORTED MINDS" }
                    }
                }
                
                div { class: "flex-1 overflow-y-auto space-y-2",
                   // Backlog is just a specialized 'QuadrantBox' with no title
                   QuadrantBox { 
                       title: "", 
                       quadrant: Quadrant::Unsorted, 
                       color: "border-transparent", 
                       dragged_id: dragged_id,
                       is_backlog: true
                   }
                }
            }

            // THE MATRIX GRID: A 2x2 grid representing the core prioritization logic
            div { class: "flex-1 p-12 grid grid-cols-2 grid-rows-2 gap-10",
                // Top-Left: Do First (Urgent & Important)
                QuadrantBox { 
                    title: "DO FIRST", 
                    subtitle: "URGENT & IMPORTANT", 
                    quadrant: Quadrant::DoFirst, 
                    color: "border-neon-pink text-neon-pink", 
                    dragged_id: dragged_id,
                }
                // Top-Right: Schedule (Important, Not Urgent)
                QuadrantBox { 
                    title: "SCHEDULE", 
                    subtitle: "IMPORTANT, NOT URGENT", 
                    quadrant: Quadrant::Schedule, 
                    color: "border-neon-cyan text-neon-cyan", 
                    dragged_id: dragged_id,
                }
                // Bottom-Left: Delegate (Urgent, Not Important)
                QuadrantBox { 
                    title: "DELEGATE", 
                    subtitle: "URGENT, NOT IMPORTANT",
                    quadrant: Quadrant::Delegate, 
                    color: "border-neon-amber text-neon-amber", 
                    dragged_id: dragged_id,
                }
                // Bottom-Right: Delete (Neither)
                QuadrantBox { 
                    title: "DELETE", 
                    subtitle: "NEITHER",
                    quadrant: Quadrant::Delete, 
                    color: "border-space-700 text-gray-400", 
                    dragged_id: dragged_id, 
                }
            }
        }
    }
}

/// QUADRANT BOX: A reusable component for each of the four areas + the backlog
#[component]
fn QuadrantBox(
    title: &'static str, 
    subtitle: Option<&'static str>,
    quadrant: Quadrant, 
    color: &'static str, 
    dragged_id: Signal<Option<uuid::Uuid>>,
    is_backlog: Option<bool>,
) -> Element {
    let app_state = use_context::<AppState>();
    
    // Track if a task is currently being dragged over this specific box
    let is_drop_target = use_signal(|| false);

    let drop_target_class = if is_drop_target() { "bg-space-700/50" } else { "" };
    
    // Get all tasks matching this quadrant
    let tasks: Vec<crate::models::Task> = app_state.tasks.read().iter()
        .filter(|t| t.quadrant == quadrant)
        .cloned()
        .collect();

    let container_class = if is_backlog.unwrap_or(false) {
        format!("flex flex-col h-full transition-colors {}", drop_target_class)
    } else {
        // Added hover:z-50 to bring the whole box to front when interacting (helps bubbles overlap neighbors)
        format!("glass-panel rounded-xl px-6 py-6 flex flex-col h-full border-2 transition-all duration-300 hover:bg-space-800/80 hover:shadow-2xl hover:z-50 relative overflow-visible {} {}", color, drop_target_class)
    };

    rsx! {
        div {
            class: "{container_class}",
            // Handle dragging over: and dropping into this box
            ondragover: move |evt| {
                evt.prevent_default(); // Required to allow a drop
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
                // Step 1: Detect which task was dropped
                if let Some(id) = id {
                    // Step 2: Update the task's location in global state
                    app_state.update_task_quadrant(id, quadrant.clone());
                    // Step 3: Reset the drag tracking signal
                    let mut dragged_id = dragged_id;
                    dragged_id.set(None);
                }
            },
            
            // Render the header (Only for the four matrix quadrants)
            if !is_backlog.unwrap_or(false) {
                div { class: "flex justify-between items-start mb-6 border-b border-white/5 pb-4 relative z-20",
                    div {
                        h3 { class: "text-2xl font-bold tracking-widest leading-none mb-1", "{title}" }
                        if let Some(sub) = subtitle {
                            p { class: "text-xs font-mono opacity-70 uppercase tracking-wider", "{sub}" }
                        }
                    }
                }
            }
            
            // List the tasks inside this section
            div { class: "flex-1 overflow-y-auto space-y-4 min-h-0 z-10 pr-2 scrollbar-thin scrollbar-thumb-space-600 scrollbar-track-transparent hover:scrollbar-thumb-space-500",
                if tasks.is_empty() {
                    div { class: "h-full flex items-center justify-center text-gray-700 text-sm font-mono italic",
                        "Empty Sector"
                    }
                }
                for task in tasks {
                    div {
                        key: "{task.id.to_string()}",
                        class: "bg-space-900 border border-space-700 p-5 rounded-lg group cursor-move hover:border-current transition-all shadow-sm relative hover:translate-x-1",
                        draggable: true,
                        // Update the global 'dragging' signal when the user picks up this task
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
