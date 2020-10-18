use thirtyfour_sync::{By, WebDriverCommands, WebElement};

use crate::driver::Driver;
use crate::league::League;
use crate::player::Player;

#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub url: String,
    pub logo_url: String,
    pub players: Vec<Player>,
}

impl Team {
    pub fn new(name: String, url: String, logo_url: String, players: Vec<Player>) -> Self {
        Self {
            name,
            url,
            logo_url,
            players,
        }
    }

    pub fn scrape_team_element(team_element: &WebElement) -> Self {
        let team_data = team_element
            .find_element(By::XPath("./td[2]/a[1]"))
            .expect("Team data not found");

        let name = team_data
            .get_attribute("innerHTML")
            .expect("Team name was not found");

        let url = team_data
            .get_attribute("href")
            .expect("Team link was not found");

        let logo_url = team_element
            .find_element(By::XPath("./td[1]//img[1]"))
            .expect("Team image was not found")
            .get_attribute("src")
            .expect("Team image url was not found");

        Self::new(name, url, logo_url, vec![])
    }

    pub fn get_teams_raw_data(driver: &Driver, league: &League) -> Vec<WebElement> {
        driver
            .get(&league.url)
            .expect(&format!("Error while loading {} league page", &league.name));

        driver
            .find_elements(By::XPath("//div[@id='yw1']//tbody//tr"))
            .expect("Error while getting teams data")
    }
}
