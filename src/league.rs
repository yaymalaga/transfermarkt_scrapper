use std::collections::HashMap;

use serde::Serialize;
use thirtyfour_sync::{By, WebDriverCommands, WebElement};

use crate::driver::Driver;
use crate::team::Team;

const LEAGUES_URL: &str = "https://www.transfermarkt.com/wettbewerbe/europa";

#[derive(Serialize)]
pub struct League {
    pub name: String,
    pub url: String,
    pub logo_url: String,
    pub teams: HashMap<String, Team>,
}

impl League {
    pub fn new(name: String, url: String, logo_url: String, teams: HashMap<String, Team>) -> Self {
        Self {
            name,
            url,
            logo_url,
            teams,
        }
    }

    pub fn scrape_league_element(league_element: &WebElement) -> Self {
        let league_data = league_element
            .find_element(By::XPath("./td[2]/a[1]"))
            .expect("League data not found");

        let name = league_data.text().expect("League name was not found");

        let url = league_data
            .get_attribute("href")
            .expect("League link was not found");

        let logo_url = league_element
            .find_element(By::XPath("./td[1]//img[1]"))
            .expect("League image was not found")
            .get_attribute("src")
            .expect("League image url was not found");

        Self::new(name, url, logo_url, HashMap::new())
    }

    pub fn get_leagues_raw_data<'a>(driver: &'a Driver) -> Vec<WebElement<'a>> {
        driver
            .get(LEAGUES_URL)
            .expect("Error while loading leagues page");

        driver
            .find_elements(By::XPath("//div[@id='yw1']//td[@class='hauptlink']//tr"))
            .expect("Error while getting leagues data")
    }
}