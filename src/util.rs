use crate::{ map, vector };

pub fn raycast(map: &map::Map, start: &vector::Vec2, direction: &vector::Vec2) -> (vector::Vec2, Option<u8>) {
    // x
    let mut current = start.clone();
    let dy = direction.y / direction.x.abs();

    if direction.x < 0.0 { println!("Dir: {direction}, Current: {current}, dy: {dy}"); }

    if direction.x > 0.0 {
        let dx = current.x.ceil() - current.x;

        current.x = current.x.ceil();
        current.y += dx * dy;
    } else {
        let dx = current.x.floor() - current.x;

        current.x = current.x.floor();
        current.y += dx * dy;
    }

    let mut step = 0;

    let x_result = loop {
        let wall = map.get(current.x as usize, current.y.floor() as usize);

        if wall.is_err() {
            break (current, None);
        }

        let wall = wall.unwrap();

        if wall != 0 {
            break (current, Some(wall));
        }
        
        if direction.x < 0.0 { println!("Step: {step}, Current: {current}, dy: {dy}"); }

        if direction.x > 0.0 {
            current.x += 1.0;
        } else {
            current.x -= 1.0;
        }
        current.y += dy;

        step += 1;
    };

    x_result

    /*
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
    */
}
