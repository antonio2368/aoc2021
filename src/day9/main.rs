use std::collections::{HashSet, VecDeque};

fn up(location: (usize, usize), _heightmap: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
    if location.1 == 0 {
        return None;
    }

    Some((location.0, location.1 - 1))
}

fn down(location: (usize, usize), heightmap: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
    if location.1 == heightmap.len() - 1 {
        return None;
    }

    Some((location.0, location.1 + 1))
}

fn left(location: (usize, usize), _heightmap: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
    if location.0 == 0 {
        return None;
    }

    Some((location.0 - 1, location.1))
}

fn right(location: (usize, usize), heightmap: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
    assert!(heightmap.len() > 0);
    if location.0 == heightmap[0].len() - 1 {
        return None;
    }

    Some((location.0 + 1, location.1))
}

fn main() {
    let heightmap: Vec<Vec<i32>> = std::fs::read_to_string("res/day9/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let risk: i32 = heightmap
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, _val)| {
                    [up, down, left, right]
                        .iter()
                        .all(|f| match f((*j, i), &heightmap) {
                            Some((new_j, new_i)) => heightmap[i][*j] < heightmap[new_i][new_j],
                            None => true,
                        })
                })
                .map(|(_j, val)| val + 1)
                .sum::<i32>()
        })
        .sum();

    println!("Part1 result {}", risk);

    let mut basins: Vec<HashSet<(usize, usize)>> = Vec::new();

    for (i, row) in heightmap.iter().enumerate() {
        for (j, _val) in row.iter().enumerate().filter(|(_j, &val)| val != 9) {
            if basins.iter().filter(|set| set.contains(&(j, i))).count() != 0 {
                continue;
            }
            let mut basin: HashSet<(usize, usize)> = HashSet::new();
            basin.insert((j, i));

            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            queue.push_back((j, i));

            while !queue.is_empty() {
                let (j, i) = queue.pop_front().unwrap();
                basin.insert((j, i));

                for (new_j, new_i) in [up, down, left, right]
                    .iter()
                    .filter_map(|f| f((j, i), &heightmap))
                    .filter(|(new_j, new_i)| {
                        !basin.contains(&(*new_j, *new_i)) && heightmap[*new_i][*new_j] != 9
                    })
                {
                    queue.push_back((new_j, new_i));
                }
            }

            basins.push(basin);
        }
    }

    let mut lengths: Vec<usize> = basins.iter().map(|set| set.len()).collect();
    lengths.sort_by(|a, b| b.cmp(a));
    println!(
        "Part2 result {}",
        lengths
            .into_iter()
            .take(3)
            .reduce(|accum, val| (accum * val))
            .unwrap()
    );
}
