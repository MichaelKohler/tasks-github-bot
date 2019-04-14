use crate::reqwest;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bug {
    summary: String,
    id: u32,
}

pub fn get_bugs() -> Result<HashMap<String, Vec<Bug>>, Box<std::error::Error>> {
    let client = reqwest::Client::new();
    let res: HashMap<String, Vec<Bug>> = client.get("https://bugzilla.mozilla.org/rest/bug?assigned_to=me@michaelkohler.info&resolution=---")
        .send()?
        .json()?;

    Ok(res)
}