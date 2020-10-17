use thirtyfour_sync::{By, WebDriverCommands};

use crate::driver::Driver;
use crate::team::Team;

const LEAGUES_URL: &str = "https://www.transfermarkt.com/wettbewerbe/europa";

pub struct League {
    name: String,
    url: String,
    logo_url: String,
    teams: Vec<Team>,
}

impl League {
    pub fn new(name: String, url: String, logo_url: String, teams: Vec<Team>) -> Self {
        Self { name, url, logo_url, teams }
    }

    pub fn leagues_data_scrapping<'a>(
        driver: &'a Driver,
        whitelist: Option<Vec<&str>>,
    ) -> Vec<League> {
        driver
            .get(LEAGUES_URL)
            .expect("Error while loading leagues page");

        let leagues_raw_data = driver
            .find_elements(By::XPath("//div[@id='yw1']//td[@class='hauptlink']//tr"))
            .expect("Error while getting leagues data");

        let mut leagues: Vec<League> = vec![];

        for league in &leagues_raw_data {
            let league_data = league
                .find_element(By::XPath("./td[2]/a[1]"))
                .expect("League data not found");

            let name = league_data.text().expect("League name was not found");
            let url = league_data
                .get_attribute("href")
                .expect("League link was not found");

            let logo_url = league
                .find_element(By::XPath("./td[1]//img[1]"))
                .expect("League image was not found")
                .get_attribute("src")
                .expect("League image url was not found");

            leagues.push(League::new(name, url, logo_url, vec![]))
        }

        leagues
    }
}
