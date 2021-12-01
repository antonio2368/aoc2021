use std::fs;
use std::io;

fn part1(lines: &[i32]) -> usize {
    lines.windows(2).filter(|value| value[1] > value[0]).count()
}

fn part2(lines: &[i32]) -> i32 {
    let mut current_sum: i32 = lines.iter().take(3).sum();
    lines
        .iter()
        .zip(lines.iter().skip(3))
        .fold(0, |mut count, (first, second)| {
            let next_sum = current_sum - first + second;

            if next_sum > current_sum {
                count += 1;
            }

            current_sum = next_sum;

            return count;
        })
}

fn main() -> io::Result<()> {
    let file_content = fs::read_to_string("res/day1/input.txt")?;
    let lines: Vec<i32> = file_content
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    println!("Part1 result {}", part1(&lines));
    println!("Part2 result {}", part2(&lines));
    Ok(())
}
