use crate::{ map, vector };

const STEP_SIZE: f32 = 0.01;
const STEP_LIMIT: u32 = 500;

pub fn raycast(map: &map::Map, start: &vector::Vec2, direction: &vector::Vec2) -> (vector::Vec2, Option<u8>) {
    let mut current = start.clone();

    let mut steps = 0;

    'step: loop {
        let x = current.x.floor() as i32;
        let y = current.y.floor() as i32;

        if x < 0 || x >= map.width as i32 || y < 0 || y >= map.height as i32 { break 'step (current, None); }

        let wall = map.get(x as usize, y as usize).unwrap();

        if wall != 0 {
            break (current, Some(wall));
        }

        current += *direction * STEP_SIZE;

        if steps == STEP_LIMIT { break (current, None); }
        steps += 1;
    }
}
