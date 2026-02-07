/// VIEWS MODULE: This connects all our individual page files
pub mod inbox;    // Brain Dump page
pub mod matrix;   // Eisenhower Matrix page
pub mod calendar; // Flight Plan page
pub mod credits;  // System Info page
pub mod tutorial; // Flight Manual page

// Re-export so they can be accessed directly as 'crate::views::Inbox', etc.
pub use inbox::Inbox;
pub use matrix::Matrix;
pub use calendar::Calendar;
pub use credits::Credits;
pub use tutorial::Tutorial;
