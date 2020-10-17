use thirtyfour_sync::prelude::*;

mod league;
mod team;
mod player;

fn main() {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).unwrap();
}
