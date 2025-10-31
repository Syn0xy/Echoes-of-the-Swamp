use bevy::prelude::*;

#[derive(Reflect, Component, Debug, Clone)]
pub struct SkeletonRig {
    pub(crate) segment_chain_constraint: f32,
    pub(crate) main_segment: Option<Entity>,
    pub(crate) segments: Vec<Entity>,
    pub(crate) look_at_target: Option<Entity>,
}

#[derive(Reflect, Component, Debug, Clone, Copy)]
pub struct SkeletonRigEntity(pub Entity);

#[derive(Reflect, Component, Debug, Clone)]
pub struct SkeletonLookAtTarget {
    pub(crate) target: Entity,
}

#[derive(Reflect, Debug, Clone, Copy)]
pub enum SegmentType {
    Head,
    Body,
    Joint,
    Foot {
        distance_from_root: f32,
        distance_threshold: f32,
    },
    Tail,
}

#[derive(Reflect, Component, Debug, Clone, Copy)]
#[require(Transform)]
pub struct SegmentDescription {
    pub segment_type: SegmentType,
    pub length_offset: f32,
    pub radius: f32,
}

#[derive(Reflect, Component, Debug, Clone, Copy)]
pub struct SegmentDistance {
    pub distance_from_root: f32,
    pub distance_threshold: f32,
}

#[derive(Reflect, Component, Debug, Default, Clone, Copy)]
pub struct SegmentState {
    pub(crate) angle: f32,
}

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
