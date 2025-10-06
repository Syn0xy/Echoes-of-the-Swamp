use eots_skeleton_rig::{
    SegmentDescription,
    descriptions::{SegmentType, SkeletonRigDescription, SkeletonSegmentDescription},
};

use crate::creature::CreatureData;

pub(super) const DATA: CreatureData = CreatureData {
    // Description
    name: "Toad",

    // Stats
    health: 20,
    attack: 1,

    // Speed
    move_speed: 10.0,
    attack_speed: 2.0,

    // Range
    attack_range: 10.0,
    vision_range: 20.0,
};

pub(super) const SKELETON_RIG: SkeletonRigDescription = SkeletonRigDescription {
    segments: &[SkeletonSegmentDescription {
        segment: SegmentDescription {
            segment_type: SegmentType::Body,
            length_offset: 0.0,
            radius: 1.0,
        },
        childs: &[],
    }],
};
