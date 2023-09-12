use crate::{map, player, vector, util, WIDTH};

const FOV: f32 = 1.0;

pub fn render(map: &map::Map, player: &player::Player, buffer: &mut softbuffer::Buffer) {
    for i in 0..WIDTH {
        let uv: f32 = i as f32 / WIDTH as f32 * 2.0 - 1.0;

        let direction = vector::Vec2::new(uv * FOV, 1.0).normalize().rotate(player.rotation);

        let (position, wall) = util::raycast(map, &player.position, &direction);

        if wall != None {
            buffer[i as usize] = 0xff;
        }
    }
}
