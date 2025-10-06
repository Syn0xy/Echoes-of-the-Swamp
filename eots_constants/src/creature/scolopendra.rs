use eots_skeleton_rig::{
    SegmentDescription,
    descriptions::{SegmentType, SkeletonRigDescription, SkeletonSegmentDescription},
};

use crate::creature::CreatureData;

pub(super) const DATA: CreatureData = CreatureData {
    // Description
    name: "Scolopendra",

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
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
        ONE_LEGS_PART,
    ],
};

const ONE_LEGS_PART: SkeletonSegmentDescription = SkeletonSegmentDescription {
    segment: SegmentDescription {
        segment_type: SegmentType::Body,
        length_offset: 1.75,
        radius: 1.0,
    },
    childs: &[ONE_LEG_PART, ONE_LEG_PART],
};

const ONE_LEG_PART: SkeletonSegmentDescription = SkeletonSegmentDescription {
    segment: SegmentDescription {
        segment_type: SegmentType::Joint,
        length_offset: 1.0,
        radius: 0.35,
    },
    childs: &[SkeletonSegmentDescription {
        segment: SegmentDescription {
            segment_type: SegmentType::Joint,
            length_offset: 1.0,
            radius: 0.3,
        },
        childs: &[SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Joint,
                length_offset: 1.0,
                radius: 0.25,
            },
            childs: &[SkeletonSegmentDescription {
                segment: SegmentDescription {
                    segment_type: SegmentType::Foot {
                        distance_from_root: 2.0,
                        distance_threshold: 1.5,
                    },
                    length_offset: 1.0,
                    radius: 0.3,
                },
                childs: &[],
            }],
        }],
    }],
};
