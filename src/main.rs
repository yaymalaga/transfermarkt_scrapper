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
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).unwrap();

    let mut terminal_helper = TerminalHelper::new();

    let whitelist: Option<Vec<&str>> = None;
    let mut scrapping_data: HashMap<String, League> = HashMap::new();

    let leagues_raw_data = League::get_leagues_raw_data(&driver);
    for league_raw in leagues_raw_data {
        let mut league = League::scrape_league_element(&league_raw);

        if let Some(leagues_list) = &whitelist {
            if leagues_list.iter().any(|&i| i == league.name) {
                continue;
            }
        }

        terminal_helper.push_league_item(league.name.clone());

        let teams_raw_data = Team::get_teams_raw_data(&driver, &league);
        for team_raw in teams_raw_data {
            let mut team = Team::scrape_team_element(&team_raw);

            terminal_helper.push_team_item(team.name.clone());

            let players_raw_data = Player::get_players_raw_data(&driver, &team);
            for player_raw in players_raw_data {
                let player = Player::scrape_player_element(&player_raw);

                terminal_helper.push_player_item(player.name.clone());

                team.players.insert(player.name.clone(), player);
                break;
            }

            league.teams.insert(team.name.clone(), team);
            terminal_helper.clean_players_list();
            break;
        }

        scrapping_data.insert(league.name.clone(), league);
        terminal_helper.clean_teams_list();
        break;
    }

    terminal_helper.close();

    let scrapping_data_json =
        serde_json::to_string(&scrapping_data).expect("Error while serializing data to JSON");
    println!("{}", scrapping_data_json);
}
