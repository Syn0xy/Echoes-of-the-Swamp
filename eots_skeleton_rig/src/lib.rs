mod builder;
mod components;
mod descriptions;
mod inverse_kinematic;
mod plugin;

pub mod debug;
pub mod utils;

pub use builder::build_skeleton_rig;
pub use components::*;
pub use descriptions::*;
pub use plugin::SkeletonRigPlugin;
