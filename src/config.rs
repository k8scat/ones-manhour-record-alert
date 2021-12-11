use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Error};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub base_api: Option<String>,
    pub email: String,
    pub password: String,
    pub team_uuid: Option<String>,
    pub members: Option<Vec<String>>,
    pub days: Option<u16>,
    pub alert_webhook: String,
    pub at_all: Option<bool>
}

pub fn load_config(path: &str) -> Result<Config> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}

impl Config {
    pub fn validate(&mut self) -> Result<()> {
        if self.alert_webhook.is_empty() {
            return Err(Error::msg("no alert webhook provided"));
        }
        if self.email.is_empty() {
            return Err(Error::msg("no email provided"));
        }
        if self.password.is_empty() {
            return Err(Error::msg("no password provided"));
        }
        if self.members.is_none() || self.members.as_ref().unwrap().is_empty() {
            return Err(Error::msg("no members specified"));
        }
        if self.base_api.is_none() || self.base_api.as_ref().unwrap().is_empty() {
            self.base_api = Some(String::from("https://ones.ai/project/api"));
        }
        if self.days.is_none() || self.days.unwrap() == 0 {
            self.days = Some(7);
        }
        if self.at_all.is_none() {
            self.at_all = Some(true);
        }
        Ok(())
    }
}
