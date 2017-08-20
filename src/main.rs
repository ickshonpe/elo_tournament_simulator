extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod tournament_data;
mod player;
mod tournament;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub elo: i64,
    pub seeding: i64    // lower means better
}

impl Player {
    pub fn new(name: &str, elo: i64, seeding: i64) -> Player {
        let name = String::from(name);
        Player {
            name,
            elo,
            seeding
        }
    }
}

#[derive(Clone)]
struct GroupRecord {
    player: Player,
    wins: i64,
    losses: i64,
    map_difference: i64
}

impl GroupRecord {
    pub fn new(player: Player) -> GroupRecord {
        GroupRecord {
            player,
            wins: 0,
            losses: 0,
            map_difference: 0
        }
    }
}

fn main() {
    let mut player_data = tournament_data::load_players();
    let total_players = player_data.len();
    let group_size = 4;
    let groups_count = player_data.len() / group_size;
    let mut groups = generate_groups(&player_data, 4);
    let knockout_players_count = total_players / 2;
    let mut knockout_high_seeds = Vec::<Player>::new();
    let mut knockout_low_seeds = Vec::<Player>::new();

    for (i, group) in groups.iter().enumerate() {
        println!("Group {}:", i);
        for player in group {
            println!("\t{}", player.name);
        }
        println!("Match Results:");
        let mut group_standings: Vec<GroupRecord> = (0..group_size).map(|i| { GroupRecord::new(group[i].clone()) }).collect();
        for player_a_index in 0..group.len() - 1 {
            for player_b_index in player_a_index + 1..group.len() {
                let player_a: &Player = &group[player_a_index];
                let player_b: &Player = &group[player_b_index];
                let (a_score, b_score) = predict_match_winner(player_a, player_b, 3);

                group_standings[player_a_index].map_difference += a_score - b_score;
                group_standings[player_b_index].map_difference += b_score - a_score;
                println!("\t{} vs {}, {} to {} -> {} wins ", player_a.name, player_b.name, a_score, b_score, if a_score > b_score { &player_a.name } else { &player_b.name });
                if a_score > b_score {
                    group_standings[player_a_index].wins += 1;
                    group_standings[player_b_index].losses += 1;
                } else {
                    group_standings[player_a_index].losses += 1;
                    group_standings[player_b_index].wins += 1;
                }
            }
        }
        println!("Final Standings:");
        for i in 0..group.len() {
            println!("\t{}: wins: {}, losses: {}, map diff: {} ", group_standings[i].player.name, group_standings[i].wins, group_standings[i].losses, group_standings[i].map_difference);
        }
        println!("qualifiers for knockout:");

        group_standings.sort_by(group_comparator);
        println!("high seed: {}", group_standings[3].player.name);
        println!("low seed: {}", group_standings[2].player.name);
        knockout_high_seeds.push(group_standings[3].player.clone());
        knockout_low_seeds.push(group_standings[2].player.clone());
        println!();
    }

    // Knockout stages
    let mut knockout_players: Vec<Player> = interleave_slices(&knockout_high_seeds, &knockout_low_seeds);

    let number_of_knockout_rounds = calculate_number_of_rounds(knockout_players_count);
    println!("Number of knockout rounds: {}", number_of_knockout_rounds);
    println!();
    println!("Knockout Round of 16");
    let knockout_players = predict_knockout_matches(knockout_players, 3);

    println!();
    println!("Quarter Finals");
    let mut round_of_4 = predict_knockout_matches(knockout_players, 3);

    println!();
    println!("Semi Finals");
    let mut finalists = predict_knockout_matches(round_of_4, 3);
    println!();
    println!("Final");
    let player_a = finalists[0].clone();
    let player_b = finalists[1].clone();
    let (a_score, b_score) = predict_match_winner(&player_a, &player_b, 3);
    let winner = if a_score > b_score {
        player_a.clone()
    } else {
        player_b.clone()
    };
    println!("\t{} vs {}, {} to {} -> {} wins ", player_a.name, player_b.name, a_score, b_score, winner.name);

    println!();
    println!("The Tournament Champion is {}!", winner.name);

}

fn draw_player(seed_group: &mut Vec<Player>) -> Player {
    use rand::Rng;
    let drawn_index = rand::thread_rng().gen_range(0, seed_group.len());
    seed_group.remove(drawn_index)
}

fn predict_round_winner<'a>(a: &'a Player, b : &'a Player) -> &'a Player {
    use std::f64;
    use rand::Rng;
    let diff = (b.elo - a.elo) as f64;
    let m: f64 = diff / 400.0;
    let p = 1.0/ (1.0 + (10.0 as f64).powf(m));
    let c = rand::thread_rng().next_f64();
    if p >= c {
        a
    }  else {
        b
    }
}

fn predict_match_winner(player_a: &Player, player_b: &Player, num_rounds: i64) -> (i64, i64) {
    if num_rounds % 2 == 0 || num_rounds < 1 {
        panic!("Invalid numbers of rounds for a match: {}", num_rounds);
    }
    let target = num_rounds / 2 + 1;
    let mut a_score = 0;
    let mut b_score = 0;
    for map in 0..num_rounds {
        let map_result = predict_round_winner(player_a, player_b);
        if map_result == player_a {
            a_score += 1;
        } else {
            b_score += 1;
        }
        if a_score == target || b_score == target {
            break;
        }
    }
    (a_score, b_score)
}

fn predict_knockout_matches(knockout_players: Vec<Player>, num_rounds: i64) -> Vec<Player> {
    let mut winners = Vec::new();
    let num_matches = knockout_players.len() / 2;
    for i in 0..num_matches {
        let player_a = knockout_players[i * 2].clone();
        let player_b = knockout_players[i * 2 + 1].clone();
        let (a_score, b_score) = predict_match_winner(&player_a, &player_b, num_rounds);
        let winner = if a_score > b_score {
            player_a.clone()
        } else {
            player_b.clone()
        };
        println!("\t{} vs {}, {} to {} -> {} wins ", player_a.name, player_b.name, a_score, b_score, winner.name);
        winners.push(winner);
    }
    if knockout_players.len() %2 == 1 {
        winners.push(knockout_players.last().unwrap().clone());
    }
    winners
}

use std::cmp::Ordering;
fn group_comparator(a: &GroupRecord, b: &GroupRecord) -> Ordering {
    if a.wins > b.wins {
        return Ordering::Greater;
    }
    if b.wins > a.wins {
        return Ordering::Less;
    }
    if a.map_difference > b.map_difference {
        return Ordering::Greater;
    }
    if b.map_difference > a.map_difference {
        return Ordering::Less;
    }
    Ordering::Less
}

fn seeding_comparator(player_a: &Player, player_b: &Player) -> Ordering {
    if player_a.seeding < player_b.seeding {
        Ordering::Less
    } else if player_a.seeding > player_b.seeding {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn interleave_slices<T: Clone>(xs: &[T], ys: &[T]) -> Vec<T> {
    if xs.len() != ys.len() {
        panic!("Slices for interleaving not same length.");
    }
    let mut out = Vec::new();
    for i in 0..xs.len() {
        out.push(xs[i].clone());
        out.push(ys[i].clone());
    }
    out
}

fn calculate_number_of_rounds(num_players: usize) -> usize {
    if num_players == 0 {
        0
    } else {
        (num_players as f64).log2().ceil() as usize
    }
}


fn generate_groups(players: &[Player], group_size: usize) -> Vec<Vec<Player>> {
    let total_players = players.len();
    let mut players = players.clone().to_vec();
    use rand::Rng;
    rand::thread_rng().shuffle(&mut players);
    players.sort_by(seeding_comparator);
    let groups_count = total_players / group_size;
    let mut groups = (0..groups_count).map(|_| { Vec::<Player>::new() }).collect::<Vec<Vec<Player>>>();
    for i in 0..group_size {
        for group in &mut groups {
            let next_player = players.pop().unwrap();
            group.push(next_player);
        }
    }
    groups
}
