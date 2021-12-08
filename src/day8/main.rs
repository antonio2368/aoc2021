use std::collections::{HashMap, HashSet};
use std::fs;

struct Line<'a> {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
}

fn get_hashset_for_segments(segments: &str) -> HashSet<char> {
    segments.chars().collect::<HashSet<char>>()
}

fn main() {
    let file_content = fs::read_to_string("res/day8/input.txt").unwrap();

    let lines: Vec<Line> = file_content
        .lines()
        .map(|line| {
            let input_outputs: Vec<&str> = line.trim().split(" | ").collect();
            Line {
                input: input_outputs[0].split_whitespace().collect(),
                output: input_outputs[1].split_whitespace().collect(),
            }
        })
        .collect();

    let unique_digit_count: usize = lines
        .iter()
        .map(|line| {
            line.output
                .iter()
                .filter(|out| [2, 4, 3, 7].iter().any(|&len| len == out.len()))
                .count()
        })
        .sum();

    let number_map: Vec<HashSet<char>> = vec![
        get_hashset_for_segments("abcefg"),
        get_hashset_for_segments("cf"),
        get_hashset_for_segments("acdeg"),
        get_hashset_for_segments("acdfg"),
        get_hashset_for_segments("bcdf"),
        get_hashset_for_segments("abdfg"),
        get_hashset_for_segments("abdefg"),
        get_hashset_for_segments("acf"),
        get_hashset_for_segments("abcdefg"),
        get_hashset_for_segments("abcdfg"),
    ];

    let decoded_segment_count: HashMap<char, i32> =
        number_map
            .iter()
            .fold(HashMap::new(), |mut map, segment_set| {
                for &c in segment_set {
                    *map.entry(c).or_insert(0) += 1;
                }
                map
            });

    println!("part1 result {}", unique_digit_count);

    let mut sum: usize = 0;
    for line in &lines {
        let segment_count: HashMap<char, i32> =
            line.input.iter().fold(HashMap::new(), |mut map, inp| {
                for c in inp.chars() {
                    *map.entry(c).or_insert(0) += 1;
                }
                map
            });

        let mut mapping: HashMap<char, char> = HashMap::new();

        let map_based_on_count = |map: &mut HashMap<char, char>,
                                  first_input,
                                  second_input,
                                  first_character,
                                  second_character| {
            if segment_count.get(&first_input).unwrap()
                == decoded_segment_count.get(&first_character).unwrap()
            {
                map.insert(first_input, first_character);
                map.insert(second_input, second_character);
            } else {
                map.insert(first_input, second_character);
                map.insert(second_input, first_character);
            }
        };

        for inp in line.input.iter().filter(|out| out.len() == 2) {
            map_based_on_count(
                &mut mapping,
                inp.chars().nth(0).unwrap(),
                inp.chars().nth(1).unwrap(),
                'f',
                'c',
            );
        }

        for inp in line
            .input
            .iter()
            .filter(|out| [3, 4].iter().any(|&len| len == out.len()))
        {
            if inp.len() == 3 {
                let pos = inp.chars().position(|c| !mapping.contains_key(&c)).unwrap();
                mapping.insert(inp.chars().nth(pos).unwrap(), 'a');
            } else {
                let unmapped: Vec<char> =
                    inp.chars().filter(|c| !mapping.contains_key(&c)).collect();
                map_based_on_count(&mut mapping, unmapped[0], unmapped[1], 'd', 'b');
            }
        }

        let eight_digit = line.input[line.input.iter().position(|inp| inp.len() == 7).unwrap()];
        let unmapped: Vec<char> = eight_digit
            .chars()
            .filter(|c| !mapping.contains_key(&c))
            .collect();
        map_based_on_count(&mut mapping, unmapped[0], unmapped[1], 'e', 'g');

        sum += line
            .output
            .iter()
            .map(|out| {
                let decoded: HashSet<char> =
                    out.chars().map(|c| *mapping.get(&c).unwrap()).collect();
                number_map
                    .iter()
                    .position(|segments| segments.eq(&decoded))
                    .unwrap()
            })
            .fold(0, |accum, num| accum * 10 + num);
    }

    println!("part2 result {}", sum);
}
