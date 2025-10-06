use bevy::prelude::*;

use crate::descriptions::SegmentType;

#[derive(Reflect, Component, Debug, Clone)]
pub struct SkeletonRig {
    pub(crate) main_segment: Option<Entity>,
    pub(crate) segments: Vec<Entity>,
}

#[derive(Reflect, Component, Debug, Clone, Copy)]
pub struct SkeletonRigEntity(pub Entity);

#[derive(Reflect, Component, Debug, Clone)]
#[require(Transform)]
pub struct IKRigTarget {
    pub(crate) root_segment: Entity,
    pub(crate) segments: Vec<Entity>,
}

#[derive(Reflect, Default, Component, Debug, Clone)]
pub struct IKRigRoot {
    pub(crate) targets: Vec<Entity>,
}

#[derive(Reflect, Component, Debug, Clone, Copy)]
#[require(Transform)]
pub struct SegmentDescription {
    pub segment_type: SegmentType,
    pub length_offset: f32,
    pub radius: f32,
}

#[derive(Reflect, Component, Debug, Default, Clone, Copy)]
pub struct SegmentState {
    pub(crate) last_position: Option<Vec2>,
    pub(crate) angle: f32,
}

#[derive(Reflect, Component, Debug, Clone, Copy)]
pub struct SegmentDistance {
    pub distance_from_root: f32,
    pub distance_threshold: f32,
}
