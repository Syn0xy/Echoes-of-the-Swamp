use bevy::prelude::*;

use crate::SegmentDescription;

#[derive(Debug, Clone, Copy)]
pub struct SkeletonRigDescription {
    pub segments: &'static [SkeletonSegmentDescription],
}

#[derive(Debug, Clone, Copy)]
pub struct SkeletonSegmentDescription {
    pub segment: SegmentDescription,
    pub childs: &'static [SkeletonSegmentDescription],
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
