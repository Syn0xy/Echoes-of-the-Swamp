use bevy::prelude::*;

use crate::{IKRigRoot, IKRigTarget, SegmentDescription, SegmentDistance, SegmentState, utils};

pub(crate) fn update_ik_targets(
    mut transform_q: Query<&mut Transform>,
    ik_root_q: Query<(Entity, &IKRigRoot, &SegmentState)>,
    segment_distance_q: Query<&SegmentDistance>,
) {
    for (root_entity, IKRigRoot { targets }, SegmentState { angle, .. }) in &ik_root_q {
        let Ok(root_transform) = transform_q.get(root_entity) else {
            continue;
        };

        let root_position = root_transform.translation;
        let (group1, group2) = targets.split_at(targets.len() / 2);

        for (group, base_angle) in [(group1, *angle), (group2, 180.0 - *angle)] {
            let spread = 180.0 / (group.len() + 1) as f32;

            for (i, &target) in group.iter().enumerate() {
                let Ok(mut ik_trfm) = transform_q.get_mut(target) else {
                    continue;
                };

                let Ok(SegmentDistance {
                    distance_from_root,
                    distance_threshold,
                }) = segment_distance_q.get(target)
                else {
                    continue;
                };

                let rel_angle = (base_angle + spread * (i as f32 + 1.0)).to_radians();

                let ik_pos = ik_trfm.translation.truncate();
                let target_pos = Vec2::new(
                    root_position.x + rel_angle.sin() * distance_from_root,
                    root_position.y + rel_angle.cos() * distance_from_root,
                );

                if ik_pos.distance_squared(target_pos) > distance_threshold * distance_threshold {
                    let target_pos_with_movement_offset =
                        target_pos - (ik_pos - target_pos).normalize() * 1.0;
                    ik_trfm.translation.x = target_pos_with_movement_offset.x;
                    ik_trfm.translation.y = target_pos_with_movement_offset.y;
                }
            }
        }
    }
}

pub(crate) fn update_ik_chains(
    mut transform_q: Query<&mut Transform>,
    ik_target_q: Query<(Entity, &IKRigTarget)>,
    segment_q: Query<&SegmentDescription>,
) {
    for (
        ik_target_entity,
        IKRigTarget {
            root_segment,
            segments,
        },
    ) in &ik_target_q
    {
        let Ok(ik_target_transform) = transform_q.get(ik_target_entity) else {
            continue;
        };

        let mut target_pos = ik_target_transform.translation.truncate();
        let mut length_offset = 0.0;

        for segment_entity in segments.iter().rev() {
            if let (Ok(mut seg_transform), Ok(seg_desc)) = (
                transform_q.get_mut(*segment_entity),
                segment_q.get(*segment_entity),
            ) {
                target_pos = utils::chain_segments(&mut seg_transform, target_pos, length_offset);
                length_offset = seg_desc.length_offset;
            }
        }

        if let Ok(root_transform) = transform_q.get(*root_segment) {
            target_pos.x = root_transform.translation.x;
            target_pos.y = root_transform.translation.y;
        }

        for segment_entity in segments.iter() {
            if let (Ok(mut seg_transform), Ok(seg_desc)) = (
                transform_q.get_mut(*segment_entity),
                segment_q.get(*segment_entity),
            ) {
                length_offset = seg_desc.length_offset;
                target_pos = utils::chain_segments(&mut seg_transform, target_pos, length_offset);
            }
        }
    }
}
