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

    let mut leagues_data = League::scrape_leagues_basic(&driver, None);

    for league in leagues_data.iter_mut() {
        League::scrape_league_teams_basic(&driver, league);

        for team in league.teams.iter_mut() {
            Team::scrape_team_players(&driver, team);
            println!("{:#?}", leagues_data);
            panic!();
        }
    }
}
