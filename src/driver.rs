use thirtyfour_sync::{GenericWebDriver, http::reqwest_sync::ReqwestDriverSync};

pub type Driver = GenericWebDriver<ReqwestDriverSync>;