use config_macro::ConfigHelper;
use confy::ConfyError;
use serde_derive::{Serialize, Deserialize};

const APP_NAME: &str = "millefeuille";

#[derive(ConfigHelper, Serialize, Deserialize)]
pub struct Config {
    iso: i32,
    aperture: i32,
    shutter_speed: i32,
    hdr_bracketing: i16,
    count: i16,
    output_dir: String
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self{
            iso: -1,
            aperture: -1,
            shutter_speed: -1,
            hdr_bracketing: 0,
            count: 10,
            output_dir: "".into() // TODO default dir
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