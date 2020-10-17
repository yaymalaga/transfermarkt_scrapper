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
        Self {
            name,
            url,
            logo_url,
            teams,
        }
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

            match &whitelist {
                Some(leagues_list) => {
                    if !leagues_list.iter().any(|&x| x == &name) {
                        continue;
                    }
                }
                None => (),
            }

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

#[cfg(test)]
mod tests {
    use thirtyfour_sync::{DesiredCapabilities, WebDriver};

    use super::*;

    #[test]
    fn scrape_every_league() {
        let caps = DesiredCapabilities::chrome();
        let driver =
            WebDriver::new("http://localhost:4444", &caps).expect("ChromeDriver not available");
        assert_eq!(League::leagues_data_scrapping(&driver, None).len(), 25);
    }

    #[test]
    fn scrape_whitelist() {
        let caps = DesiredCapabilities::chrome();
        let driver =
            WebDriver::new("http://localhost:4444", &caps).expect("ChromeDriver not available");
        assert_eq!(
            League::leagues_data_scrapping(&driver, Some(vec!["LaLiga", "TEST"])).len(),
            1
        );
    }
}
