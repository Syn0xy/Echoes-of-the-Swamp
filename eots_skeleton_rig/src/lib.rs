mod builder;
mod components;
mod inverse_kinematic;
mod plugin;

pub mod debug;
pub mod descriptions;
pub mod utils;

pub use builder::build_skeleton_rig;
pub use components::*;
pub use plugin::SkeletonRigPlugin;
