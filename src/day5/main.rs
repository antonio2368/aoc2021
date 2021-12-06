use std::fs;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

fn get_range(first: i32, second: i32) -> Box<dyn Iterator<Item = i32>> {
    if first < second {
        Box::new((first..second + 1).into_iter())
    } else {
        Box::new((second..first + 1).rev().into_iter())
    }
}

fn part1(lines: &Vec<Line>, mut board: Vec<i32>, row_size: i32, allow_diagonal: bool) -> usize {
    for line in lines {
        if line.from.y == line.to.y {
            let row = line.from.y * row_size;
            for i in get_range(line.from.x, line.to.x) {
                let idx = (row + i) as usize;
                board[idx] += 1;
            }
        } else if line.from.x == line.to.x {
            // vertical move
            for i in get_range(line.from.y, line.to.y) {
                let idx = (i * row_size + line.from.x) as usize;
                board[idx] += 1;
            }
        } else if allow_diagonal {
            for (x, y) in get_range(line.from.x, line.to.x).zip(get_range(line.from.y, line.to.y)) {
                let idx = (y * row_size + x) as usize;
                board[idx] += 1;
            }
        }
    }
    board.into_iter().filter(|&elem| elem >= 2).count()
}

fn main() {
    let file_content = fs::read_to_string("res/day5/input.txt").unwrap();

    let lines: Vec<Line> = file_content
        .lines()
        .map(|line| {
            let points: Vec<Point> = line
                .split(" -> ")
                .map(|point| {
                    let xy: Vec<i32> = point
                        .split(',')
                        .map(|c| c.parse::<i32>().unwrap())
                        .collect();
                    Point { x: xy[0], y: xy[1] }
                })
                .collect();
            Line {
                from: points[0],
                to: points[1],
            }
        })
        .collect();

    let max_xy = lines.iter().fold((0, 0), |(max_x, max_y), line| {
        (
            max_x.max(line.from.x).max(line.to.x),
            max_y.max(line.from.y).max(line.to.y),
        )
    });

    let board: Vec<i32> = vec![0; (max_xy.0 + 1) as usize * (max_xy.1 + 1) as usize];
    println!(
        "Part1 result {}",
        part1(&lines, board.clone(), max_xy.0 + 1, false)
    );
    println!("Part2 result {}", part1(&lines, board, max_xy.0 + 1, true));
}
