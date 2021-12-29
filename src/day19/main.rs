use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

type Beacon = [i64; 3];

fn distance(first: &Beacon, second: &Beacon) -> i64 {
    first
        .iter()
        .zip(second)
        .map(|(first, second)| i64::abs(second - first))
        .sum()
}

fn common_element_count(first: &HashMap<i64, usize>, second: &HashMap<i64, usize>) -> usize {
    let mut count: usize = 0;
    for (first_key, first_count) in first {
        if let Some(second_count) = second.get(first_key) {
            count += first_count.min(second_count);
        }
    }
    count
}

fn main() {
    let file_content = std::fs::read_to_string("res/day19/input.txt").unwrap();
    let scanner_reads: Vec<Vec<Beacon>> =
        file_content.lines().fold(Vec::new(), |mut scanners, line| {
            if line.is_empty() {
                return scanners;
            } else if line.starts_with("--- scanner") {
                scanners.push(Vec::new());
                return scanners;
            }

            let beacon: Vec<i64> = line
                .split(",")
                .map(|val| val.parse::<i64>().unwrap())
                .collect();
            scanners
                .last_mut()
                .unwrap()
                .push([beacon[0], beacon[1], beacon[2]]);
            scanners
        });

    let mut distances: Vec<HashMap<Beacon, HashMap<i64, usize>>> = Vec::new();
    for scanner in &scanner_reads {
        let mut distance_map = HashMap::new();

        for (first, second) in scanner.iter().tuple_combinations() {
            let distance = distance(first, second);
            *distance_map
                .entry(*first)
                .or_insert(HashMap::new())
                .entry(distance)
                .or_insert(0) += 1;
            *distance_map
                .entry(*second)
                .or_insert(HashMap::new())
                .entry(distance)
                .or_insert(0) += 1;
        }

        distances.push(distance_map);
    }

    let mut mapping: Vec<Option<(i64, (usize, usize, usize), Beacon, Beacon)>> =
        scanner_reads.iter().map(|_| None).collect();

    let mut unmapped_scanners: HashSet<usize> = (0..distances.len()).collect();
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0);
    while !queue.is_empty() {
        let i = queue.pop_front().unwrap();
        unmapped_scanners.remove(&i);
        for &j in &unmapped_scanners {
            if i == j {
                continue;
            }

            let first_scanner = &distances[i];
            let second_scanner = &distances[j];
            let mut beacon_pairs: Vec<(Beacon, Beacon)> = Vec::new();
            for (first_beacon, first_distances) in first_scanner {
                for (second_beacon, second_distances) in second_scanner {
                    if common_element_count(&first_distances, &second_distances) >= 11 {
                        beacon_pairs.push((*first_beacon, *second_beacon));
                    }
                }
            }
            if beacon_pairs.len() >= 11 {
                for x in [1, -1] {
                    for y in [1, -1] {
                        for z in [1, -1] {
                            for idx in (0..3).permutations(3) {
                                let idx = (idx[0], idx[1], idx[2]);
                                let transform_beacon = |beacon: Beacon| {
                                    [beacon[idx.0] * x, beacon[idx.1] * y, beacon[idx.2] * z]
                                };

                                let (first_beacon, second_beacon) = beacon_pairs[0];
                                let get_beacon_difference = |first: Beacon, second: Beacon| {
                                    let second = transform_beacon(second);
                                    [
                                        first[0] - second[0],
                                        first[1] - second[1],
                                        first[2] - second[2],
                                    ]
                                };
                                let difference = get_beacon_difference(first_beacon, second_beacon);

                                if beacon_pairs.iter().skip(1).all(|(first, second)| {
                                    get_beacon_difference(*first, *second) == difference
                                }) {
                                    mapping[j] = Some((i as i64, idx, [x, y, z], difference));
                                    queue.push_back(j);
                                }
                            }
                        }
                    }
                }
            }

        }
    }

    let mut beacons: HashSet<Beacon> = HashSet::new();
    for (i, scanner) in scanner_reads.iter().enumerate() {
        let transform_beacon = |mut beacon: Beacon| {
            let mut mapping_info = mapping[i];
            while let Some(info) = mapping_info {
                let (next_mapping, idx, multiplier, addition) = info;
                let mut new_beacon = [0; 3];
                new_beacon[0] = beacon[idx.0] * multiplier[0] + addition[0];
                new_beacon[1] = beacon[idx.1] * multiplier[1] + addition[1];
                new_beacon[2] = beacon[idx.2] * multiplier[2] + addition[2];
                beacon = new_beacon;

                mapping_info = mapping[next_mapping as usize];
            }
            beacon
        };

        for beacon in scanner {
            beacons.insert(transform_beacon(*beacon));
        }
    }

    println!("Part 1 result {}", beacons.len());

    let max_scanner_distance: i64 = mapping.iter().map(|mut mapping_info| {
        let mut beacon = [0, 0, 0];
        while let Some(info) = mapping_info {
            let (next_mapping, idx, multiplier, addition) = info;
            let mut new_beacon = [0; 3];
            new_beacon[0] = beacon[idx.0] * multiplier[0] + addition[0];
            new_beacon[1] = beacon[idx.1] * multiplier[1] + addition[1];
            new_beacon[2] = beacon[idx.2] * multiplier[2] + addition[2];
            beacon = new_beacon;

            mapping_info = &mapping[*next_mapping as usize];
        }
        beacon
    }).tuple_combinations().fold(i64::MIN, |max_value, (first, second)| {
        max_value.max(distance(&first, &second))
    });

    println!("Part2 result {}", max_scanner_distance);
}
