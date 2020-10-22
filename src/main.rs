use std::{collections::HashMap, fs::File, io::Write, path::Path};

use player::Player;
use team::Team;
use thirtyfour_sync::prelude::*;

mod driver;
mod league;
mod player;
mod team;
mod terminal_helper;

use crate::driver::Driver;
use crate::league::League;
use crate::terminal_helper::TerminalHelper;

fn main() {
    let path = Path::new("scrapping.json");
    let mut file = File::create(&path).expect("Error while creating output file");

    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless()
        .expect("Couldn't set chrome in headless mode");
    let driver = WebDriver::new("http://localhost:4444", &caps)
        .expect("Error while connecting to chromedriver");

    let mut terminal_helper = TerminalHelper::new();

    let mut whitelist: Option<Vec<&str>> = Some(vec![
        "Premier League",
        "LaLiga",
        "Serie A",
        "Bundesliga",
        "Ligue 1",
    ]);
    let mut scrapping_data: HashMap<String, League> = HashMap::new();

    let leagues_raw_data = League::get_leagues_raw_data(&driver);
    let league_percentage = match &whitelist {
        Some(list) => 95.0 / list.len() as f64,
        None => 95.0 / leagues_raw_data.len() as f64,
    };
    for league_raw in leagues_raw_data {
        let mut league = League::scrape_league_element(&league_raw);

        // Filter whitelist
        if let Some(leagues_list) = &mut whitelist {
            if leagues_list.is_empty() {
                break;
            }

            let mut index = None;

            for (i, &name) in leagues_list.iter().enumerate() {
                if name == league.name {
                    index = Some(i);
                    break;
                }
            }

            match index {
                Some(i) => {
                    leagues_list.remove(i);
                }
                None => continue,
            }
        }

        terminal_helper.push_league_item(league.name.clone());
        terminal_helper.clean_teams_list();
        terminal_helper.clean_players_list();

        open_new_tab(&driver);
        switch_to_tab(&driver, 1);

        let teams_raw_data = Team::get_teams_raw_data(&driver, &league);
        let teams_percentage = league_percentage / teams_raw_data.len() as f64;
        for team_raw in teams_raw_data {
            let mut team = Team::scrape_team_element(&team_raw);

            terminal_helper.push_team_item(team.name.clone());
            terminal_helper.clean_players_list();

            open_new_tab(&driver);
            switch_to_tab(&driver, 2);

            let players_raw_data = Player::get_players_raw_data(&driver, &team);
            let players_percentage = teams_percentage / players_raw_data.len() as f64;
            for player_raw in players_raw_data {
                let player = Player::scrape_player_element(&player_raw);

                terminal_helper.push_player_item(player.name.clone());
                terminal_helper.add_percentage(players_percentage);

                team.players.insert(player.name.clone(), player);
            }

            league.teams.insert(team.name.clone(), team);
            // Close current tab
            driver.close().expect("Error while closing browser tab");
            switch_to_tab(&driver, 1);
        }

        scrapping_data.insert(league.name.clone(), league);
        // Close current tab
        driver.close().expect("Error while closing browser tab");
        switch_to_tab(&driver, 0);
    }

    // 95% finish aprox
    let scrapping_data_json =
        serde_json::to_string(&scrapping_data).expect("Error while serializing data to JSON");

    file.write_all(scrapping_data_json.as_bytes())
        .expect("Error while saving the scrapping data");

    // 100% finish
    terminal_helper.set_percentage(100.0);
}

fn open_new_tab(driver: &Driver) {
    driver
        .execute_script(r#"window.open("about:blank", target="_blank");"#)
        .expect("Error while executing browser script");
}

fn switch_to_tab(driver: &Driver, tab_index: usize) {
    let handles = driver.window_handles().expect("Error while getting tabs");
    driver
        .switch_to()
        .window(&handles[tab_index])
        .expect("Error while exectuing browser script");
}
