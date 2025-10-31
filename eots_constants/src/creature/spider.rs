use eots_skeleton_rig::{
    SegmentDescription, SegmentType, SkeletonRigDescription, SkeletonSegmentDescription,
};

use crate::creature::CreatureData;

pub(super) const DATA: CreatureData = CreatureData {
    // Description
    name: "Spider",

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
    segment_chain_constraint: 2.0,
    segments: &[SkeletonSegmentDescription {
        segment: SegmentDescription {
            segment_type: SegmentType::Body,
            length_offset: 0.0,
            radius: 1.0,
        },
        childs: &[
            ONE_LEG_PART,
            ONE_LEG_PART,
            ONE_LEG_PART,
            ONE_LEG_PART,
            ONE_LEG_PART,
            ONE_LEG_PART,
            ONE_LEG_PART,
            ONE_LEG_PART,
        ],
    }],
};

const ONE_LEG_PART: SkeletonSegmentDescription = SkeletonSegmentDescription {
    segment: SegmentDescription {
        segment_type: SegmentType::Joint,
        length_offset: 1.0,
        radius: 0.25,
    },
    childs: &[SkeletonSegmentDescription {
        segment: SegmentDescription {
            segment_type: SegmentType::Joint,
            length_offset: 1.0,
            radius: 0.25,
        },
        childs: &[SkeletonSegmentDescription {
            segment: SegmentDescription {
                segment_type: SegmentType::Joint,
                length_offset: 1.0,
                radius: 0.25,
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
                            distance_from_root: 4.0,
                            distance_threshold: 1.0,
                        },
                        length_offset: 1.0,
                        radius: 0.25,
                    },
                    childs: &[],
                }],
            }],
        }],
    }],
};
