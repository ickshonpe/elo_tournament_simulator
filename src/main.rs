extern crate rand;

mod tournament_data;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Player {
    pub name: String,
    pub elo: i64
}

impl Player {
    pub fn new(name: &str, elo: i64) -> Player {
        let name = String::from(name);
        Player {
            name,
            elo
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
    let (mut top_seeds, mut mid_seeds, mut low_seeds) = tournament_data::get_player_data();

    let total_players = top_seeds.len() + mid_seeds.len() + low_seeds.len();
    let group_size = 4;
    let groups_count = total_players / group_size;
    let mut groups: Vec<Vec<Player>> = Vec::with_capacity(groups_count);
    let knockout_players = total_players / 2;
    let mut knockout_high_seeds: Vec<Player> = Vec::with_capacity(knockout_players / 2);
    let mut knockout_low_seeds: Vec<Player> = Vec::with_capacity(knockout_players / 2);

    for i in 0..groups_count {
        let mut group : Vec<Player> = Vec::with_capacity(group_size);
        group.push(draw_player(&mut top_seeds));
        group.push(draw_player(&mut top_seeds));
        group.push(draw_player(&mut mid_seeds));
        group.push(draw_player(&mut low_seeds));
        groups.push(group);
    }

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
                println!("\t{} vs {} -> {} to {}", player_a.name, player_b.name, a_score, b_score);
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
            println!("\t{}: wins: {}, losses: {}, maps: {} ", group_standings[i].player.name, group_standings[i].wins, group_standings[i].losses, group_standings[i].map_difference);
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
    let mut knockout_players: Vec<Player> = Vec::new();
    println!();
    println!("Knockout Round of 16");

    for i in 0..8 {
        let player_a = draw_player(&mut knockout_high_seeds);
        let player_b = draw_player(&mut knockout_low_seeds);
        let (a_score, b_score) = predict_match_winner(&player_a, &player_b, 3);
        let winner = if a_score > b_score {
            player_a.clone()
        } else {
            player_b.clone()
        };
        println!("\t{} vs {}, {} to {} -> {} wins ", player_a.name, player_b.name, a_score, b_score, winner.name);
        knockout_players.push(winner);
    }
    println!();
    println!("Quarter Finals");
    let mut round_of_4 = Vec::new();
    for i in 0..4 {
        let player_a = knockout_players[i * 2].clone();
        let player_b = knockout_players[i * 2 + 1].clone();
        let (a_score, b_score) = predict_match_winner(&player_a, &player_b, 3);
        let winner = if a_score > b_score {
            player_a.clone()
        } else {
            player_b.clone()
        };
        println!("\t{} vs {}, {} to {} -> {} wins ", player_a.name, player_b.name, a_score, b_score, winner.name);
        round_of_4.push(winner);
    }

    println!();
    println!("Semi Finals");
    let mut finalists = Vec::new();
    for i in 0..2 {
        let player_index = i * 2;
        let player_a = round_of_4[player_index].clone();
        let player_b = round_of_4[player_index + 1].clone();
        let (a_score, b_score) = predict_match_winner(&player_a, &player_b, 3);
        let winner = if a_score > b_score {
            player_a.clone()
        } else {
            player_b.clone()
        };
        println!("\t{} vs {}, {} to {} -> {} wins ", player_a.name, player_b.name, a_score, b_score, winner.name);
        finalists.push(winner);
    }
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
    println!("The Tournament Champion is {}", winner.name);

}

fn draw_player(seed_group: &mut Vec<Player>) -> Player {
    use rand::Rng;
    let drawn_index = rand::thread_rng().gen_range(0, seed_group.len());
    seed_group.remove(drawn_index)
}



fn predict_map_winner<'a>(a: &'a Player, b : &'a Player) -> &'a Player {
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
        let map_result = predict_map_winner(player_a, player_b);
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