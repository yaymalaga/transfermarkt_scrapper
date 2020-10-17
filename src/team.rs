use crate::league::League;
use crate::player::Player;

pub struct Team {
    name: String,
    logo_url: String,
    team_url: String,
    players: Vec<Player>,
}

impl Team {
    pub fn scrape_teams_data(league: &League) -> Vec<Self>{
        vec![]
    }
}
