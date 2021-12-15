use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Eq, PartialEq)]
struct State {
    distance: i64,
    coordinate: (i64, i64),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.coordinate.cmp(&other.coordinate))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn move_position(
    mut pos: (i64, i64),
    movement: (i64, i64),
    dimension: (i64, i64),
) -> Option<(i64, i64)> {
    pos.0 += movement.0;
    if pos.0 < 0 || pos.0 == dimension.0 {
        return None;
    }
    pos.1 += movement.1;
    if pos.1 < 0 || pos.1 == dimension.1 {
        return None;
    }

    Some(pos)
}

fn wrap_result(result: i64) -> i64 {
    (result - 1) % 9 + 1
}

fn shortest_path(
    start: (i64, i64),
    base_dimension: (i64, i64),
    mul: i64,
    weights: &HashMap<(i64, i64), i64>,
) -> Option<i64> {
    let grid_dimension = (mul * base_dimension.0, mul * base_dimension.1);
    let mut dist: HashMap<(i64, i64), i64> = HashMap::new();

    dist.insert(start, 0);
    let mut heap = BinaryHeap::<State>::new();
    heap.push(State {
        distance: 0,
        coordinate: start,
    });

    let goal = (grid_dimension.0 - 1, grid_dimension.1 - 1);
    while let Some(state) = heap.pop() {
        if state.coordinate == goal {
            return Some(state.distance);
        }

        for next_node in [(0, -1), (-1, 0), (0, 1), (1, 0)]
            .iter()
            .filter_map(|movement| move_position(state.coordinate, *movement, grid_dimension))
        {
            let weight_coordinate = (
                next_node.0 % base_dimension.0,
                next_node.1 % base_dimension.1,
            );
            let addition = (
                next_node.0 / base_dimension.0,
                next_node.1 / base_dimension.1,
            );
            let weight = weights[&weight_coordinate];
            let distance = state.distance + wrap_result(weight + addition.0 + addition.1);
            match dist.get_mut(&next_node) {
                Some(value) => {
                    if distance < *value {
                        *value = distance;
                        heap.push(State {
                            distance,
                            coordinate: next_node,
                        });
                    }
                }
                None => {
                    dist.insert(next_node, distance);
                    heap.push(State {
                        distance,
                        coordinate: next_node,
                    });
                }
            }
        }
    }
    None
}

fn main() {
    let file_content = std::fs::read_to_string("res/day15/input.txt").unwrap();
    let weights: HashMap<(i64, i64), i64> =
        file_content
            .lines()
            .enumerate()
            .fold(HashMap::new(), |map, (i, line)| {
                line.chars().enumerate().fold(map, |mut map, (j, c)| {
                    map.insert((j as i64, i as i64), c.to_digit(10).unwrap() as i64);
                    map
                })
            });

    let (max_x, max_y) = weights.keys().fold((0, 0), |(max_x, max_y), &(x, y)| {
        (max_x.max(x), max_y.max(y))
    });

    println!(
        "Part 1 result {}",
        shortest_path((0, 0), (max_x + 1, max_y + 1), 1, &weights).unwrap()
    );

    println!(
        "Part 2 result {}",
        shortest_path((0, 0), (max_x + 1, max_y + 1), 5, &weights).unwrap()
    );
}
