use thirtyfour_sync::prelude::*;

mod driver;
mod league;
mod team;
mod player;

use crate::league::League;

fn main() {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).unwrap();
    
    let leagues_raw_data = League::scrape_leagues_basic(&driver, None);
    println!("{}", leagues_raw_data.len())
}
