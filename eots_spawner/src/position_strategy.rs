use bevy::math::Vec2;
use rand::Rng;



pub trait PositionStrategy: Send + Sync {
    fn generate_position(&self, origin: &Vec2, radius: f32) -> Vec2;
}

pub struct RandomPositionStrategy;

impl PositionStrategy for RandomPositionStrategy {
    fn generate_position(&self, origin: &Vec2, radius: f32) -> Vec2 {
        use std::f32::consts::PI;

        let mut rng = rand::rng();
        let r = rng.random_range(0.0_f32..1.0_f32).sqrt() * radius;
        let a = rng.random_range(0.0_f32..2.0_f32 * PI);

        Vec2::new(origin.x + r * a.cos(), origin.y + r * a.sin())
    }
}