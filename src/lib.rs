pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;

// Re-export commonly used items
pub use application::services;
pub use domain::{entities, repositories, value_objects};
pub use infrastructure::persistence;
pub use interfaces::api; 