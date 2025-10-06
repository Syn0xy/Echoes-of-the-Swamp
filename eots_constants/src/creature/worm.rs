use eots_skeleton_rig::{
    SegmentDescription,
    descriptions::{SegmentType, SkeletonRigDescription, SkeletonSegmentDescription},
};

use crate::creature::CreatureData;

pub(super) const DATA: CreatureData = CreatureData {
    // Description
    name: "Worm",

    // Stats
    health: 20,
    attack: 1,

    // Speed
    move_speed: 10.0,
    attack_speed: 2.0,

    // Range
    attack_range: 10.0,
    vision_range: 5.0,
};

pub(super) const SKELETON_RIG: SkeletonRigDescription = SkeletonRigDescription {
    segments: &[
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Head,
                length_offset: 1.0,
                radius: 1.0,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Tail,
                length_offset: 1.0,
                radius: 1.0,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Tail,
                length_offset: 1.0,
                radius: 1.0,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Tail,
                length_offset: 1.0,
                radius: 1.0,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Tail,
                length_offset: 1.0,
                radius: 1.0,
            },
            childs: &[],
        },
    ],
};
