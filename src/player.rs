use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub elo: i64,
    pub seeding: i64
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

    pub fn seeding_comparator(player_a: &Player, player_b: &Player) -> Ordering {
        player_a.seeding.cmp(&player_b.seeding)
    }
}