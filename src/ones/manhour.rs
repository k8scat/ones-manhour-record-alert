use serde_json::{json, Value};
use super::client::Client;
use anyhow::{Result, Error};

impl Client {
    pub async fn query_manhours(&self, start_time: String, end_time: String, owners: &Vec<String>) -> Result<Vec<Value>> {
        let query = r#"
        query QUERY_MANHOURS (
            $filter: Filter
        ) {
            manhours (
                filter: $filter
            ) {
                hours
                startTime
                owner {
                    name
                }
            }
        }
        "#;
        let variables = json!({
            "filter": {
                "startTime_range": {
                    "gte": start_time,
                    "lte": end_time
                },
                "type_equal": "recorded",
                "owner": {
                    "name_in": owners
                }
            }
        });
        let result = self.graphql(String::from(query), &variables).await?;
        let manhours: Value = serde_json::from_str(result.as_str())?;
        let manhours = manhours["data"]["manhours"].as_array().ok_or(Error::msg("manhours not found"))?;
        Ok(manhours.to_vec())
    }
}