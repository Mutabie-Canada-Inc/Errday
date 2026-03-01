use dioxus::prelude::*;
use crate::store::AppState;
use crate::models::{Task, Quadrant};
use chrono::{Datelike, Timelike, Local, NaiveDate};
use icalendar::{Calendar as ICalendar, Component, Event, EventLike};

// ─── CONSTANTS ───────────────────────────────────────────────────────────────
/// Total height of the scrollable grid in pixels — 60px per hour × 24 hours
const GRID_HEIGHT_PX: u32 = 1440;
/// Height of a single hour row
const HOUR_HEIGHT_PX: u32 = GRID_HEIGHT_PX / 24; // 60px

// ─── MAIN CALENDAR COMPONENT ────────────────────────────────────────────────

/// Weekly calendar view modeled after Apple Calendar / Google Calendar.
/// X-axis = Days (Mon–Sun), Y-axis = Hours (00:00–23:00) in 24h clock.
#[component]
pub fn Calendar() -> Element {
    let app_state = use_context::<AppState>();

    // ── Navigation State ────────────────────────────────────────────────────
    let mut week_start = use_signal(|| {
        let now = Local::now();
        let offset = now.weekday().num_days_from_monday();
        now.date_naive() - chrono::Duration::days(offset as i64)
    });

    // ── Interaction State ───────────────────────────────────────────────────
    let mut dragged_task_id = use_signal(|| None::<uuid::Uuid>);
    let mut stretching_task_id = use_signal(|| None::<uuid::Uuid>);

    // ── Derived Data ────────────────────────────────────────────────────────
    let week_days: Vec<NaiveDate> = (0..7)
        .map(|i| week_start.read().checked_add_signed(chrono::Duration::days(i)).unwrap())
        .collect();

    let today = Local::now().date_naive();
    let now = Local::now();
    let current_time_pct = ((now.hour() as f32 + now.minute() as f32 / 60.0) / 24.0) * 100.0;

    // Tasks eligible for the calendar (Do First + Schedule quadrants only)
    let all_eligible: Vec<Task> = app_state
        .tasks
        .read()
        .iter()
        .filter(|t| matches!(t.quadrant, Quadrant::DoFirst | Quadrant::Schedule))
        .cloned()
        .collect();
    let (scheduled, unscheduled): (Vec<Task>, Vec<Task>) =
        all_eligible.into_iter().partition(|t| t.scheduled_start.is_some());

    // Week label for the header
    let week_end = week_days.last().cloned().unwrap_or(today);
    let week_label = format!(
        "{} – {}",
        week_start.read().format("%b %d"),
        week_end.format("%b %d, %Y")
    );

    rsx! {
        div {
            // ROOT: Apply ns-resize cursor globally during active stretch
            class: if stretching_task_id().is_some() { "flex h-full bg-space-900 text-gray-100 font-sans overflow-hidden select-none cursor-ns-resize" } else { "flex h-full bg-space-900 text-gray-100 font-sans overflow-hidden select-none" },
            // Release stretch on mouseup anywhere
            onmouseup: move |_| stretching_task_id.set(None),

            // ─── LEFT SIDEBAR: Unscheduled Tasks ────────────────────────
            div { class: "w-64 bg-space-800/70 border-r border-space-700/60 flex flex-col shrink-0",
                // Sidebar header
                div { class: "p-4 border-b border-space-700/40 flex justify-between items-center",
                    div {
                        h2 { class: "text-sm font-bold text-white tracking-wide", "Unscheduled" }
                        p { class: "text-[10px] text-gray-500 font-mono mt-0.5 uppercase tracking-widest", "Drag to schedule" }
                    }
                    // ICS Export Button
                    button {
                        class: "p-2 rounded-lg hover:bg-space-700/60 text-gray-400 hover:text-neon-cyan transition-all duration-200",
                        title: "Export Calendar (.ics)",
                        onclick: move |_| {
                            let tasks = scheduled.clone();
                            dioxus::prelude::spawn(async move {
                                export_ics(&tasks).await;
                            });
                        },
                        svg { class: "w-4 h-4", fill: "none", stroke: "currentColor", view_box: "0 0 24 24",
                            path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" }
                        }
                    }
                }

                // Task list
                div { class: "flex-1 overflow-y-auto p-3 space-y-2",
                    if unscheduled.is_empty() {
                        div { class: "text-center py-12 text-gray-600 text-xs italic font-mono", "All tasks scheduled." }
                    }
                    for task in unscheduled {
                        {
                            let badge_color = quadrant_badge_color(&task.quadrant);
                            rsx! {
                                div {
                                    key: "{task.id}",
                                    draggable: true,
                                    ondragstart: move |_| dragged_task_id.set(Some(task.id)),
                                    class: "bg-space-900/60 px-3 py-2.5 rounded-lg border border-space-700/50 cursor-grab hover:border-white/15 transition-all duration-200 group active:cursor-grabbing",
                                    div { class: "flex items-start justify-between gap-2",
                                        span { class: "text-[13px] font-medium text-gray-200 leading-snug group-hover:text-white transition-colors", "{task.title}" }
                                        span { class: "shrink-0 text-[9px] font-bold px-1.5 py-0.5 rounded {badge_color}",
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
                }
            }

            // ─── MAIN CALENDAR AREA ─────────────────────────────────────
            div { class: "flex-1 flex flex-col min-w-0",

                // ── Top Bar: Week navigation ────────────────────────────
                div { class: "h-14 border-b border-space-700/60 bg-space-800/50 flex items-center px-4 gap-4 shrink-0",
                    button {
                        class: "w-12 h-12 rounded-full hover:bg-space-700 text-gray-400 hover:text-white flex items-center justify-center transition-colors text-2xl font-bold",
                        onclick: move |_| {
                            let new_date = *week_start.read() - chrono::Duration::weeks(1);
                            week_start.set(new_date);
                        },
                        "‹"
                    }
                    button {
                        class: "w-12 h-12 rounded-full hover:bg-space-700 text-gray-400 hover:text-white flex items-center justify-center transition-colors text-2xl font-bold",
                        onclick: move |_| {
                            let new_date = *week_start.read() + chrono::Duration::weeks(1);
                            week_start.set(new_date);
                        },
                        "›"
                    }
                    span { class: "text-base font-semibold text-white ml-2", "{week_label}" }
                    // Today button
                    button {
                        class: "ml-auto text-sm font-semibold px-4 py-2 rounded-md border border-space-700/60 text-gray-300 hover:text-white hover:bg-space-700/40 transition-all",
                        onclick: move |_| {
                            let now = Local::now();
                            let offset = now.weekday().num_days_from_monday();
                            week_start.set(now.date_naive() - chrono::Duration::days(offset as i64));
                        },
                        "Today"
                    }
                }

                // ── Day Header Row (sticky) ─────────────────────────────
                div { class: "flex border-b border-space-700/60 bg-space-900 shrink-0 z-20",
                    // Time gutter spacer
                    div { class: "w-14 shrink-0 border-r border-space-700/30" }
                    // Day columns
                    div { class: "flex-1 grid grid-cols-7",
                        for day in &week_days {
                            {
                                let is_today = *day == today;
                                rsx! {
                                    div {
                                        class: "py-2 flex flex-col items-center justify-center border-r border-space-700/20 last:border-r-0",
                                        span {
                                            class: if is_today { "text-[11px] font-semibold text-neon-cyan uppercase tracking-widest mb-1" } else { "text-[11px] font-medium text-gray-500 uppercase tracking-widest mb-1" },
                                            "{day.format(\"%a\")}"
                                        }
                                        span {
                                            class: if is_today { "text-lg font-bold w-10 h-10 flex items-center justify-center rounded-full bg-neon-cyan text-space-900 shadow-md" } else { "text-lg font-medium text-gray-300 w-10 h-10 flex items-center justify-center" },
                                            "{day.format(\"%e\")}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // ── Scrollable Time Grid ────────────────────────────────
                div { class: "flex-1 overflow-y-auto overflow-x-hidden relative",
                    div {
                        class: "flex relative",
                        style: "height: {GRID_HEIGHT_PX}px; min-height: {GRID_HEIGHT_PX}px;",

                        // ── Time Gutter (Y-axis labels) ─────────────────
                        div { class: "w-14 shrink-0 border-r border-space-700/30 relative bg-space-900/80 z-20",
                            for hour in 0..24 {
                                div {
                                    class: "absolute -right-1 z-30 text-[14px] font-mono font-bold text-white leading-none bg-space-900 px-1",
                                    style: "top: {hour * HOUR_HEIGHT_PX}px; transform: translateY(-50%);",
                                    if hour == 0 { "" } else {
                                        "{hour:02}:00"
                                    }
                                }
                            }
                        }

                        // ── Grid Area (7 day columns) ───────────────────
                        div { class: "flex-1 grid grid-cols-7 relative",

                            // Horizontal hour lines (full-width background)
                            div { class: "absolute inset-0 pointer-events-none z-0",
                                for hour in 0..24 {
                                    div {
                                        class: "absolute left-0 right-0 border-t border-space-700/25",
                                        style: "top: {hour * HOUR_HEIGHT_PX}px;",
                                    }
                                }
                            }

                            // Day columns with vertical dotted separators
                            for (col_idx, &day) in week_days.iter().enumerate() {
                                div {
                                    class: if col_idx > 0 { "relative border-l border-dashed border-space-700/40" } else { "relative" },
                                    style: "height: {GRID_HEIGHT_PX}px;",

                                    // Current time indicator (red line) — only on today
                                    if day == today {
                                        div {
                                            class: "absolute left-0 right-0 z-40 pointer-events-none flex items-center",
                                            style: "top: {current_time_pct}%;",
                                            div { class: "w-2.5 h-2.5 rounded-full bg-red-500 -ml-[5px] shadow-[0_0_6px_rgba(239,68,68,0.7)]" }
                                            div { class: "flex-1 border-t-2 border-red-500" }
                                        }
                                    }

                                    // 15-minute interactive drop zones (96 per day)
                                    for hour in 0..24i32 {
                                        for quarter in 0..4i32 {
                                            {
                                                let minute = quarter * 15;
                                                let slot_top = hour as u32 * HOUR_HEIGHT_PX + quarter as u32 * (HOUR_HEIGHT_PX / 4);
                                                let slot_height = HOUR_HEIGHT_PX / 4;
                                                // Raise z-index above blocks (z-20) when stretching or dragging
                                                let zone_z = if stretching_task_id().is_some() || dragged_task_id().is_some() { "absolute left-0 right-0 z-30 hover:bg-white/[0.03] transition-colors" } else { "absolute left-0 right-0 z-10 hover:bg-white/[0.03] transition-colors cursor-pointer" };
                                                rsx! {
                                                    div {
                                                        class: "{zone_z}",
                                                        style: "top: {slot_top}px; height: {slot_height}px;",
                                                        ondragover: move |e| e.prevent_default(),
                                                        ondrop: move |_| {
                                                            if let Some(id) = dragged_task_id() {
                                                                let start_dt = day
                                                                    .and_hms_opt(hour as u32, minute as u32, 0)
                                                                    .unwrap()
                                                                    .and_local_timezone(Local)
                                                                    .unwrap();
                                                                
                                                                let mut new_end = start_dt + chrono::Duration::hours(1);
                                                                let tasks = app_state.tasks.read();
                                                                if let Some(task) = tasks.iter().find(|t| t.id == id) {
                                                                    if let (Some(old_start), Some(old_end)) = (task.scheduled_start, task.scheduled_end) {
                                                                        let dur = old_end.signed_duration_since(old_start);
                                                                        new_end = start_dt + dur;
                                                                    }
                                                                }
                                                                drop(tasks);

                                                                app_state.update_task_schedule(id, Some(start_dt), Some(new_end));
                                                                dragged_task_id.set(None);
                                                            }
                                                        },
                                                        onmouseenter: move |_| {
                                                            if let Some(id) = stretching_task_id() {
                                                                // Calculate the END time this slot represents
                                                                let mut end_min = (minute + 15) as u32;
                                                                let mut end_hour = hour as u32;
                                                                if end_min >= 60 {
                                                                    end_min = 0;
                                                                    end_hour += 1;
                                                                }

                                                                let end_dt = if end_hour >= 24 {
                                                                    day.and_hms_opt(0, 0, 0)
                                                                        .unwrap()
                                                                        .and_local_timezone(Local)
                                                                        .unwrap()
                                                                        + chrono::Duration::hours(24)
                                                                } else {
                                                                    day.and_hms_opt(end_hour, end_min, 0)
                                                                        .unwrap()
                                                                        .and_local_timezone(Local)
                                                                        .unwrap()
                                                                };

                                                                let tasks = app_state.tasks.read();
                                                                if let Some(task) = tasks.iter().find(|t| t.id == id) {
                                                                    if let Some(start) = task.scheduled_start {
                                                                        if end_dt > start {
                                                                            let mins = end_dt.signed_duration_since(start).num_minutes();
                                                                            let final_end = if mins > 24 * 60 {
                                                                                start + chrono::Duration::hours(24)
                                                                            } else {
                                                                                end_dt
                                                                            };
                                                                            drop(tasks);
                                                                            app_state.update_task_schedule(id, Some(start), Some(final_end));
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Rendered task blocks for this day
                                    DayTaskBlocks {
                                        tasks: scheduled.clone(),
                                        day: day,
                                        dragged_task_id: dragged_task_id,
                                        stretching_task_id: stretching_task_id,
                                        is_dragging: dragged_task_id().is_some(),
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

// ─── TASK BLOCK OVERLAY COMPONENT ────────────────────────────────────────────

/// Renders the positioned task blocks for a single day column.
#[component]
fn DayTaskBlocks(
    tasks: Vec<Task>,
    day: NaiveDate,
    dragged_task_id: Signal<Option<uuid::Uuid>>,
    stretching_task_id: Signal<Option<uuid::Uuid>>,
    is_dragging: bool,
) -> Element {
    let app_state = use_context::<AppState>();
    
    // State to track if a task is currently being edited
    let mut editing_task_id = use_signal(|| None::<uuid::Uuid>);

    let blocks: Vec<(Task, u32, u32, String)> = tasks
        .iter()
        .filter(|t| t.scheduled_start.map(|dt| dt.date_naive() == day).unwrap_or(false))
        .filter_map(|t| {
            let start = t.scheduled_start?;
            let end = t.scheduled_end.unwrap_or(start + chrono::Duration::hours(1));

            let start_mins = start.hour() * 60 + start.minute();
            let mut end_mins = end.hour() * 60 + end.minute();
            if end_mins <= start_mins {
                end_mins = 24 * 60; // wraps to end of day
            }

            let top_px = start_mins * (HOUR_HEIGHT_PX / 60);
            let height_px = (end_mins - start_mins) * (HOUR_HEIGHT_PX / 60);
            let height_px = height_px.max(15); // minimum visible height

            let time_str = format!("{} – {}", start.format("%-I:%M"), end.format("%-I:%M %p"));

            Some((t.clone(), top_px, height_px, time_str))
        })
        .collect();

    let elements = blocks.into_iter().map(|(task, top_px, height_px, time_str)| {
        let task_id = task.id;
        let title = task.title.clone();
        let (block_bg, block_border) = quadrant_block_colors(&task.quadrant);

        // When another task is being dragged, make all blocks pass-through
        // so HTML5 drop events land on the grid zones beneath.
        let block_pointer = if is_dragging { "pointer-events-none" } else { "" };

        rsx! {
            div {
                key: "{task_id}",
                // ── SECTION: Task Block Container ────────────────
                // z-20 so drop zones (z-10 idle / z-30 active) can layer correctly
                class: "absolute left-1 right-2 rounded-md z-20 group/block transition-shadow duration-200 hover:shadow-lg border-l-[3px] outline outline-1 outline-white/10 {block_border} {block_bg} {block_pointer}",
                style: "top: {top_px}px; height: {height_px}px;",
                draggable: if editing_task_id() != Some(task_id) { "true" } else { "false" },
                ondragstart: move |_| {
                    if editing_task_id() != Some(task_id) {
                        dragged_task_id.set(Some(task_id))
                    }
                },
                ondoubleclick: move |e| {
                    e.stop_propagation();
                    editing_task_id.set(Some(task_id));
                },

                // Content
                div { class: "px-2 py-1 h-full flex flex-col overflow-hidden",
                    // Title rendering (normal vs inline input edit)
                    if editing_task_id() == Some(task_id) {
                        input {
                            class: "w-full bg-black/40 text-white text-[20px] font-semibold px-1 py-0.5 rounded border border-white/30 focus:outline-none focus:border-neon-cyan",
                            value: "{title}",
                            autofocus: true,
                            onkeydown: move |e| {
                                if e.key() == dioxus::events::Key::Enter {
                                    editing_task_id.set(None);
                                } else if e.key() == dioxus::events::Key::Escape {
                                    editing_task_id.set(None);
                                }
                            },
                            oninput: move |e| {
                                let new_val = e.value();
                                let mut tasks_sig = app_state.tasks;
                                let mut tasks = tasks_sig.write();
                                if let Some(t) = tasks.iter_mut().find(|x| x.id == task_id) {
                                    t.title = new_val;
                                }
                                drop(tasks);
                                app_state.save_tasks();
                            },
                            onblur: move |_| {
                                editing_task_id.set(None);
                            }
                        }
                    } else {
                        div { 
                            class: "text-[11px] font-semibold text-white leading-tight truncate cursor-grab active:cursor-grabbing", 
                            "{title}" 
                        }
                    }
                    
                    if height_px > 30 {
                        div { class: "text-[9px] text-white/60 font-medium mt-0.5 truncate pointer-events-none", "{time_str}" }
                    }
                }

                // Close button (hover reveal)
                button {
                    class: "absolute top-0.5 right-0.5 w-5 h-5 rounded-full bg-black/30 hover:bg-black/60 text-white/50 hover:text-white flex items-center justify-center opacity-0 group-hover/block:opacity-100 transition-opacity backdrop-blur-sm z-10",
                    onclick: move |e| {
                        e.stop_propagation();
                        app_state.update_task_schedule(task_id, None, None);
                    },
                    span { class: "text-[10px] leading-none", "×" }
                }

                // ── SECTION: Resize Handle ──────────────────────
                // Always slightly visible; full opacity on hover.
                // cursor-ns-resize gives the ↕ icon feedback.
                div {
                    class: "absolute bottom-0 left-0 right-0 h-3 cursor-ns-resize opacity-40 group-hover/block:opacity-100 transition-opacity flex items-end justify-center pb-0.5 z-10",
                    onmousedown: move |e| {
                        e.stop_propagation();
                        stretching_task_id.set(Some(task_id));
                    },
                    div { class: "w-8 h-[3px] rounded-full bg-white/50 group-hover/block:bg-white/80" }
                }
            }
        }
    });

    rsx! { {elements} }
}

// ─── HELPER FUNCTIONS ────────────────────────────────────────────────────────

/// Returns Tailwind classes for the sidebar quadrant badge.
fn quadrant_badge_color(q: &Quadrant) -> &'static str {
    match q {
        Quadrant::DoFirst => "bg-neon-pink/20 text-neon-pink",
        Quadrant::Schedule => "bg-neon-cyan/20 text-neon-cyan",
        Quadrant::Delegate => "bg-neon-amber/20 text-neon-amber",
        Quadrant::Delete => "bg-space-700 text-gray-400",
        Quadrant::Unsorted => "bg-space-700 text-gray-400",
    }
}

/// Returns (background class, border-left class) for calendar task blocks.
fn quadrant_block_colors(q: &Quadrant) -> (&'static str, &'static str) {
    match q {
        Quadrant::DoFirst => ("bg-neon-pink/20 hover:bg-neon-pink/30", "border-neon-pink"),
        Quadrant::Schedule => ("bg-neon-cyan/20 hover:bg-neon-cyan/30", "border-neon-cyan"),
        Quadrant::Delegate => ("bg-neon-amber/20 hover:bg-neon-amber/30", "border-neon-amber"),
        Quadrant::Delete => ("bg-gray-500/20 hover:bg-gray-500/30", "border-gray-500"),
        Quadrant::Unsorted => ("bg-gray-500/20 hover:bg-gray-500/30", "border-gray-500"),
    }
}

/// Exports all scheduled tasks as an ICS file via a native save dialog.
async fn export_ics(tasks: &[Task]) {
    let mut cal = ICalendar::new();

    for t in tasks {
        if let (Some(start), Some(end)) = (t.scheduled_start, t.scheduled_end) {
            let mut event = Event::new();
            event.summary(&t.title);
            event.starts(start.with_timezone(&chrono::Utc));
            event.ends(end.with_timezone(&chrono::Utc));

            // Build description with Eisenhower status + notes
            let status_label = match t.quadrant {
                Quadrant::DoFirst => "Do First (Urgent & Important)",
                Quadrant::Schedule => "Schedule (Important, Not Urgent)",
                Quadrant::Delegate => "Delegate (Urgent, Not Important)",
                Quadrant::Delete => "Delete (Neither)",
                Quadrant::Unsorted => "Unsorted",
            };
            let notes = t.description.as_deref().unwrap_or("No notes.");
            let desc = format!("Eisenhower Status: {}\n\nNotes:\n{}", status_label, notes);
            event.description(&desc);

            cal.push(event);
        }
    }

    let ics_string = cal.to_string();

    if let Some(handle) = rfd::AsyncFileDialog::new()
        .set_file_name("errday_calendar.ics")
        .add_filter("iCalendar", &["ics"])
        .save_file()
        .await
    {
        let _ = std::fs::write(handle.path(), ics_string);
    }
}
