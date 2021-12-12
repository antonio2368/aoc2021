use std::collections::{HashMap, HashSet};

fn dfs<'a>(
    current: &'a str,
    extra_visit_used: bool,
    mut visited: HashSet<&'a str>,
    edges: &HashMap<&str, Vec<&str>>,
) -> u32 {
    if current == "end" {
        return 1;
    }

    if current.chars().nth(0).unwrap().is_lowercase() {
        visited.insert(current);
    }

    let mut count = 0;
    for &to in &edges[current] {
        let mut extra_visit_used = extra_visit_used;
        if visited.contains(to) {
            if !extra_visit_used && to != "start" && to != "end" {
                extra_visit_used = true;
            } else {
                continue;
            }
        }
        count += dfs(to, extra_visit_used, visited.clone(), edges);
    }
    count
}

fn main() {
    let file_content = std::fs::read_to_string("res/day12/input.txt").unwrap();
    let edges: HashMap<&str, Vec<&str>> =
        file_content.lines().fold(HashMap::new(), |mut map, line| {
            let edge: Vec<&str> = line.trim().split("-").collect();
            map.entry(edge[0]).or_insert(Vec::new()).push(edge[1]);
            map.entry(edge[1]).or_insert(Vec::new()).push(edge[0]);
            map
        });

    println!(
        "Part 1 result {}",
        dfs("start", true, HashSet::new(), &edges)
    );
    println!(
        "Part 2 result {}",
        dfs("start", false, HashSet::new(), &edges)
    );
}
