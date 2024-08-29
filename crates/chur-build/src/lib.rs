#![doc = include_str!("../../../README.md")]

mod cfg;
mod defined_constants;
mod execute;
mod manifest;

pub mod dependency;
pub mod error;

pub use cfg::{Config, ConfigBuilder};
pub use dependency::Dependency;
pub use execute::execute;