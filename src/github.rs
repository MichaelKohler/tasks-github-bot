use crate::reqwest;

use std::collections::HashMap;
use reqwest::header::AUTHORIZATION;

use serde_json::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Issue {
    html_url: String,
    title: String,
}

pub fn get_issues(auth_value: String) -> Result<(), Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let res: String = client.get("https://api.github.com/user/issues")
        .header(AUTHORIZATION, auth_value)
        .send()?
        .text()?;

    let array: Vec<Issue> = serde_json::from_str(&res)?;

    for elem in array.iter() {
        println!("{:?}", elem);
    }

    Ok(())
}