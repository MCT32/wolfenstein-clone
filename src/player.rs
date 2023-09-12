use crate::vector;

pub struct Player {
    pub position: vector::Vec2,
    pub rotation: f32,
}

impl Player {
    pub fn new(position: vector::Vec2, rotation: f32) -> Self {
        Self { position, rotation }
    }
}
