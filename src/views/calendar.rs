use dioxus::prelude::*;
use crate::store::AppState;
use crate::models::{Task, Quadrant};
use chrono::{Datelike, Timelike, Local, NaiveDate};

/// THE CALENDAR VIEW: A weekly "Flight Plan" for time-blocking tasks
#[component]
pub fn Calendar() -> Element {
    let app_state = use_context::<AppState>();
    
    // NAVIGATION STATE: Tracks which week we are currently viewing
    let mut current_week_start = use_signal(|| {
        let now = Local::now();
        // Step 1: Find the most recent Monday to start our week view
        let days_from_monday = now.weekday().num_days_from_monday();
        now.date_naive() - chrono::Duration::days(days_from_monday as i64)
    });

    // INTERACTION STATE: Tracks which task is being moved or resized
    let mut dragged_task_id = use_signal(|| None::<uuid::Uuid>);
    let mut stretching_task_id = use_signal(|| None::<uuid::Uuid>);

    // COMPUTATION: Create a list of the 7 dates for the current week
    let week_days: Vec<NaiveDate> = (0..7).map(|i| {
        current_week_start.read().checked_add_signed(chrono::Duration::days(i)).unwrap()
    }).collect();

    // FILTERING: Get tasks eligible for the calendar (Must be Important: Do First or Schedule)
    let (scheduled_tasks, unscheduled_tasks): (Vec<Task>, Vec<Task>) = app_state.tasks.read().iter()
        .filter(|t| matches!(t.quadrant, Quadrant::DoFirst | Quadrant::Schedule))
        .cloned()
        .partition(|t| t.scheduled_start.is_some());

    rsx! {
        div { 
            class: "flex h-full bg-space-900 text-gray-100 overflow-hidden",
            // Stop stretching when the user releases the mouse anywhere
            onmouseup: move |_| {
                stretching_task_id.set(None);
            },
            
            // THE FLIGHT DECK: Sidebar showing tasks waiting to be scheduled
            div { class: "w-72 bg-space-800/50 border-r border-space-700 p-4 flex flex-col",
                div { class: "mb-4 pb-4 border-b border-space-700",
                    h2 { class: "text-lg font-bold text-white tracking-tight", "Flight Deck" }
                    p { class: "text-xs text-neon-cyan/70 font-mono mt-1", "DRAG TO SCHEDULE" }
                }
                
                div { class: "flex-1 overflow-y-auto space-y-3",
                    if unscheduled_tasks.is_empty() {
                         div { class: "text-center py-10 text-gray-600 font-mono text-xs italic",
                            "All systems go."
                        }
                    }
                    for task in unscheduled_tasks {
                        div {
                            key: "{task.id.to_string()}",
                            draggable: true,
                            ondragstart: move |_| dragged_task_id.set(Some(task.id)),
                            class: "bg-space-900 p-3 rounded border border-space-700 cursor-move hover:border-neon-cyan transition-colors shadow-sm group",
                            div { class: "flex justify-between items-start",
                                span { class: "text-sm font-medium leading-tight", "{task.title}" }
                                span { class: "text-[10px] font-mono p-1 rounded bg-space-800 text-gray-500", 
                                    match task.quadrant {
                                        Quadrant::DoFirst => "DO",
                                        Quadrant::Schedule => "SCH",
                                        _ => ""
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // THE MAIN CALENDAR GRID
            div { class: "flex-1 flex flex-col min-w-0",
                // HEADER: Navigation controls and Day labels
                div { class: "flex border-b border-space-700 bg-space-900",
                    // Previous Week Button
                    div { class: "w-16 border-r border-space-700 p-4 flex justify-center items-center bg-space-800/30",
                         button { 
                            class: "text-gray-400 hover:text-white pb-1",
                            onclick: move |_| {
                                let new_date = *current_week_start.read() - chrono::Duration::weeks(1);
                                current_week_start.set(new_date);
                            },
                            "←" 
                        }
                    }
                    
                    // Display each day of the week
                    div { class: "flex-1 grid grid-cols-7 divide-x divide-space-700",
                        for day in &week_days {
                            div { class: "p-2 text-center",
                                div { class: "text-xs font-mono text-neon-cyan uppercase tracking-wider", "{day.format(\"%a\")}" }
                                div { class: "text-lg font-bold text-white", "{day.format(\"%d\")}" }
                            }
                        }
                    }

                    // Next Week Button
                    div { class: "w-16 border-l border-space-700 p-4 flex justify-center items-center bg-space-800/30",
                        button { 
                            class: "text-gray-400 hover:text-white pb-1",
                            onclick: move |_| {
                                let new_date = *current_week_start.read() + chrono::Duration::weeks(1);
                                current_week_start.set(new_date);
                            },
                            "→" 
                        }
                    }
                }

                // THE TIME GRID: Rows represent hours of the day
                div { class: "flex-1 flex",
                    // Left Column: Time labels (00:00 to 23:00)
                    div { class: "w-16 border-r border-space-700/30 bg-space-900 flex-shrink-0 flex flex-col h-full",
                        for hour in 0..24 {
                            div { class: "flex-1 border-b border-white/5 flex items-start justify-center pt-1 text-[10px] font-mono text-gray-400/50",
                                "{hour:02}:00"
                            }
                        }
                    }

                    // MAIN AREA: 7 Columns for the 7 days
                    div { class: "flex-1 grid grid-cols-7 divide-x divide-space-700/50 bg-space-900/50 relative h-full",
                        for day in week_days.clone() {
                            div { 
                                class: "relative group flex flex-col h-full",
                                // Create 24 invisible drop zones for each hour
                                for hour in 0..24 {
                                    div { 
                                        class: "flex-1 border-b border-white/5 hover:bg-white/[0.02] transition-colors relative",
                                        // HANDLE STRETCHING: Update the task end time when hovering while stretching
                                        onmouseenter: move |_| {
                                            if let Some(id) = stretching_task_id() {
                                                let end_dt = day.and_hms_opt(hour as u32, 0, 0).unwrap().and_local_timezone(Local).unwrap() + chrono::Duration::hours(1);
                                                let tasks = app_state.tasks.read();
                                                if let Some(task) = tasks.iter().find(|t| t.id == id) {
                                                    if let Some(start) = task.scheduled_start {
                                                        if end_dt > start {
                                                            drop(tasks);
                                                            app_state.update_task_schedule(id, Some(start), Some(end_dt));
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        ondragover: move |e| e.prevent_default(),
                                        // HANDLE DROPPING: Set the task start time to this hour
                                        ondrop: move |_| {
                                            if let Some(id) = dragged_task_id() {
                                                let start_dt = day.and_hms_opt(hour as u32, 0, 0).unwrap().and_local_timezone(Local).unwrap();
                                                let end_dt = start_dt + chrono::Duration::hours(1);
                                                app_state.update_task_schedule(id, Some(start_dt), Some(end_dt));
                                                dragged_task_id.set(None);
                                            }
                                        }
                                    }
                                }
                                
                                // RENDER OVERLAY: Show the colored task blocks on top of the grid
                                DayTasks {
                                    tasks: scheduled_tasks.clone(),
                                    day: day,
                                    dragged_task_id: dragged_task_id,
                                    stretching_task_id: stretching_task_id
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// SUB-COMPONENT: Renders the blocks for scheduled tasks within a single day
#[component]
fn DayTasks(
    tasks: Vec<Task>, 
    day: NaiveDate, 
    dragged_task_id: Signal<Option<uuid::Uuid>>,
    stretching_task_id: Signal<Option<uuid::Uuid>>,
) -> Element {
    let app_state = use_context::<AppState>();

    // Step 1: Filter tasks for this day and calculate their physical position/height as percentages
    let day_tasks: Vec<(Task, f32, f32, chrono::DateTime<Local>)> = tasks.iter()
        .filter(|t| t.scheduled_start.map(|dt| dt.date_naive() == day).unwrap_or(false))
        .filter_map(|t| {
            let start = t.scheduled_start?;
            let end = t.scheduled_end.unwrap_or(start + chrono::Duration::hours(1));
            
            let start_hours = start.hour() as f32 + (start.minute() as f32 / 60.0);
            let end_hours = end.hour() as f32 + (end.minute() as f32 / 60.0);
            let end_hours = if end_hours <= start_hours { 24.0 } else { end_hours }; 
            
            let top_percent = (start_hours / 24.0) * 100.0;
            let height_percent = ((end_hours - start_hours) / 24.0) * 100.0;
            
            Some((t.clone(), top_percent, height_percent, start))
        })
        .collect();

    // Step 2: Map each task to a colored UI block
    let elements = day_tasks.into_iter().map(|(task, top_percent, height_percent, start)| {
        let key_str = task.id.to_string();
        let style_str = format!("top: {}%; height: {}%;", top_percent, height_percent);
        let title = task.title.clone();
        let time_str = start.format("%H:%M").to_string();
        let task_id = task.id;

        rsx! {
            div {
                key: "{key_str}",
                class: "absolute left-1 right-1 rounded p-1 text-[10px] border bg-neon-cyan/5 border-neon-cyan/30 text-neon-cyan/80 hover:bg-neon-cyan/10 hover:border-neon-cyan/60 cursor-move z-10 overflow-hidden min-h-[1.5rem] group/task transition-all",
                style: "{style_str}",
                draggable: true,
                ondragstart: move |_| dragged_task_id.set(Some(task_id)),
                
                div { class: "font-semibold truncate", "{title}" }
                div { class: "opacity-60 text-[9px] font-mono", "{time_str}" }
                
                // ACTION - REMOVE: Clicking 'x' unschedules the task
                button { 
                    class: "absolute top-1 right-1 text-neon-cyan/40 hover:text-neon-cyan opacity-0 group-hover/task:opacity-100 transition-opacity",
                    onclick: move |e| {
                        e.stop_propagation();
                        app_state.update_task_schedule(task_id, None, None);
                    },
                    "×"
                }

                // ACTION - RESIZE: Drag this handle to increase the task duration
                div { 
                    class: "absolute bottom-0 left-0 right-0 h-1.5 cursor-ns-resize hover:bg-neon-cyan/50 opacity-0 group-hover/task:opacity-100 transition-all flex items-center justify-center",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        stretching_task_id.set(Some(task_id));
                    },
                    div { class: "w-4 h-0.5 bg-neon-cyan/30 rounded-full" }
                }
            }
        }
    });

    rsx! {
        {elements}
    }
}
