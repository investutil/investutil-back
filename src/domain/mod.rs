// This module contains the core domain logic, including entities, value objects, and domain services.

pub mod entities;
pub mod repositories;
pub mod value_objects;
pub mod events;
pub mod aggregates;

// Re-export commonly used items
pub use entities::*;
pub use repositories::*;