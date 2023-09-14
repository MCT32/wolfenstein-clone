use crate::{ map, vector };

pub fn raycast(map: &map::Map, start: &vector::Vec2, direction: &vector::Vec2) -> (vector::Vec2, Option<u8>) {
    // x
    let mut current = start.clone();
    let dy = direction.y / direction.x.abs();

    if direction.x > 0.0 {
        let dx = current.x.ceil() - current.x;

        current.x = current.x.ceil();
        current.y += dx * dy;
    } else {
        let dx = current.x - current.x.floor();

        current.x = current.x.floor();
        current.y += dx * dy;
    }

    let x_result = loop {
        let wall = map.get(current.x as usize, current.y.floor() as usize);

        if wall.is_err() {
            break (current, None);
        }

        let wall = wall.unwrap();

        if wall != 0 {
            break (current, Some(wall));
        }
        
        if direction.x > 0.0 {
            current.x += 1.0;
        } else {
            current.x -= 1.0;
        }
        current.y += dy;
    };

    // y
    let mut current = start.clone();
    let dx = direction.x / direction.y.abs();

    if direction.y > 0.0 {
        let dy = current.y.ceil() - current.y;

        current.y = current.y.ceil();
        current.x += dy * dx;
    } else {
        let dy = current.y - current.y.floor();

        current.y = current.y.floor();
        current.x += dy * dx;
    }

    let y_result = loop {
        let wall = map.get(current.x.floor() as usize, current.y as usize);

        if wall.is_err() {
            break (current, None);
        }

        let wall = wall.unwrap();

        if wall != 0 {
            break (current, Some(wall));
        }
        
        if direction.y > 0.0 {
            current.y += 1.0;
        } else {
            current.y -= 1.0;
        }
        current.x += dx;
    };

    if (x_result.0 - *start).length() < (y_result.0 - *start).length() {
        x_result
    } else {
        y_result
    }
}
