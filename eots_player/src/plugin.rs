use bevy::prelude::*;
use eots_constants::player;
use eots_creature::components::CreatureStats;

use crate::{Player, SpawnPlayerEvent, spawn};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        // .register_type::<Player>()
            .add_message::<SpawnPlayerEvent>()
            .add_systems(Update, (spawn::spawn_player, handle_inputs).chain());
    }
}

fn handle_inputs(
    mut player_transform: Query<(&mut Transform, &CreatureStats), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) -> Result {
    let (mut transform, stats) = player_transform.single_mut()?;
    let mut direction = IVec2::default();

    for key in keys.get_pressed() {
        match key {
            KeyCode::KeyD => direction.x = 1,
            KeyCode::KeyA => direction.x = -1,
            KeyCode::KeyW => direction.y = 1,
            KeyCode::KeyS => direction.y = -1,
            _ => {}
        }
    }

    let delta_time = time.delta_secs();
    let direction_vec2 = direction.as_vec2();
    let add_speed = direction_vec2.normalize_or_zero()
        * stats.move_speed
        * player::PLAYER_MUL_MOVE_SPEED
        * delta_time;

    transform.translation.x += add_speed.x;
    transform.translation.y += add_speed.y;

    Ok(())
}
