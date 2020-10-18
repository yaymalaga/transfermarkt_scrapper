use thirtyfour_sync::{By, WebDriverCommands, WebElement};

use crate::driver::Driver;
use crate::team::Team;

const LEAGUES_URL: &str = "https://www.transfermarkt.com/wettbewerbe/europa";

#[derive(Debug)]
pub struct League {
    pub name: String,
    pub url: String,
    pub logo_url: String,
    pub teams: Vec<Team>,
}

impl League {
    pub fn new(name: String, url: String, logo_url: String, teams: Vec<Team>) -> Self {
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

        Self::new(name, url, logo_url, vec![])
    }

    pub fn get_leagues_raw_data(driver: &Driver) -> Vec<WebElement> {
        driver
            .get(LEAGUES_URL)
            .expect("Error while loading leagues page");

        driver
            .find_elements(By::XPath("//div[@id='yw1']//td[@class='hauptlink']//tr"))
            .expect("Error while getting leagues data");
    }
}

#[cfg(test)]
mod tests {
    use thirtyfour_sync::{DesiredCapabilities, WebDriver};

    use super::*;

    #[test]
    fn scrape_every_league() {
        let caps = DesiredCapabilities::chrome();
        let driver =
            WebDriver::new("http://localhost:4444", &caps).expect("ChromeDriver not available");

        assert_eq!(League::scrape_leagues_basic(&driver, None).len(), 25);
    }

    #[test]
    fn scrape_whitelist() {
        let caps = DesiredCapabilities::chrome();
        let driver =
            WebDriver::new("http://localhost:4444", &caps).expect("ChromeDriver not available");

        let mut scrapping_result =
            League::scrape_leagues_basic(&driver, Some(vec!["LaLiga", "TEST"]));
        assert_eq!(scrapping_result.len(), 1);

        League::scrape_league_teams_basic(&driver, &mut scrapping_result[0]);
        assert_eq!(scrapping_result[0].teams.len(), 20);
    }
}
