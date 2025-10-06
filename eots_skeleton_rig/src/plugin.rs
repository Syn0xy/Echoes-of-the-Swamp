use bevy::prelude::*;

use crate::{
    IKRigRoot, IKRigTarget, SegmentDescription, SegmentDistance, SegmentState, SkeletonRig,
    SkeletonRigEntity, debug, descriptions::SegmentType, inverse_kinematic, utils,
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
                    update_segment_chains,
                    update_segment_states,
                    inverse_kinematic::update_ik_targets,
                    inverse_kinematic::update_ik_chains,
                )
                    .chain(),
            )
            .add_systems(PostUpdate, debug::draw_segment_gizmos);
    }
}

fn update_main_segment(
    mut transform_q: Query<&mut Transform, Without<SkeletonRigEntity>>,
    skeleton_rig_q: Query<&SkeletonRig>,
    skeleton_q: Query<(&Transform, &SkeletonRigEntity), Changed<Transform>>,
) {
    for (target_transform, &SkeletonRigEntity(rig_entity)) in &skeleton_q {
        let Some(main_segment) = skeleton_rig_q
            .get(rig_entity)
            .ok()
            .and_then(|skeleton_rig| skeleton_rig.main_segment)
        else {
            continue;
        };

        if let Ok(mut segment_transform) = transform_q.get_mut(main_segment) {
            segment_transform.translation.x = target_transform.translation.x;
            segment_transform.translation.y = target_transform.translation.y;
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

            let Ok(mut crnt_transform) = transform_q.get_mut(crnt_segment) else {
                continue;
            };

            utils::chain_segments(&mut crnt_transform, prev_position, prev_desc.length_offset);
        }
    }
}

fn update_segment_states(
    mut segment_state_q: Query<(&Transform, &mut SegmentState), Changed<Transform>>,
) {
    for (transform, mut segment_state) in &mut segment_state_q {
        let crnt_position = transform.translation.truncate();
        let last_position = segment_state.last_position.get_or_insert(crnt_position);

        if crnt_position != *last_position {
            let angle = utils::angle_between(last_position, &crnt_position);
            *last_position = crnt_position;
            segment_state.angle = angle;
        }
    }
}
