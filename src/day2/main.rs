use std::fs;
use std::io;

enum Direction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn part1(directions: &[Direction]) -> i32 {
    let (horizontal, vertical) = directions.iter().fold(
        (0, 0),
        |(horizontal, vertical), directions| match directions {
            Direction::Up(value) => (horizontal, vertical - value),
            Direction::Down(value) => (horizontal, vertical + value),
            Direction::Forward(value) => (horizontal + value, vertical),
        },
    );

    horizontal * vertical
}

fn part2(directions: &[Direction]) -> i32 {
    let (horizontal, vertical, _aim) = directions.iter().fold(
        (0, 0, 0),
        |(horizontal, vertical, aim), directions| match directions {
            Direction::Up(value) => (horizontal, vertical, aim - value),
            Direction::Down(value) => (horizontal, vertical, aim + value),
            Direction::Forward(value) => (horizontal + value, vertical + aim * value, aim),
        },
    );

    horizontal * vertical
}

fn main() -> io::Result<()> {
    let file_content = fs::read_to_string("res/day2/input.txt")?;
    let directions: Vec<Direction> = file_content
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace();
            let direction_string = words.next().unwrap();
            let value = words.next().unwrap().parse::<i32>().unwrap();
            assert_eq!(None, words.next());
            match direction_string {
                "up" => Direction::Up(value),
                "down" => Direction::Down(value),
                "forward" => Direction::Forward(value),
                other => panic!("Invalid value for direction {}", other),
            }
        })
        .collect();

    println!("Part1 result {}", part1(&directions));
    println!("Part2 result {}", part2(&directions));

    Ok(())
}
