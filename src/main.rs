extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod tournament_data;
mod player;
mod tournament;
mod util;

use player::Player;
use util::interleave_slices;

#[derive(Clone)]
struct GroupRecord {
    player: Player,
    wins: i64,
    losses: i64,
    game_difference: i64
}

impl GroupRecord {
    pub fn new(player: Player) -> GroupRecord {
        GroupRecord {
            player,
            wins: 0,
            losses: 0,
            game_difference: 0
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

                group_standings[player_a_index].game_difference += a_score - b_score;
                group_standings[player_b_index].game_difference += b_score - a_score;
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
            println!("\t{}: wins: {}, losses: {}, game diff: {} ", group_standings[i].player.name, group_standings[i].wins, group_standings[i].losses, group_standings[i].game_difference);
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
    let mut knockout_players: Vec<Player> = interleave_slices(&knockout_high_seeds, &knockout_low_seeds.iter().rev().cloned().collect::<Vec<Player>>());
    let number_of_knockout_rounds = calculate_number_of_rounds(knockout_players_count);
    for i in 0..number_of_knockout_rounds {
        let remaining_players = knockout_players.len();
        println!("Round {}, {} players remain.", i, remaining_players );
        knockout_players = predict_knockout_matches(knockout_players, if remaining_players == 2 { 5 } else { 3 });
    }
    println!();
    println!("The Tournament Champion is {}!", knockout_players[0].name);

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

fn predict_match_winner(player_a: &Player, player_b: &Player, num_games: i64) -> (i64, i64) {
    if num_games % 2 == 0 || num_games < 1 {
        panic!("Invalid numbers of rounds for a match: {}", num_games);
    }
    let target = num_games / 2 + 1;
    let mut a_score = 0;
    let mut b_score = 0;
    for game in 0..num_games {
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
    if a.game_difference > b.game_difference {
        return Ordering::Greater;
    }
    if b.game_difference > a.game_difference {
        return Ordering::Less;
    }
    Ordering::Less
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
    players.sort_by(Player::seeding_comparator);
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
