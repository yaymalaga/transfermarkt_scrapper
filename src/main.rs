use player::Player;
use team::Team;
use thirtyfour_sync::prelude::*;

mod driver;
mod league;
mod player;
mod team;

use crate::league::League;

fn main() {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).unwrap();

    let whitelist: Option<Vec<&str>> = None;
    let mut scrapping_data = vec![];

    let leagues_raw_data = League::get_leagues_raw_data(&driver);
    for league_raw in leagues_raw_data {
        let mut league = League::scrape_league_element(&league_raw);

        if let Some(leagues_list) = &whitelist {
            if leagues_list.iter().any(|&i| i == league.name) {
                continue;
            }
        }

        let teams_raw_data = Team::get_teams_raw_data(&driver, &league);
        for team_raw in teams_raw_data {
            let mut team = Team::scrape_team_element(&team_raw);

            let players_raw_data = Player::get_players_raw_data(&driver, &team);
            for player_raw in players_raw_data {
                let player = Player::scrape_player_element(&player_raw);

                team.players.push(player);
            }

            league.teams.push(team);
        }

        scrapping_data.push(league);
    }
}
