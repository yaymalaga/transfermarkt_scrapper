use crate::player::Player;

pub struct Team {
    name: String,
    logo_url: String,
    team_url: String,
    players: Vec<Player>,
}