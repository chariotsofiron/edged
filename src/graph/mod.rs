//! A graph is a collection of nodes and edges.
//! <https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)>
pub mod matrix;
pub mod traits;
#[allow(
    clippy::arithmetic_side_effects,
    clippy::integer_arithmetic,
    clippy::integer_division
)]
pub mod util;
pub mod visit_map;
