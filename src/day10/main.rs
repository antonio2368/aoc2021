use std::collections::HashMap;

fn is_close(c: char) -> bool {
    let closing_characters = ")]}>";
    closing_characters
        .chars()
        .position(|closing| closing == c)
        .is_some()
}

fn main() {
    let file_content = std::fs::read_to_string("res/day10/input.txt").unwrap();
    let lines: Vec<&str> = file_content.lines().collect();

    let corrupted_points: HashMap<char, u64> = {
        let mut map: HashMap<char, u64> = HashMap::new();
        map.insert(')', 3);
        map.insert(']', 57);
        map.insert('}', 1197);
        map.insert('>', 25137);
        map
    };

    let incomplete_points: HashMap<char, u64> = {
        let mut map: HashMap<char, u64> = HashMap::new();
        map.insert('(', 1);
        map.insert('[', 2);
        map.insert('{', 3);
        map.insert('<', 4);
        map
    };

    let bracket_pairs: HashMap<char, char> = {
        let mut map: HashMap<char, char> = HashMap::new();
        map.insert('(', ')');
        map.insert('[', ']');
        map.insert('{', '}');
        map.insert('<', '>');
        map
    };

    let (corrupted_points, mut incomplete_points): (u64, Vec<u64>) = lines
        .iter()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();
            for c in line.chars() {
                if is_close(c) {
                    if let Some(last_open) = stack.pop() {
                        if bracket_pairs[&last_open] != c {
                            return (corrupted_points[&c], 0);
                        }
                    }
                } else {
                    stack.push(c);
                }
            }

            let incomplete_points = stack
                .iter()
                .rev()
                .fold(0, |acc, value| acc * 5 + incomplete_points[&value]);
            (0, incomplete_points)
        })
        .fold(
            (0, Vec::new()),
            |(acc_corrupted, mut acc_incomplete), (corrupted, incomplete)| {
                if incomplete != 0 {
                    acc_incomplete.push(incomplete);
                }
                (acc_corrupted + corrupted, acc_incomplete)
            },
        );

    incomplete_points.sort();
    println!("part1 result {}", corrupted_points);
    println!(
        "part2 result {}",
        incomplete_points[incomplete_points.len() / 2]
    );
}
