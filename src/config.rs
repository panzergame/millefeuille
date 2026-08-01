use config_macro::ConfigHelper;
use confy::ConfyError;
use serde_derive::{Serialize, Deserialize};

const APP_NAME: &str = "millefeuille";

#[derive(ConfigHelper, Serialize, Deserialize)]
pub struct Config {
    iso: i32,
    count: i16
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self{
            iso: -1,
            count: 10
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, ConfyError> {
        confy::load::<Self>(APP_NAME, None)
    }

    pub fn save(&self) -> Result<(), ConfyError> {
        confy::store(APP_NAME, None, self)
    }
}