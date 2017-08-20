
#[derive(Clone, Debug, Eq, PartialEq)]
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

}