use bevy::prelude::*;

use crate::{
    IKRigRoot, IKRigTarget, SegmentDistance, SegmentState, SkeletonRig,
    descriptions::{SegmentType, SkeletonRigDescription, SkeletonSegmentDescription},
};

pub fn build_skeleton_rig(commands: &mut Commands, description: &SkeletonRigDescription) -> Entity {
    let root = commands
        .spawn((
            Name::new("Skeleton Rig"),
            Transform::default(),
            GlobalTransform::default(),
        ))
        .id();

    let mut segments = Vec::default();

    for segment_desc in description.segments {
        let mut ik_root = None;
        let entity = spawn_segment_recursive(commands, root, segment_desc, &mut ik_root, &mut None);

        segments.push(entity);

        if let Some(ik_root) = ik_root {
            commands.entity(entity).insert(ik_root);
        }
    }

    commands.entity(root).insert(SkeletonRig {
        main_segment: segments.first().copied(),
        segments,
    });

    root
}

fn spawn_segment_recursive(
    commands: &mut Commands,
    skeleton_root: Entity,
    description: &SkeletonSegmentDescription,
    ik_root: &mut Option<IKRigRoot>,
    ik_target: &mut Option<IKRigTarget>,
) -> Entity {
    let entity = commands
        .spawn((
            description.segment,
            SegmentState::default(),
            ChildOf(skeleton_root),
        ))
        .id();

    if let Some(target) = ik_target {
        target.segments.push(entity);

        if let SegmentType::Foot {
            distance_from_root,
            distance_threshold,
        } = description.segment.segment_type
        {
            let target_entity = commands
                .spawn((
                    target.clone(),
                    SegmentDistance {
                        distance_from_root,
                        distance_threshold,
                    },
                    ChildOf(skeleton_root),
                ))
                .id();

            target.segments.clear();

            if let Some(IKRigRoot { targets }) = ik_root {
                targets.push(target_entity);
            }
        }
    } else if !description.childs.is_empty() {
        *ik_root = Some(IKRigRoot::default());
        *ik_target = Some(IKRigTarget {
            root_segment: entity,
            segments: Vec::default(),
        });
    }

    for segment_child in description.childs {
        spawn_segment_recursive(commands, skeleton_root, segment_child, ik_root, ik_target);
    }

    entity
}
