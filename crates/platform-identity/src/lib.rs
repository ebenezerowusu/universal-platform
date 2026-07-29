//! Identity foundation for Universal Platform.
//!
//! This crate owns identity-specific primitives and application-facing contracts.
//! It must remain domain-agnostic and must not contain Religious, Commerce, HR,
//! or other domain-specific concepts.

pub mod models;
pub mod ports;
pub mod service;

pub use models::{EmailAddress, IdentityUser, IdentityUserStatus, PasswordHash};
pub use ports::{IdentityRepository, PasswordHasher};
pub use service::{CreateUserInput, IdentityService};
