use std::collections::HashMap;
use std::fs;
use std::ops::Fn;

fn solution<F>(positions: &HashMap<i32, i32>, cost_function: F) -> i32
where
    F: Fn(i32) -> i32,
{
    let min: i32 = *positions.keys().min().unwrap();
    let max: i32 = *positions.keys().max().unwrap();
    (min..max + 1)
        .map(|i| {
            positions.iter().fold(0, |acc, (position, count)| {
                let difference = (position - i).abs();
                let cost = cost_function(difference);
                acc + (cost * count)
            })
        })
        .min()
        .unwrap()
}

fn main() {
    let positions: HashMap<i32, i32> = fs::read_to_string("res/day7/input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|val| val.parse::<i32>().unwrap())
        .fold(HashMap::new(), |mut map, position| {
            *map.entry(position).or_insert(0) += 1;
            map
        });

    println!(
        "part1 result {}",
        solution(&positions, |difference| difference)
    );
    println!(
        "part2 result {}",
        solution(&positions, |difference| (difference * (difference + 1) / 2))
    );
}
