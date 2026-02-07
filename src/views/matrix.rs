use dioxus::prelude::*;
use crate::models::{Quadrant, Task};
use crate::store::AppState;

#[component]
pub fn Matrix() -> Element {
    let _app_state = use_context::<AppState>();
    let dragged_id = use_signal(|| None::<uuid::Uuid>);

    rsx! {
        div { class: "flex-1 h-full p-4 grid grid-cols-2 grid-rows-2 gap-4",
            QuadrantBox { title: "DO FIRST // URGENT & IMPORTANT", quadrant: Quadrant::DoFirst, color: "border-neon-pink text-neon-pink", dragged_id: dragged_id }
            QuadrantBox { title: "SCHEDULE // IMPORTANT", quadrant: Quadrant::Schedule, color: "border-neon-cyan text-neon-cyan", dragged_id: dragged_id }
            QuadrantBox { title: "DELEGATE // URGENT", quadrant: Quadrant::Delegate, color: "border-neon-amber text-neon-amber", dragged_id: dragged_id }
            QuadrantBox { title: "DELETE // NEITHER", quadrant: Quadrant::Delete, color: "border-space-700 text-gray-400", dragged_id: dragged_id }
        }
    }
}

#[component]
fn QuadrantBox(title: &'static str, quadrant: Quadrant, color: &'static str, dragged_id: Signal<Option<uuid::Uuid>>) -> Element {
    let app_state = use_context::<AppState>();
    
    let is_drop_target = use_signal(|| false);

    let drop_target_class = if is_drop_target() { "bg-space-700/50" } else { "" };
    
    let tasks: Vec<crate::models::Task> = app_state.tasks.read().iter()
        .filter(|t| t.quadrant == quadrant)
        .cloned()
        .collect();

    rsx! {
        div {
            class: "glass-panel rounded-lg p-4 flex flex-col h-full border-2 transition-colors {color} {drop_target_class}",
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
            h3 { class: "text-xl font-bold mb-4 tracking-widest border-b border-white/10 pb-2", "{title}" }
            
            div { class: "flex-1 overflow-y-auto space-y-2",
                for task in tasks {
                    div {
                        key: "{task.id}",
                        class: "bg-space-900/80 p-3 rounded cursor-move hover:bg-space-800 transition-colors border border-space-700",
                        draggable: true,
                        ondragstart: move |_| {
                            let mut dragged_id = dragged_id;
                            dragged_id.set(Some(task.id));
                        },
                        "{task.title}"
                    }
                }
            }
        }
    }
}
