use std::fs;
use std::io;

fn value_counters(content: &String) -> Vec<i32> {
    let lines: Vec<&str> = content.lines().collect();
    assert!(lines.len() > 0);
    let line_size = lines.first().unwrap().len();
    let mut counters = vec![0; line_size];

    for line in content.lines() {
        for (counter, character) in counters.iter_mut().zip(line.chars()) {
            if character == '1' {
                *counter += 1;
            } else {
                *counter -= 1;
            }
        }
    }
    counters
}

fn number_from_counters(counters: &[i32]) -> i32 {
    let mut number = 0;
    for (counter, index) in counters.iter().rev().zip(0..counters.len()) {
        let bit = counter >= &0;
        number |= (bit as i32) << index;
    }
    number
}

fn part1(counters: &[i32]) -> i32 {
    let gamma = number_from_counters(counters);
    gamma * (!gamma & ((1 << counters.len()) - 1))
}

fn part2(content: &String, mut counters: Vec<i32>, most_common: bool) -> i32 {
    let mut current_lines: Vec<&str> = content.lines().collect();
    for i in 0..counters.len() {
        if current_lines.len() == 1 {
            break;
        }
        let value = if most_common && counters[i] >= 0 || !most_common && counters[i] < 0 {
            '1'
        } else {
            '0'
        };
        let mut new_current_lines = Vec::<&str>::new();
        for line in current_lines {
            if line.chars().nth(i).unwrap() == value {
                new_current_lines.push(line);
            } else {
                for (character, counter) in line.chars().skip(i).zip(counters.iter_mut().skip(i)) {
                    if character == '0' {
                        *counter += 1;
                    } else {
                        *counter -= 1;
                    }
                }
            }
        }

        current_lines = new_current_lines;
    }

    assert!(current_lines.len() == 1);
    let mut number = 0;
    for (character, index) in current_lines
        .first()
        .unwrap()
        .chars()
        .rev()
        .zip(0..counters.len())
    {
        let bit = character == '1';
        number |= (bit as i32) << index;
    }

    number
}

fn main() -> io::Result<()> {
    let file_content = fs::read_to_string("res/day3/input.txt")?;
    let counters = value_counters(&file_content);

    println!("Part1 result {}", part1(&counters));
    let oxy = part2(&file_content, counters.clone(), true);
    let co2 = part2(&file_content, counters, false);
    println!("Part2 result {}", oxy * co2);
    Ok(())
}
