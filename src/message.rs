use serde_json::json;
use anyhow::{Result, Error};

pub async fn send_message(webhook: String,
                          content: &str,
                          mentioned_list: Option<Vec<String>>,
                          mentioned_mobile_list: Option<Vec<String>>) -> Result<()> {
    let client = reqwest::Client::new();
    let payload = json!({
        "msgtype": "text",
        "text": {
            "content": content,
            "mentioned_list": mentioned_list,
            "mentioned_mobile_list": mentioned_mobile_list
        }
    });
    let resp = client.post(webhook)
        .json(&payload)
        .send()
        .await?;
    if !resp.status().is_success() {
        let content = resp.text().await?;
        Err(Error::msg(format!("send message failed: {}", content)))
    } else {
        Ok(())
    }
}