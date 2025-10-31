use bevy::prelude::*;

use crate::{
    IKRigRoot, IKRigTarget, SegmentDescription, SegmentDistance, SegmentState, SegmentType,
    SkeletonLookAtTarget, SkeletonRig, SkeletonRigEntity, debug, inverse_kinematic, utils,
};

pub struct SkeletonRigPlugin;

impl Plugin for SkeletonRigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SkeletonRig>()
            .register_type::<SkeletonRigEntity>()
            .register_type::<SegmentType>()
            .register_type::<SegmentDescription>()
            .register_type::<SegmentDistance>()
            .register_type::<SegmentState>()
            .register_type::<IKRigTarget>()
            .register_type::<IKRigRoot>()
            .add_systems(
                Update,
                (
                    update_main_segment,
                    // update_main_segment_direction,
                    update_segment_chains,
                    update_segment_constraints,
                    update_segment_states,
                    inverse_kinematic::update_ik_targets,
                    inverse_kinematic::update_ik_chains,
                )
                    .chain(),
            )
            .add_systems(PostUpdate, debug::draw_segment_gizmos);
    }
}

// fn update_main_segment_direction(
//     mut transform_q: Query<&mut Transform>,
//     // mut segment_state_q: Query<&mut SegmentState>,
//     skeleton_rig_q: Query<&SkeletonRig>,
//     look_at_target_q: Query<(Entity, &SkeletonLookAtTarget)>,
// ) {
//     for (look_at_entity, look_at) in look_at_target_q {
//         let Ok(look_at_t) = transform_q.get(look_at_entity) else {
//             continue;
//         };

//         let _look_at_pos = look_at_t.translation.truncate();

//         let Ok(skeleton_rig) = skeleton_rig_q.get(look_at.target) else {
//             continue;
//         };

//         let Some([main_seg, seg]) = skeleton_rig.segments.windows(2).next() else {
//             continue;
//         };

//         let Ok(main_seg_t) = transform_q.get(*main_seg) else {
//             continue;
//         };

//         let _main_seg_pos = main_seg_t.translation.truncate();

//         let Ok(seg_t) = transform_q.get_mut(*seg) else {
//             continue;
//         };

//         let _seg_pos = seg_t.translation.truncate();
//     }
// }

fn update_main_segment(
    mut transform_q: Query<(&mut Transform, &mut SegmentState), Without<SkeletonRigEntity>>,
    skeleton_rig_q: Query<&SkeletonRig>,
    skeleton_q: Query<(&Transform, &SkeletonRigEntity), Changed<Transform>>,
) {
    for (target_transform, &SkeletonRigEntity(rig_entity)) in &skeleton_q {
        let Ok(skeleton_rig) = skeleton_rig_q.get(rig_entity) else {
            continue;
        };

        let Some(main_segment) = skeleton_rig.main_segment else {
            continue;
        };

        if let Ok((mut segment_transform, mut segment_state)) = transform_q.get_mut(main_segment) {
            let before_position = segment_transform.translation.truncate();
            let after_position = target_transform.translation.truncate();

            if before_position == after_position {
                continue;
            }

            segment_transform.translation.x = target_transform.translation.x;
            segment_transform.translation.y = target_transform.translation.y;

            segment_state.angle = utils::angle_between(&before_position, &after_position);
        }
    }
}

fn update_segment_chains(
    mut transform_q: Query<&mut Transform>,
    skeleton_rig_q: Query<&SkeletonRig>,
    segment_desc_q: Query<&SegmentDescription>,
) {
    for SkeletonRig { segments, .. } in &skeleton_rig_q {
        for window in segments.windows(2) {
            let &[prev_segment, crnt_segment] = window else {
                continue;
            };

            let (Ok(prev_transform), Ok(prev_desc)) = (
                transform_q.get(prev_segment),
                segment_desc_q.get(prev_segment),
            ) else {
                continue;
            };

            let prev_position = prev_transform.translation.truncate();

            if let Ok(mut crnt_transform) = transform_q.get_mut(crnt_segment) {
                utils::chain_segments(&mut crnt_transform, prev_position, prev_desc.length_offset);
            }
        }
    }
}

fn update_segment_states(
    transform_q: Query<&Transform>,
    skeleton_rig_q: Query<&SkeletonRig>,
    mut segment_state_q: Query<&mut SegmentState>,
) {
    for SkeletonRig { segments, .. } in &skeleton_rig_q {
        for window in segments.windows(2) {
            let &[later, crnt] = window else {
                continue;
            };

            let Ok(mut segment_state) = segment_state_q.get_mut(crnt) else {
                continue;
            };

            let (Ok(later_transform), Ok(crnt_transform)) =
                (transform_q.get(later), transform_q.get(crnt))
            else {
                continue;
            };

            let later_position = later_transform.translation.truncate();
            let crnt_position = crnt_transform.translation.truncate();

            if later_position != crnt_position {
                segment_state.angle = utils::angle_between(&crnt_position, &later_position);
            }
        }
    }
}

fn update_segment_constraints(
    mut transform_q: Query<&mut Transform>,
    skeleton_rig_q: Query<&SkeletonRig>,
) {
    for skeleton_rig in &skeleton_rig_q {
        for window in skeleton_rig.segments.windows(3) {
            let [sa, sb, sc] = window else {
                continue;
            };

            let (Ok(ta), Ok(tb)) = (transform_q.get(*sa), transform_q.get(*sb)) else {
                continue;
            };

            let pa = ta.translation.truncate();
            let pb = tb.translation.truncate();

            let Ok(mut tc) = transform_q.get_mut(*sc) else {
                continue;
            };

            let pc = tc.translation.truncate();

            let angle_ba = utils::angle_between(&pb, &pa);
            let angle_bc = utils::angle_between(&pb, &pc);
            let angle_diff = utils::angle_diff_signed(angle_ba, angle_bc);

            let angle_correction = skeleton_rig.segment_chain_constraint - angle_diff.abs();

            if angle_correction > 0.0 {
                let distance = pb.distance(pc);
                let target_angle = angle_bc + angle_correction * angle_diff.signum();

                tc.translation.x = pb.x + distance * target_angle.cos();
                tc.translation.y = pb.y + distance * target_angle.sin();
            }
        }
    }
}
