use crate::reqwest;

use reqwest::header::AUTHORIZATION;

const URL: &str = "https://api.github.com/user/issues";

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    html_url: String,
    title: String,
}

pub fn get_issues(auth_value: String) -> Result<Vec<Issue>, Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let res: Vec<Issue> = client.get(URL)
        .header(AUTHORIZATION, auth_value)
        .send()?
        .json()?;

    Ok(res)
}