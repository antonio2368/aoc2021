use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn move_position(mut pos: (i32, i32), movement: (i32, i32)) -> Option<(i32, i32)> {
    pos.0 += movement.0;
    if pos.0 < 0 || pos.0 == 10 {
        return None;
    }
    pos.1 += movement.1;
    if pos.1 < 0 || pos.1 == 10 {
        return None;
    }

    Some(pos)
}

fn main() {
    let mut energy_level: HashMap<(i32, i32), i32> = std::fs::read_to_string("res/day11/input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .fold(HashMap::new(), |map, (y, line)| {
            line.chars().enumerate().fold(map, |mut map, (x, c)| {
                map.insert((x as i32, y as i32), c.to_digit(10).unwrap() as i32);
                map
            })
        });

    let mut light_num = 0;
    let mut i = 1;
    loop {
        for value in energy_level.values_mut() {
            *value += 1;
        }

        loop {
            let mut current_light_num = 0;
            for i in 0..10 {
                for j in 0..10 {
                    if energy_level[&(i, j)] <= 9 {
                        continue;
                    }
                    for new_pos in [
                        (-1, -1),
                        (0, -1),
                        (1, -1),
                        (-1, 0),
                        (1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                    ]
                    .iter()
                    .filter_map(|&movement| move_position((i, j), movement))
                    {
                        match energy_level.entry(new_pos) {
                            Entry::Occupied(mut entry) => {
                                let energy = entry.get_mut();
                                if *energy != -1 {
                                    *energy += 1
                                }
                            }
                            Entry::Vacant(_) => panic!("Invalid movement"),
                        }
                    }
                    current_light_num += 1;
                    *energy_level.get_mut(&(i, j)).unwrap() = -1;
                }
            }

            if current_light_num == 0 {
                break;
            }

            light_num += current_light_num;
        }

        if energy_level.values().all(|value| value == &-1) {
            println!("All flashed at step {}", i);
            break;
        }

        for value in energy_level.values_mut().filter(|value| *value == &-1) {
            *value = 0;
        }
        if i == 100 {
            println!("Activations in step {}: {}", i, light_num);
        }
        i += 1;
    }
}
