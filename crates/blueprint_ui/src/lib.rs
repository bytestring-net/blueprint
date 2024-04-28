#![doc = include_str!("../../../README.md")]

// #======================#
// #=== PRELUDE EXPORT ===#


pub mod structs;
pub use structs::*;

pub mod systems;
pub use systems::*;


pub mod prelude {
    pub use super::structs::*;
    pub use super::systems::*;
}
