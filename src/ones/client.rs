use std::str::FromStr;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::{json, Value};
use serde::{Serialize};
use anyhow::{Result, Error};

#[derive(Debug)]
pub struct Client {
    base_api: String,
    email: String,
    password: String,
    team_uuid: Option<String>,
    client: Option<reqwest::Client>,
}

pub fn new(base_api: String, email: String, password: String, team_uuid: Option<String>) -> Client {
    Client {
        base_api,
        email,
        password,
        client: None,
        team_uuid,
    }
}

impl Client {
    pub async fn auth(&mut self) -> Result<()> {
        let client = reqwest::Client::new();
        let payload = json!({
            "email": self.email,
            "password": self.password
        });
        let resp = client.post(format!("{}/project/auth/login", self.base_api).as_str())
            .json(&payload)
            .send()
            .await?;
        let content = resp.text().await?;
        let data: Value = serde_json::from_str(content.as_str())?;

        let user_uuid = data["user"]["uuid"].as_str().ok_or(Error::msg("user uuid not found"))?;
        let token = data["user"]["token"].as_str().ok_or(Error::msg("token not found"))?;

        if self.team_uuid.is_none() {
            let teams = data["teams"].as_array().ok_or(Error::msg("teams not found"))?;
            if teams.len() != 1 {
                println!("found {} teams:", teams.len());
                for (i, team) in teams.iter().enumerate() {
                    let team_uuid = team["uuid"].as_str().ok_or(Error::msg("team uuid not found"))?;
                    let team_name = team["name"].as_str().ok_or(Error::msg("team name not found"))?;
                    println!("{}. {} (uuid: {})", i + 1, team_name, team_uuid);
                }
                return Err(Error::msg("found none or more than one team"));
            }
            let team_uuid = teams[0]["uuid"].as_str().ok_or(Error::msg("team uuid not found"))?;
            self.team_uuid = Some(team_uuid.to_string());
        }

        let headers = HeaderMap::from_iter(vec![
            (HeaderName::from_str("Ones-User-ID").unwrap(), HeaderValue::from_str(user_uuid)?),
            (HeaderName::from_str("Ones-Auth-Token").unwrap(), HeaderValue::from_str(token)?),
        ]);
        let client = reqwest::ClientBuilder::new().default_headers(headers).build()?;
        self.client = Some(client);
        Ok(())
    }

    pub async fn graphql<T: Serialize + ?Sized>(&self, query: String, variables: &T) -> Result<String> {
        let client = self.client.as_ref().ok_or(Error::msg("client not found"))?;
        let payload = json!({
            "query": query,
            "variables": variables
        });
        let url = format!("{}/project/team/{}/items/graphql",
                          self.base_api,
                          self.team_uuid.as_ref().ok_or(Error::msg("team uuid not found"))?);
        let resp = client.post(url)
            .json(&payload)
            .send().await?;

        let ok = resp.status().is_success();
        let content = resp.text().await?;
        if ok {
            Ok(content)
        } else {
            Err(Error::msg(format!("graphql request failed: {}", content)))
        }
    }
}


