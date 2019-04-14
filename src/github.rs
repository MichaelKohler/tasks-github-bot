use crate::reqwest;

use reqwest::header::AUTHORIZATION;

#[derive(Serialize, Deserialize, Debug)]
struct Issue {
    html_url: String,
    title: String,
}

pub fn get_issues(auth_value: String) -> Result<(), Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let res: Vec<Issue> = client.get("https://api.github.com/user/issues")
        .header(AUTHORIZATION, auth_value)
        .send()?
        .json()?;

    println!("{:?}", res);

    Ok(())
}