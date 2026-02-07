# Compiler Errors & Solutions

## 1. RSX Macro Ambiguity in Loop
**Problem:** `error: Expected Ident or Expression` inside `rsx!` macro when using conditional rendering or loops with complex expressions.
**Context:** Occurred in `src/views/calendar.rs` when trying to render tasks or days.
**Solution:**
- Simplified the `rsx!` structure.
- Extracted iterator logic outside the `rsx!` macro into a variable (e.g., `let tasks = ...`).
- Used explicit `.to_string()` for `key` attributes to avoid type inference issues (e.g., `key: "{task.id.to_string()}"`).

## 2. Mutable Borrow in Closure
**Problem:** `error[E0596]: cannot borrow ... as mutable, as it is not declared as mutable` inside `move` closures for event handlers.
**Context:** Drag-and-drop handlers in `src/views/matrix.rs`.
**Solution:**
- Shadowed the signal variable inside the closure with `let mut signal = signal;` immediately before using it mutably.
- This allows the closure to capture the signal and mutate it correctly.

## 3. Borrow Conflict (Read/Write)
**Problem:** `error[E0502]: cannot borrow ... as mutable because it is also borrowed as immutable`.
**Context:** In `ondrop` handler, trying to read `dragged_id` (immutable borrow) and then set it to `None` (mutable borrow) in the same scope.
**Solution:**
- Copied the value out of the read guard first: `let id = *dragged_id.read();`
- Dropped the read guard (implicitly or explicitly).
- Then performed the mutable operation: `dragged_id.set(None);`
