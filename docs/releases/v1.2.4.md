# Release Notes: Errday v1.2.4

**Supported Platforms:** macOS (Apple Silicon ARM, Intel x86_64) and Linux Distributions.

We are excited to announce the release of **Errday v1.2.4**! This update heavily focuses on supercharging the Calendar view, tightening UI details in the Eisenhower Matrix, and providing much clearer documentation and onboarding flows.

---

## What's New 🚀

- **Download Calendar as `.ics`:** You can now seamlessly export your scheduled task layout directly to an `.ics` file. This allows you to effortlessly sync your Errday "Flight Plan" with Google Calendar, Apple Calendar, Outlook, and other standard calendar clients.
- **Extend Task Durations directly in the Calendar:** When scheduling tasks on the grid, you can now drag the bottom handle of any task block to easily decrease or extend its duration in 15-minute increments.
- **Enhanced Tutorial Experience:** The Flight Manual has been significantly upgraded. It now provides a clearer, more intuitive onboarding flow to help users master both the brain dump and the matrix.
- **Enhanced Security & CVE Documentation:** We have committed to transparency by compiling dedicated documentation that tracks current CVEs and security auditing metrics specifically for Rust and Dioxus projects, guaranteeing peace of mind regarding dependency safety.
- **Single-Source Version Control:** The application's version number is now dynamically strictly controlled from a single place (`Cargo.toml`). This seamlessly minimizes developer overhead manually updating variables twice for production artifacts.

---

## Bug Fixes & Improvements 🛠️

- **Info Bubbles Relocation to Tutorial:** To minimize visual distraction during deep work sessions, all floating informational tooltips/bubbles have been migrated out of the primary UI. They are now centralized within the Tutorial Page acting as a persistent reference index.
- **Internal Matrix Scrolling:** Fixed a major visibility issue by introducing dedicated scrollbars *inside* each of the 4 Eisenhower quadrant boxes. No matter how many tasks you drag into a specific quadrant, all tasks remain fully viewable and accessible.
- **Whitespace UI Bugs Resolved:** We conducted a thorough clean-up of the user interface by injecting reasonable, calculated whitespace around elements. This eliminates cramped collision bugs and results in a much cleaner, breathable, and premium aesthetic.
- **Simplified Navigation Labelling:** Rebranded specific tabs on the sidebar mapping to clearer, modern standards. "Flight Plan" is now accurately titled "**Calendar**", and "System Info" is officially "**Credits**".
