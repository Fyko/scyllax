//! The Person entity
//! Viewing the expanded code:
//!
//! ```console
//! cargo expand -p example --lib entities::person::model > example/expanded.rs
//! ```

/// The model itself
pub mod model;

/// All select queries
pub mod queries;
