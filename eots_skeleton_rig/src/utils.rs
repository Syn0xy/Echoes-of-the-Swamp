use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

pub(crate) fn chain_segments(
    transform: &mut Transform,
    parent_position: Vec2,
    parent_length_offset: f32,
) -> Vec2 {
    let segment_pos = transform.translation.truncate();
    let offset = (segment_pos - parent_position).normalize_or_zero() * parent_length_offset;
    let target_pos = parent_position + offset;

    transform.translation.x = target_pos.x;
    transform.translation.y = target_pos.y;

    target_pos
}

pub(crate) fn angle_between(a: &Vec2, b: &Vec2) -> f32 {
    (b.y - a.y).atan2(b.x - a.x)
}

pub(crate) fn angle_diff_signed(a1: f32, a2: f32) -> f32 {
    (a2 - a1 + PI).rem_euclid(TAU) - PI
}
