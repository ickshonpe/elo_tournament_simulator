use ::Player;

pub fn get_player_data() -> (Vec<Player>, Vec<Player>, Vec<Player>) {
    let mut top_seeds = vec![
        Player::new("chance", 2012),
        Player::new("dahang", 1975),
        Player::new("tomservo", 1909),
        Player::new("whaz", 1979),
        Player::new("carnage", 1982),
        Player::new("rapha", 1994),
        Player::new("vedmedik", 1967),
        Player::new("dooi", 1890),
        Player::new("xron", 2038),
        Player::new("avek", 2078),
        Player::new("k1llsen", 2166),
        Player::new("clawz", 2020),
        Player::new("fazz", 2114),
        Player::new("noctis", 2047),
        Player::new("strenx", 2111),
        Player::new("cooller", 2262)
    ];
    let mut mid_seeds = vec![
        Player::new("raisy", 2131),
        Player::new("witchl", 2055),
        Player::new("toxic", 2153),
        Player::new("voo", 2058),
        Player::new("gellehsak", 1965),
        Player::new("griffin", 1968),
        Player::new("scizr", 1941),
        Player::new("dandaking", 1980)
    ];
    let mut low_seeds = vec![
        Player::new("ev1l", 2010),
        Player::new("zenaku", 1783),
        Player::new("astroboy", 1936),
        Player::new("fraze", 2050),
        Player::new("python", 1810),
        Player::new("steej", 1966),
        Player::new("pit", 1866),
        Player::new("ruleth", 1854)
    ];
    (top_seeds, mid_seeds, low_seeds)
}