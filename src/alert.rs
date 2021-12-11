use std::collections::HashMap;
use std::ops::{Add, Sub};
use chrono::{Duration, TimeZone, Local, DateTime};
use serde_json::{json, Value};
use tera::{Context, Tera};
use crate::message::send_message;
use crate::config::Config;
use crate::ones::client;
use anyhow::{Result, Error};

pub async fn alert(conf: &Config) -> Result<()> {
    let mut c = client::new(conf.base_api.as_ref().unwrap().to_string(),
                            conf.email.clone(),
                            conf.password.clone(),
                            conf.team_uuid.clone());
    c.auth().await?;

    let end_time = Local::now();
    let start_time = end_time.sub(Duration::days(i64::from(conf.days.unwrap())));
    let dates = gen_dates(&start_time, &end_time);
    let members = conf.members.as_ref().unwrap();

    let end_time = end_time.format("%Y-%m-%d").to_string();
    let start_time = start_time.format("%Y-%m-%d").to_string();
    println!("start_time: {}\nend_time: {}", start_time, end_time);
    let manhours = c.query_manhours(
        String::from(start_time),
        String::from(end_time),
        members).await?;
    let content = gen_content(dates, manhours.as_ref())?;
    let mut mentioned_list: Vec<String> = Vec::new();
    if conf.at_all.unwrap() {
        mentioned_list.push(String::from("@all"));
    }
    send_message(conf.alert_webhook.clone(),
                 content.as_str(),
                 Some(mentioned_list),
                 None).await
}

fn gen_dates(start_time: &DateTime<Local>, end_time: &DateTime<Local>) -> Vec<String> {
    let mut dates: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        let t = start_time.add(Duration::days(i));
        if t.gt(&end_time) {
            break;
        }
        dates.push(t.format("%Y-%m-%d").to_string());
        i += 1;
    }
    dates
}

fn gen_content(dates: Vec<String>, manhours: &Vec<Value>) -> Result<String> {
    let mut user_manhours: HashMap<String, HashMap<String, f64>> = HashMap::new();
    for m in manhours {
        let owner_name = m["owner"]["name"].as_str().ok_or(Error::msg("owner_name not found"))?;
        let start_time = m["startTime"].as_i64().ok_or(Error::msg("start_time not found"))?;
        let start_time = Local.timestamp(start_time, 0);
        let start_time = start_time.format("%Y-%m-%d").to_string();

        let hours = m["hours"].as_f64().ok_or(Error::msg("hours not found"))?;
        let hours = hours / f64::from(100000);

        if user_manhours.contains_key(owner_name) {
            let ms = user_manhours.get(owner_name).ok_or(Error::msg("user_manhours not found"))?;
            let mut ms = ms.clone();
            if ms.contains_key(&start_time) {
                let s = ms.get(&start_time).ok_or(Error::msg("get start_time error"))?;
                let s = s.clone();
                ms.insert(start_time, s + hours);
            } else {
                ms.insert(start_time, hours);
            }
            user_manhours.insert(owner_name.to_string(), ms);
            continue;
        }

        let ms = HashMap::from([(start_time, hours)]);
        user_manhours.insert(owner_name.to_string(), ms);
    }

    let mut user_manhours_sorted: HashMap<String, Vec<f64>> = HashMap::new();
    for (k, v) in user_manhours {
        let mut hours_sorted: Vec<f64> = Vec::new();
        for d in dates.iter() {
            if v.contains_key(d) {
                hours_sorted.push(v.get(d).unwrap().clone());
            } else {
                hours_sorted.push(0.0);
            }
        }
        user_manhours_sorted.insert(k, hours_sorted);
    }

    let mut t = Tera::default();
    let message_tpl = r#"工时提醒：请同学们根据工时记录检查是否及时登记工时~
{% for d in dates %}{{ d }}🕙{% endfor %}
{% for user, manhours in user_manhours %}{{ user }} {% for h in manhours %}{% if loop.index == 1 %}{{ h }}{% else %} / {{ h }} {% endif %}{% endfor %}
{% endfor %}"#;
    t.add_raw_template("message", message_tpl)?;
    let context = Context::from_serialize(json!({
        "dates": dates,
        "user_manhours": user_manhours_sorted,
    }))?;
    let content = t.render("message", &context)?;
    Ok(content)
}