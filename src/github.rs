use crate::reqwest;
use crate::bugzilla;

use reqwest::header::AUTHORIZATION;
use std::env;

const GITHUB_OWNER_ENV_NAME: &str = "GITHUB_OWNER";
const GITHUB_REPO_ENV_NAME: &str = "GITHUB_REPO";
const URL: &str = "https://api.github.com/user/issues";

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    pub html_url: String,
    pub title: String,
}

pub fn get_issues(auth_value: String) -> Result<Vec<Issue>, Box<std::error::Error>> {
    let client = reqwest::Client::new();
    debug!("Getting GitHub issues from {}", URL);
    let res: Vec<Issue> = client.get(URL)
        .header(AUTHORIZATION, auth_value)
        .send()?
        .json()?;
    info!("Got {} GitHub issues", res.len());
    Ok(res)
}

pub fn create_issue_from_bugzilla(bug: &bugzilla::Bug) {
    let (github_owner, github_repo) = get_env();

    info!("Creating issue in {}/{} for {}", github_owner, github_repo, bug.id);
    error!("Missing implementation..");
}

pub fn create_issue_from_github(issue: &Issue) {
    let (github_owner, github_repo) = get_env();

    info!("Creating issue in {}/{} for {}", github_owner, github_repo, issue.html_url);
    error!("Missing implementation..");
}

fn get_env() -> (String, String) {
    let github_owner: String = match env::var(&GITHUB_OWNER_ENV_NAME) {
      Ok(val) => val,
      Err(_e) => {
        error!("No {} set", GITHUB_OWNER_ENV_NAME);
        panic!("ENV MISSING, ABORTING");
      },
    };

    let github_repo: String = match env::var(&GITHUB_REPO_ENV_NAME) {
      Ok(val) => val,
      Err(_e) => {
        error!("No {} set", GITHUB_REPO_ENV_NAME);
        panic!("ENV MISSING, ABORTING");
      },
    };

    return (github_owner, github_repo);
}