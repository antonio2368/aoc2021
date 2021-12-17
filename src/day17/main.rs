fn number_sum(num: i64) -> i64 {
    (num * (num + 1)) / 2
}

fn is_valid_y_velocity(y: i64, (y_min, y_max): (i64, i64)) -> (i64, Vec<i64>) {
    let abs_y = i64::abs(y);
    let peak = if y > 0 { number_sum(abs_y) } else { 0 };
    let mut current_velocity = abs_y;
    let mut current_distance = 0;
    if y > 0 {
        current_velocity += 1;
    }
    let get_steps = |current_velocity| {
        let abs_y = i64::abs(y);
        if y > 0 {
            2 * abs_y + 1 + current_velocity - abs_y
        } else {
            current_velocity - abs_y + 1
        }
    };
    let mut steps: Vec<i64> = Vec::new();
    loop {
        current_distance -= current_velocity;
        if current_distance >= y_min && current_distance <= y_max {
            steps.push(get_steps(current_velocity));
        } else if current_distance < y_min {
            return (peak, steps);
        }

        current_velocity += 1;
    }
}

fn main() {
    let ((x_min, x_max), (y_min, y_max)) = {
        let file_content = std::fs::read_to_string("res/day17/input.txt").unwrap();
        let ranges: Vec<&str> = file_content
            .strip_prefix("target area: ")
            .unwrap()
            .split(", ")
            .collect();
        let get_range = |range_string: &str| {
            let range: Vec<i64> = range_string
                .split("..")
                .map(|val| val.parse::<i64>().unwrap())
                .collect();
            (range[0], range[1])
        };
        (
            get_range(ranges[0].strip_prefix("x=").unwrap()),
            get_range(ranges[1].strip_prefix("y=").unwrap().trim_end()),
        )
    };

    let mut max_height: i64 = 0;
    for y in 0..i64::abs(y_min) {
        let current_extra = number_sum(y);
        let (extra, steps) = is_valid_y_velocity(y, (y_min, y_max));
        if !steps.is_empty() && extra > max_height {
            max_height = current_extra;
        }
    }

    println!("Part 1 result {}", max_height);

    let mut num = 0;
    for y in y_min..(i64::abs(y_min) + 1) {
        let (_, steps) = is_valid_y_velocity(y, (y_min, y_max));
        if !steps.is_empty() {
            for x in 0..(x_max + 1) {
                let x_sum = number_sum(x);
                for step in &steps {
                    let mut x_sum = x_sum;
                    if step < &x {
                        x_sum -= number_sum(x - step);
                    }

                    if x_sum >= x_min && x_sum <= x_max {
                        num += 1;
                        break;
                    }
                }
            }
        }
    }
    println!("Part 2 result {}", num);
}
