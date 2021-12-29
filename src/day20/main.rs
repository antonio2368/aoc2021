use std::collections::HashSet;

type Coordinate = (i64, i64);

fn get_index(
    lit_pixels: &HashSet<Coordinate>,
    coordinate: Coordinate,
    horizontal_range: Coordinate,
    vertical_range: Coordinate,
    padding_value: bool,
) -> usize {
    let mut num = 0;
    for vertical in -1..2 {
        for horizontal in -1..2 {
            num <<= 1;
            let coordinate = (coordinate.0 + vertical, coordinate.1 + horizontal);
            if coordinate.0 < horizontal_range.0
                || coordinate.0 > horizontal_range.1
                || coordinate.1 < vertical_range.0
                || coordinate.1 > vertical_range.1
            {
                num |= padding_value as usize;
            } else {
                num |= lit_pixels.contains(&coordinate) as usize;
            }
        }
    }
    num
}

fn enhance(
    lit_pixels: HashSet<Coordinate>,
    algorithm: &HashSet<usize>,
    padding_value: bool,
) -> (HashSet<Coordinate>, bool) {
    let ((min_x, max_x), (min_y, max_y)) = lit_pixels.iter().fold(
        ((i64::MAX, i64::MIN), (i64::MAX, i64::MIN)),
        |((min_x, max_x), (min_y, max_y)), coordinate| {
            (
                (min_x.min(coordinate.0), max_x.max(coordinate.0)),
                (min_y.min(coordinate.1), max_y.max(coordinate.1)),
            )
        },
    );
    let mut new_lit_pixels: HashSet<Coordinate> = HashSet::new();
    for i in min_y - 1..max_y + 2 {
        for j in min_x - 1..max_x + 2 {
            let coordinate = (i, j);
            if algorithm.contains(&get_index(
                &lit_pixels,
                coordinate,
                (min_x, max_x),
                (min_y, max_y),
                padding_value,
            )) {
                new_lit_pixels.insert(coordinate);
            }
        }
    }

    (
        new_lit_pixels,
        if algorithm.contains(&0) {
            !padding_value
        } else {
            false
        },
    )
}

fn main() {
    let file_content = std::fs::read_to_string("res/day20/input.txt").unwrap();
    let mut lines = file_content.lines();
    let algorithm: HashSet<usize> =
        lines
            .next()
            .unwrap()
            .chars()
            .enumerate()
            .fold(HashSet::new(), |mut set, (i, c)| {
                if c == '#' {
                    set.insert(i);
                }
                set
            });

    assert!(lines.next().unwrap().is_empty());

    let mut lit_pixels: HashSet<Coordinate> =
        lines.enumerate().fold(HashSet::new(), |set, (i, line)| {
            line.chars().enumerate().fold(set, |mut set, (j, c)| {
                if c == '#' {
                    set.insert((i as i64, j as i64));
                }
                set
            })
        });

    let mut padding_value = false;
    for i in 0..50 {
        if i == 2 {
            println!("Part 1 result {}", lit_pixels.len());
        }
        let result = enhance(lit_pixels, &algorithm, padding_value);
        lit_pixels = result.0;
        padding_value = result.1;
    }

    println!("Part 2 result {}", lit_pixels.len());
}
