use crate::team::Team;

pub struct League {
    name: String,
    logo_url: String,
    league_url: String,
    teams: Vec<Team>,
}
