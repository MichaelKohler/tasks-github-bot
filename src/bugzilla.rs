use crate::reqwest;

use std::collections::HashMap;

const URL: &str = "https://bugzilla.mozilla.org/rest/bug?assigned_to=me@michaelkohler.info&resolution=---";

#[derive(Serialize, Deserialize, Debug)]
pub struct Bug {
    summary: String,
    id: u32,
}

pub fn get_bugs() -> Result<HashMap<String, Vec<Bug>>, Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let res: HashMap<String, Vec<Bug>> = client.get(URL)
        .send()?
        .json()?;

    Ok(res)
}