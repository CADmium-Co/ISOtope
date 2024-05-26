#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::panic)]

pub mod constraints;
pub mod decompose;
pub mod error;
pub mod intersections;
pub mod primitives;
pub mod sketch;
pub mod solvers;

#[cfg(test)]
pub mod examples;
