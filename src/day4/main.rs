use std::collections::HashMap;
use std::fs;

struct Board {
    rows: Vec<Vec<i32>>,
    column_counter: HashMap<usize, usize>,
    row_counter: HashMap<usize, usize>,
    unmarked_board_sum: i32,
}

impl Board {
    fn new(rows: Vec<Vec<i32>>) -> Board {
        let unmarked_board_sum = rows.iter().map(|row| row.iter().sum::<i32>()).sum();
        Board {
            rows,
            column_counter: HashMap::new(),
            row_counter: HashMap::new(),
            unmarked_board_sum,
        }
    }

    fn reset(&mut self) {
        self.column_counter.clear();
        self.row_counter.clear();
        self.unmarked_board_sum = self.rows.iter().map(|row| row.iter().sum::<i32>()).sum();
    }

    fn new_draw(&mut self, value: i32) -> Option<i32> {
        for (i, row) in self.rows.iter().enumerate() {
            if let Some(position) = row.iter().position(|&elem| elem == value) {
                self.unmarked_board_sum -= value;
                let count = self.row_counter.entry(i).or_insert(0);
                *count += 1;
                if count == &5 {
                    return Some(self.unmarked_board_sum);
                }
                let count = self.column_counter.entry(position).or_insert(0);
                *count += 1;
                if count == &5 {
                    return Some(self.unmarked_board_sum);
                }
            }
        }

        None
    }
}

fn part1(draws: &[i32], boards: &mut Vec<Board>) -> i32 {
    for &value in draws {
        for board in &mut *boards {
            if let Some(sum) = board.new_draw(value) {
                return sum * value;
            }
        }
    }
    panic!("No board won");
}

fn part2(draws: &[i32], boards: &mut Vec<Board>) -> i32 {
    let mut current_boards: Vec<&mut Board> = boards.iter_mut().collect();
    for &value in draws {
        let mut next_boards: Vec<&mut Board> = Vec::new();
        let boards_len = current_boards.len();
        for board in current_boards {
            if let Some(sum) = board.new_draw(value) {
                if boards_len == 1 {
                    return sum * value;
                }
            } else {
                next_boards.push(board);
            }
        }
        current_boards = next_boards;
    }
    panic!("No board won");
}

fn main() {
    let file_content = fs::read_to_string("res/day4/input.txt").unwrap();

    let mut line_iter = file_content.lines();

    let draws: Vec<i32> = line_iter
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    while let Some(line) = line_iter.next() {
        assert!(line.len() == 0);

        let mut rows: Vec<Vec<i32>> = Vec::new();
        for _ in 0..5 {
            rows.push(
                line_iter
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|value| value.parse::<i32>().unwrap())
                    .collect(),
            );
        }

        boards.push(Board::new(rows));
    }

    println!("part1 result {}", part1(&draws, &mut boards));
    for board in &mut boards {
        board.reset();
    }
    println!("part2 result {}", part2(&draws, &mut boards));
}
