#[derive(Debug)]
struct Timer {
    value: u8,
    count: u64,
}

fn main() {
    let initial_timers: Vec<u8> = std::fs::read_to_string("res/day6/input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|value| value.parse::<u8>().unwrap())
        .collect();
    let mut timers: Vec<Timer> = Vec::new();

    for i in 0..7 {
        timers.push(Timer { value: i, count: 0 });
    }

    let mut total_count: u64 = 0;
    for initial_timer in initial_timers {
        total_count += 1;
        let elem = timers
            .iter_mut()
            .find(|timer| timer.value == initial_timer)
            .unwrap();
        elem.count += 1;
    }

    let mut day7_count = 0;
    let mut day8_count = 0;

    for i in 0..256 {
        let mut new_spawns: u64 = 0;
        for timer in &mut timers {
            if timer.value == 0 {
                new_spawns += timer.count;
                timer.value = 6;
            } else {
                timer.value -= 1;
            }
        }
        let elem = timers.iter_mut().find(|timer| timer.value == 6).unwrap();
        elem.count += day7_count;

        day7_count = day8_count;
        day8_count = new_spawns;
        total_count += new_spawns;

        if i == 79 || i == 255 {
            println!("After day {}: {} fish", i + 1, total_count);
        }
    }
}
