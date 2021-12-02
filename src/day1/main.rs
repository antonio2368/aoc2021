use std::fs;
use std::io;

fn solution(lines: &[i32], window_size: usize) -> usize {
    lines
        .windows(window_size + 1)
        .filter(|value| value[window_size] > value[0])
        .count()
}

fn main() -> io::Result<()> {
    let file_content = fs::read_to_string("res/day1/input.txt")?;
    let lines: Vec<i32> = file_content
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    println!("Part1 result {}", solution(&lines, 1));
    println!("Part2 result {}", solution(&lines, 3));
    Ok(())
}
