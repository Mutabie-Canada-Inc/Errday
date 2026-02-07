# 🐛 Bug Tracker & Known Issues

This document serves as a central registry for known bugs, incomplete features, and technical debt in the **Errday** application.

## 📋 Issue Reporting Template

Copy and paste the following template when adding a new issue:

```markdown
### [Bug/Feature] Short Title Here
- **Status**: [🔴 Open / 🟡 In Progress / 🟢 Resolved]
- **Priority**: [High/Medium/Low]
- **Location**: `src/views/filename.rs` (or Component Name)
- **Description**: Brief description of the issue or missing functionality.
- **Reproduction Steps**:
  1. Go to '...'
  2. Click on '...'
  3. Scroll down to '...'
  4. See error.
- **Expected Behavior**: What should happen?
- **Proposed Solution** (Optional): value...
```

---

## 🔴 Known Issues & Incomplete Features

### [Feature] Flight Plan (Calendar) Implementation
- **Status**: 🔴 Open
- **Priority**: High
- **Location**: `src/views/calendar.rs`
- **Description**: The Calendar view logic is currently stubbed. There are multiple compiler warnings regarding unused variables (`week_days`, `tasks`, `Task` import). The time-blocking functionality is not yet active.
- **Expected Behavior**: Users should be able to drag tasks from the "Schedule" quadrant onto specific days/times.

### [UI/UX] Info Bubble Z-Index & Stacking
- **Status**: 🟡 In Progress (Recent Fixes Applied)
- **Priority**: Medium
- **Location**: `src/views/matrix.rs`
- **Description**: Info bubbles in the Matrix view have historically clipped into adjacent boxes or been hidden behind other UI elements due to complex stacking contexts (opacity, transforms, glassmorphism).
- **Current State**: `z-index` values have been increased (`z-[9999]`) and `hover:scale` effects removed to prevent new stacking contexts, but this remains a fragile UI component requiring regression testing on different screen sizes.

### [Refactor] Data Persistence Robustness
- **Status**: 🟡 In Progress
- **Priority**: High
- **Location**: `src/store.rs`
- **Description**: The application currently uses a basic persistence mechanism. It needs rigorous testing to ensure data integrity during crashes or forced exits, especially for the drag-and-drop state changes in the Matrix.

### [UI] Window Resizing Extremes
- **Status**: 🔴 Open
- **Priority**: Low
- **Location**: `src/views/inbox.rs`, `src/views/matrix.rs`
- **Description**: While optimizations for "thinner" screens have been made, extreme resizing (very small width/height) may still cause layout breaks or overlapping elements in the fixed-height Matrix grid.

### [Compiler] Unused Code Warnings
- **Status**: 🟡 In Progress
- **Priority**: Low
- **Location**: Global
- **Description**: `cargo check` reports several `unused_variable` and `dead_code` warnings, primarily in `calendar.rs` and `store.rs`.
