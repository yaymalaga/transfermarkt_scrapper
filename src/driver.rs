use thirtyfour_sync::{http::reqwest_sync::ReqwestDriverSync, GenericWebDriver};

pub type Driver = GenericWebDriver<ReqwestDriverSync>;
