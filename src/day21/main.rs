use std::collections::HashMap;

type PlayerStatus = (u64, u64);

type QuantumCache = HashMap<((PlayerStatus, PlayerStatus), usize), (u64, u64)>;
fn play_quantum_game(
    players: (PlayerStatus, PlayerStatus),
    current_player: usize,
    mut cache: QuantumCache,
) -> ((u64, u64), QuantumCache) {
    let mut count = (0, 0);
    for i in 1..4 {
        for j in 1..4 {
            for k in 1..4 {
                let mut players = players.clone();
                let mut player = if current_player == 0 {
                    &mut players.0
                } else {
                    &mut players.1
                };
                player.0 = (player.0 + i + j + k - 1) % 10 + 1;
                player.1 += player.0;
                if player.1 >= 21 {
                    if current_player == 0 {
                        count.0 += 1;
                    } else {
                        count.1 += 1;
                    }
                } else {
                    if let Some(result) = cache.get(&(players, 1 - current_player)) {
                        count.0 += result.0;
                        count.1 += result.1
                    } else {
                        let (result, mut new_cache) =
                            play_quantum_game(players, 1 - current_player, cache);
                        count.0 += result.0;
                        count.1 += result.1;
                        new_cache.insert((players, 1 - current_player), result);
                        cache = new_cache;
                    }
                }
            }
        }
    }
    (count, cache)
}

fn play_game(player1_position: u64, player2_position: u64) -> u64 {
    let mut players = ((player1_position, 0), (player2_position, 0));

    let mut current_player = 0;

    let mut rolls = 0;
    let mut current_roll = 1;
    loop {
        rolls += 3;
        let (first, second) = if current_player == 0 {
            (&mut players.0, &mut players.1)
        } else {
            (&mut players.1, &mut players.0)
        };
        first.0 = ((first.0 + 3 * current_roll + 3) - 1) % 10 + 1;
        first.1 += first.0;

        if first.1 >= 1000 {
            return second.1 * rolls;
        }

        current_player = 1 - current_player;
        current_roll = ((current_roll + 3 - 1) % 100) + 1;
    }
}

fn main() {
    println!("Part 1 result {}", play_game(5, 8));

    let players = ((5, 0), (8, 0));

    let (stats, _) = play_quantum_game(players, 0, HashMap::new());
    println!(
        "Part 2 result {}",
        stats.0.max(stats.1)
    );
}
