use std::collections::HashMap;

use player::Player;
use team::Team;
use thirtyfour_sync::prelude::*;

mod driver;
mod league;
mod player;
mod team;
mod terminal_helper;

use crate::league::League;
use crate::terminal_helper::TerminalHelper;

fn main() {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless().expect("Couldn't set chrome in headless mode");
    let driver = WebDriver::new("http://localhost:4444", &caps).unwrap();

    let mut terminal_helper = TerminalHelper::new();

    let whitelist: Option<Vec<&str>> = Some(vec!["LaLiga"]);
    let mut scrapping_data: HashMap<String, League> = HashMap::new();

    let leagues_raw_data = League::get_leagues_raw_data(&driver);
    let league_percentage = match &whitelist {
        Some(list) => 95.0 / list.len() as f64,
        None => 95.0 / leagues_raw_data.len() as f64,
    };
    for league_raw in leagues_raw_data {
        let mut league = League::scrape_league_element(&league_raw);

        if let Some(leagues_list) = &whitelist {
            if !leagues_list.iter().any(|&i| i == league.name) {
                continue;
            }
        }

        terminal_helper.push_league_item(league.name.clone());
        terminal_helper.clean_teams_list();
        terminal_helper.clean_players_list();

        let teams_raw_data = Team::get_teams_raw_data(&driver, &league);
        let teams_percentage = league_percentage / teams_raw_data.len() as f64;
        for team_raw in teams_raw_data {
            let mut team = Team::scrape_team_element(&team_raw);

            terminal_helper.push_team_item(team.name.clone());
            terminal_helper.clean_players_list();

            let players_raw_data = Player::get_players_raw_data(&driver, &team);
            let players_percentage = teams_percentage / players_raw_data.len() as f64;
            for player_raw in players_raw_data {
                let player = Player::scrape_player_element(&player_raw);

                terminal_helper.push_player_item(player.name.clone());
                terminal_helper.add_percentage(players_percentage);

                team.players.insert(player.name.clone(), player);
            }

            league.teams.insert(team.name.clone(), team);
            break;
        }

        scrapping_data.insert(league.name.clone(), league);
        break;
    }

    // 95% finish aprox
    let scrapping_data_json =
        serde_json::to_string(&scrapping_data).expect("Error while serializing data to JSON");

    // 100% finish
    terminal_helper.set_percentage(100.0);
    terminal_helper.close();
}
