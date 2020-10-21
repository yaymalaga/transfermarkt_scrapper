use serde::Serialize;
use thirtyfour_sync::{By, WebDriverCommands, WebElement};

use crate::{driver::Driver, team::Team};

#[derive(Serialize)]
pub struct Player {
    pub name: String,
    pub photo_url: String,
    pub role: String,
    pub price: String,
}

impl Player {
    pub fn new(name: String, photo_url: String, role: String, price: String) -> Self {
        Self {
            name,
            photo_url,
            role,
            price,
        }
    }

    pub fn scrape_player_element(player_element: &WebElement) -> Self {
        let player_data = player_element
            .find_element(By::XPath(".//td[@class='posrela']"))
            .expect("Player data not found");

        let player_data_details = player_data
            .find_element(By::XPath(".//tr[1]"))
            .expect("Player extra data not found");

        let name = player_data_details
            .find_element(By::XPath(".//td[@class='hauptlink']//a[1]"))
            .expect("Player name was not found")
            .get_attribute("innerHTML")
            .expect("Player name text was not found");

        let photo_url = player_data_details
            .find_element(By::XPath(".//img[1]"))
            .expect("Player image was not found")
            .get_attribute("data-src")
            .expect("Player image url was not found");

        let role = player_data
            .find_element(By::XPath(".//tr[2]/td"))
            .expect("Player role was not found")
            .get_attribute("innerHTML")
            .expect("Player role text was not found");

        let price = player_element
            .find_element(By::XPath("./td[@class='rechts hauptlink']"))
            .expect("Player price was not found")
            .text()
            .expect("Player price text was not found")
            .trim_end()
            .to_string();

        Self::new(name, photo_url, role, price)
    }

    pub fn get_players_raw_data<'a>(driver: &'a Driver, team: &Team) -> Vec<WebElement<'a>> {
        driver
            .get(&team.url)
            .expect(&format!("Error while loading {} team page", &team.name));

        driver
            .execute_script("window.scrollTo(0, document.body.scrollHeight/3);")
            .expect("Error while executing scrolling script");

        driver
            .find_elements(By::XPath("//div[@id='yw1']/table/tbody/tr"))
            .expect("Error while getting teams data")
    }
}
