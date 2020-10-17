use thirtyfour_sync::{By, WebDriverCommands, WebElement};

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

        League::new(name, url, logo_url, vec![])
    }

    pub fn scrape_leagues_basic<'a>(
        driver: &'a Driver,
        whitelist: Option<Vec<&str>>,
    ) -> Vec<League> {
        driver
            .get(LEAGUES_URL)
            .expect("Error while loading leagues page");

        let leagues_raw_data = driver
            .find_elements(By::XPath("//div[@id='yw1']//td[@class='hauptlink']//tr"))
            .expect("Error while getting leagues data");

        leagues_raw_data
            .iter()
            .map(|x| Self::scrape_league_element(x))
            .filter(|x| match &whitelist {
                Some(leagues_list) => leagues_list.iter().any(|&i|i == x.name),
                None => true,
            })
            .collect()
    }

    fn scrape_league_teams_basic(league_data: &mut League) {
        league_data.teams = Team::scrape_teams_data(&league_data);
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
        assert_eq!(
            League::scrape_leagues_basic(&driver, Some(vec!["LaLiga", "TEST"])).len(),
            1
        );
    }
}
