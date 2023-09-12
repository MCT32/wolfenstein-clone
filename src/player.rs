use crate::vector;
use std::time::Duration;

pub struct Player {
    pub position: vector::Vec2,
    pub rotation: f32,

    velocity: vector::Vec2,
}

impl Player {
    pub fn new(position: vector::Vec2, rotation: f32, velocity: vector::Vec2) -> Self {
        Self {
            position,
            rotation,
            velocity,
        }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.position += self.velocity * delta_time.as_secs_f32();
    }

    pub fn set_velocity(&mut self, velocity: vector::Vec2) {
        self.velocity = velocity;
    }
}
