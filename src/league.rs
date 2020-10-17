use thirtyfour_sync::{By, WebDriverCommands, WebElement};

use crate::team::Team;
use crate::driver::Driver;

const LEAGUES_URL: &str = "https://www.transfermarkt.com/wettbewerbe/europa";

pub struct League {
    name: String,
    logo_url: String,
    league_url: String,
    teams: Vec<Team>,
}

impl League {
    pub fn scrape_leagues(driver: &Driver, whitelist: Option<Vec<&str>>) -> Vec<WebElement>{
        driver.get(LEAGUES_URL).expect("Error while loading leagues page");

        let leagues_data = driver
            .find_elements(By::XPath("//div[@id='yw1']//td[@class='hauptlink']//tr"))
            .expect("Error while getting leagues data");
    }
}
