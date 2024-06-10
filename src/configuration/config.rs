use serde::Serialize;

use crate::configuration::app::App;
use crate::configuration::package::Package;

#[derive(Debug, Serialize)]
pub struct Config {
    pub apps: Vec<App>,
    pub packages: Vec<Package>,
}
