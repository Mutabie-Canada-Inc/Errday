# Product Requirements Document (PRD)

## 1. Project Overview
**Product Name:** Errday
**Tagline:** Mission Control for Founders
**Version:** 1.0 (MVP)

**Errday** is a standalone, offline-first desktop application designed for startup founders and busy professionals. It combines the strategic clarity of the **Eisenhower Matrix** with the tactical execution of **Time Blocking**. 

Built with **Rust** and **Dioxus**, Errday prioritizes performance, privacy, and simplicity. It operates entirely locally without cloud dependencies, ensuring data ownership and lightning-fast operation.

---

## 2. Target Audience
- **Primary:** Startup Founders, CEOs, and Entrepreneurs.
- **Secondary:** Freelancers, students, and professionals managing high-volume tasks.
- **Pain Point:** Overwhelmed by daily tasks, difficulty prioritizing effectively, and aversion to bloated, subscription-based productivity tools.

---

## 3. Design & User Experience (UX)
### 3.1. Aesthetic Theme: "Aerospace in Space"
- **Visual Style:** Deep space ambiance meets high-tech cockpit interface.
    - **Backgrounds:** Deep blues (`#0B0D17`), void blacks, and subtle starfield hints (optional/minimal).
    - **Accents:** Neon cyans, ambers, and clean whites for active elements.
    - **Material:** Glassmorphism (frosted glass) for panels, reminiscent of heads-up displays (HUDs).
- **Typography:** Monospace fonts for data/numbers (e.g., *JetBrains Mono*, *Fira Code*) paired with clean sans-serif for UI text (e.g., *Inter*, *San Francisco* style).
- **Feel:** Premium, utilitarian but sleek. "If Apple designed a spaceship dashboard."

### 3.2. UX Principles
- **Minimalism:** No clutter. Every element must earn its place on the screen.
- **Ergonomics:** Controls placed where the eye naturally falls.
    - **Psychological Layout:** Common actions (Add Task) accessible via bottom-right or center. Navigation on the left or top (consistent).
    - **F-Pattern Scanning:** Key information (Urgent/Important tasks) placed top-left.
- **Interactivity:**
    - Smooth hover states (`transform: scale(1.02)`).
    - Micro-animations for task completion and transitions.
    - "Vibecoded" but professional—satisfying clicks and switches.

---

## 4. Functional Requirements

### 4.1. Core Workflow
The application follows a strict 3-step productivity funnel:
1.  **Capture (Brain Dump):** Get everything out of your head.
2.  **Process (Eisenhower Matrix):** Decisions on what to do, defer, delegate, or delete.
3.  **Execute (Time Blocking):** Schedule the "Do" tasks into reality. Ordered by Urgency and Importance down to no Urgency and no Importance.

### 4.2. Feature Specifications

#### I. Task Capture (The "Inbox")
- **Goal:** Rapid entry of tasks.
- **Interface:** Simple text input field (CMD+N shortcut).
- **Data:** Task Title, Description (optional).
- **State:** Tasks are initially "Unsorted".

#### II. Eisenhower Matrix (The "Control Center")
- **Goal:** Classify tasks by Urgency and Importance.
- **Interface:** Drag-and-drop interface with 4 quadrants:
    1.  **Do First** (Urgent & Important)
    2.  **Schedule** (Less Urgent & Important)
    3.  **Delegate** (Urgent & Less Important)
    4.  **Delete** (Less Urgent & Less Important)
- **Action:** Users drag "Unsorted" tasks into one of these quadrants.

#### III. Time Blocking (The "Flight Plan")
- **Goal:** Assign actual time slots to "Do First" and "Schedule" tasks.
- **Interface:**
    - **Weekly View:** 7-column layout (Mon-Sun).
    - **Daily View:** Granular hourly timeline.
    - **Mechanism:** Drag tasks from the "Do First" list onto the calendar to block time.
- **Constraints:** Prevent overlapping blocks (warn user).

#### IV. Calendar Export
- **Goal:** Integration with external life.
- **Feature:** "Export Schedule" button.
- **Format:** Generates a standard `.ics` (iCalendar) file.
- **Compatibility:** Google Calendar, Outlook, Apple Calendar.

### 4.3. Data Persistence
- **Strategy:** Local-first, file-based persistence.
- **Format:** JSON or TOML (human-readable, robust).
- **Location:** Standard User Data Directory (e.g., `~/Library/Application Support/Errday/` on macOS).
- **Security:** Files stored in user-space only. No cloud upload.

---

## 5. Technical Constraints & Performance

### 5.1. Platform Support
- **OS:** macOS (Apple Silicon & Intel), Windows 10/11, Linux (Ubuntu/Mint).
- **Architecture:** `aarch64`, `x86_64`.

### 5.2. Performance Metrics
- **App Size:** **Strictly < 100MB** (Target: < 30MB executable).
- **Memory:** Low footprint (< 150MB RAM idle).
- **Startup Time:** < 500ms.
- **Responsiveness:** 60fps UI interactions.

### 5.3. Tech Stack
- **Language:** Rust (Stable).
- **Framework:** Dioxus (Desktop feature).
- **Styling:** TailwindCSS (via Dioxus) or custom CSS for specific themes.
- **Icons:** Heroicons or Lucide (Rust crates).

---

## 6. Out of Scope (MVP)
- Cloud Synchronization (iCloud/Dropbox sync only via file system).
- Mobile App.
- Team Collaboration features.
- Native Calendar API integration (read/write direct to OS calendar) - Export only for now.