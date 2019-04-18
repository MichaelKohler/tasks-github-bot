use crate::reqwest;

use std::collections::HashMap;
use std::env;

const ENV_NAME: &str = "BUGZILLA_EMAIL";

#[derive(Serialize, Deserialize, Debug)]
pub struct Bug {
    pub summary: String,
    pub id: u32,
}

pub fn get_bugs() -> Result<HashMap<String, Vec<Bug>>, Box<std::error::Error>> {
    let email = match env::var(&ENV_NAME) {
      Ok(val) => val,
      Err(_e) => {
        error!("No {} set", ENV_NAME);
        panic!("ENV MISSING, ABORTING");
      },
    };

    let formatted_url = format!("https://bugzilla.mozilla.org/rest/bug?assigned_to={}&resolution=---", email);

    let client = reqwest::Client::new();
    debug!("Getting Bugzilla bugs from {}", formatted_url);
    let res: HashMap<String, Vec<Bug>> = client.get(&formatted_url)
        .send()?
        .json()?;
    info!("Got {} Bugzilla bugs", res.len());
    Ok(res)
}