use eots_skeleton_rig::{
    SegmentDescription,
    descriptions::{SegmentType, SkeletonRigDescription, SkeletonSegmentDescription},
};

use crate::creature::CreatureData;

const LENGTH_OFFSET: f32 = 1.0;

pub(super) const DATA: CreatureData = CreatureData {
    // Description
    name: "Salamander",

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
    segments: &[
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Head,
                length_offset: LENGTH_OFFSET,
                radius: 0.55,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Head,
                length_offset: LENGTH_OFFSET,
                radius: 0.65,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Joint,
                length_offset: LENGTH_OFFSET,
                radius: 0.25,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Body,
                length_offset: LENGTH_OFFSET,
                radius: 0.4,
            },
            childs: &[ONE_LEG, ONE_LEG],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Body,
                length_offset: LENGTH_OFFSET,
                radius: 0.5,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Body,
                length_offset: LENGTH_OFFSET,
                radius: 0.5,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Body,
                length_offset: LENGTH_OFFSET,
                radius: 0.4,
            },
            childs: &[ONE_LEG, ONE_LEG],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Tail,
                length_offset: LENGTH_OFFSET,
                radius: 0.4,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Tail,
                length_offset: LENGTH_OFFSET,
                radius: 0.3,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Tail,
                length_offset: LENGTH_OFFSET,
                radius: 0.2,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Tail,
                length_offset: LENGTH_OFFSET,
                radius: 0.15,
            },
            childs: &[],
        },
        SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Tail,
                length_offset: LENGTH_OFFSET,
                radius: 0.1,
            },
            childs: &[],
        },
    ],
};

const ONE_LEG: SkeletonSegmentDescription = SkeletonSegmentDescription {
    segment: SegmentDescription {
        segment_type: SegmentType::Joint,
        length_offset: 0.4,
        radius: 0.1,
    },
    childs: &[SkeletonSegmentDescription {
        segment: SegmentDescription {
            segment_type: SegmentType::Joint,
            length_offset: LENGTH_OFFSET,
            radius: 0.1,
        },
        childs: &[SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Foot {
                    distance_from_root: 2.0,
                    distance_threshold: 2.5,
                },
                length_offset: LENGTH_OFFSET,
                radius: 0.2,
            },
            childs: &[],
        }],
    }],
};
