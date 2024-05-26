#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::panic)]

pub mod constraints;
pub mod error;
#[cfg(test)]
pub mod examples;
pub mod primitives;
pub mod sketch;
pub mod solvers;
