use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

fn main() {
    let (mut dots, folds): (HashSet<(u32, u32)>, Vec<Fold>) =
        std::fs::read_to_string("res/day13/input.txt")
            .unwrap()
            .lines()
            .fold(
                (HashSet::new(), Vec::new()),
                |(mut dots, mut folds), line| {
                    let fold_start = "fold along ";
                    if line.starts_with(fold_start) {
                        let fold: Vec<&str> =
                            line.strip_prefix(fold_start).unwrap().split("=").collect();
                        let value = fold[1].parse::<u32>().unwrap();
                        folds.push(match fold[0] {
                            "y" => Fold::Y(value),
                            "x" => Fold::X(value),
                            _ => panic!("Invalid value for fold orientation"),
                        });
                    } else if !line.is_empty() {
                        let dot: Vec<u32> = line
                            .split(",")
                            .map(|value| value.parse::<u32>().unwrap())
                            .collect();
                        dots.insert((dot[0], dot[1]));
                    }
                    (dots, folds)
                },
            );

    let mut first = true;
    for fold in folds {
        let mut new_dots: HashSet<(u32, u32)> = HashSet::new();
        match fold {
            Fold::Y(value) => {
                for mut dot in dots {
                    if dot.1 < value {
                        new_dots.insert(dot);
                    } else {
                        dot.1 = value - (dot.1 - value);
                        new_dots.insert(dot);
                    }
                }
            }
            Fold::X(value) => {
                for mut dot in dots {
                    if dot.0 < value {
                        new_dots.insert(dot);
                    } else {
                        dot.0 = value - (dot.0 - value);
                        new_dots.insert(dot);
                    }
                }
            }
        }
        dots = new_dots;

        if first {
            first = false;
            println!("Number of visible dots after first fold: {}", dots.len());
        }
    }

    let (max_x, max_y) = dots.iter().fold((0, 0), |(max_x, max_y), &(x, y)| {
        (max_x.max(x), max_y.max(y))
    });

    let mut grid: Vec<Vec<bool>> = {
        let mut grid = Vec::new();

        for _ in 0..max_y + 1 {
            grid.push(vec![false; (max_x + 1) as usize]);
        }
        grid
    };

    for dot in dots {
        grid[dot.1 as usize][dot.0 as usize] = true;
    }

    for row in grid {
        for has_dot in row {
            print!("{}", if has_dot { "#" } else { " " });
        }
        print!("\n");
    }
}
