use std::collections::HashMap;

fn get_count_difference(pairs: &HashMap<(char, char), usize>, last_character: char) -> usize {
    let mut char_counts: HashMap<char, usize> =
        pairs.iter().fold(HashMap::new(), |mut map, (pair, count)| {
            *map.entry(pair.0).or_insert(0) += count;
            map
        });

    *char_counts.entry(last_character).or_insert(0) += 1;

    let (min_count, max_count) = char_counts
        .values()
        .fold((usize::MAX, usize::MIN), |(min, max), &count| {
            (min.min(count), max.max(count))
        });
    max_count - min_count
}

fn main() {
    let file_content = std::fs::read_to_string("res/day14/input.txt").unwrap();

    let mut lines_iter = file_content.lines();

    let start_line: Vec<char> = lines_iter.next().unwrap().chars().collect();
    let last_character = *start_line.iter().rev().next().unwrap();
    let mut pairs: HashMap<(char, char), usize> =
        start_line.windows(2).fold(HashMap::new(), |mut map, pair| {
            let mut pair_iter = pair.iter();
            *map.entry((*pair_iter.next().unwrap(), *pair_iter.next().unwrap()))
                .or_insert(0) += 1;
            map
        });

    assert_eq!(lines_iter.next(), Some(""));

    let rules: HashMap<(char, char), char> = lines_iter.fold(HashMap::new(), |mut map, line| {
        let rule: Vec<&str> = line.split(" -> ").collect();
        map.insert(
            (
                rule[0].chars().nth(0).unwrap(),
                rule[0].chars().nth(1).unwrap(),
            ),
            rule[1].chars().next().unwrap(),
        );
        map
    });

    for i in 0..40 {
        pairs = pairs
            .into_iter()
            .fold(HashMap::new(), |mut next, (pair, count)| {
                match rules.get(&pair) {
                    Some(&value) => {
                        *next.entry((pair.0, value)).or_insert(0) += count;
                        *next.entry((value, pair.1)).or_insert(0) += count;
                    }
                    None => {
                        *next.entry(pair).or_insert(0) += count;
                    }
                }
                next
            });
        if i == 9 {
            println!(
                "Part 1 result {}",
                get_count_difference(&pairs, last_character)
            );
        }
    }
    println!(
        "Part 2 result {}",
        get_count_difference(&pairs, last_character)
    );
}
