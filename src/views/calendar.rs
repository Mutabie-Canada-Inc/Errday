use dioxus::prelude::*;
use crate::store::AppState;
use crate::models::{Task, Quadrant};
use chrono::{Datelike, Local, NaiveDate};

#[component]
pub fn Calendar() -> Element {
    let app_state = use_context::<AppState>();
    let current_week_start = use_signal(|| {
        let now = Local::now();
        let days_from_monday = now.weekday().num_days_from_monday();
        now.date_naive() - chrono::Duration::days(days_from_monday as i64)
    });

    let week_days: Vec<NaiveDate> = (0..7).map(|i| {
        current_week_start.read().checked_add_signed(chrono::Duration::days(i)).unwrap()
    }).collect();

    let tasks: Vec<crate::models::Task> = app_state.tasks.read().iter()
        .filter(|t| matches!(t.quadrant, Quadrant::DoFirst | Quadrant::Schedule))
        .cloned()
        .collect();

    rsx! {
        div { class: "flex-1 h-full flex flex-col p-4",
            h2 { class: "text-2xl font-bold mb-4 font-mono text-neon-green", "FLIGHT PLAN // EXECUTE" }
            
            div { class: "grid grid-cols-7 gap-2 h-full",
                // for day in week_days {
                //    div { "Day" }
                // }
                div { "Week view coming soon" }
            }

            div { class: "h-48 border-t border-space-700 mt-4 p-4 overflow-x-auto flex gap-4 bg-space-900/50",
                div { class: "w-48 text-gray-400 text-sm font-mono p-2 border-r border-space-700",
                    "UNSCHEDULED TASKS"
                    br {}
                    "(Drag to calendar)"
                }
                
                // for task in tasks {
                //    div { "Task" }
                // }
                div { "Tasks here" }
            }
        }
    }
}
