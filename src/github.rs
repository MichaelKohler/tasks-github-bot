use crate::reqwest;
use crate::bugzilla;

use reqwest::header::AUTHORIZATION;
use std::collections::HashMap;
use std::env;
use std::fmt;

const GITHUB_OWNER_ENV_NAME: &str = "GITHUB_OWNER";
const GITHUB_REPO_ENV_NAME: &str = "GITHUB_REPO";
const GITHUB_TOKEN_ENV_NAME: &str = "GITHUB_TOKEN";
const URL: &str = "https://api.github.com/user/issues?per_page=100";

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    pub html_url: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskIssue {
    title: String,
    body: String,
}

impl fmt::Display for TaskIssue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.title, self.body)
    }
}

pub fn get_issues() -> Result<Vec<Issue>, Box<std::error::Error>> {
    let (github_owner, github_repo) = get_env();
    let tasks_repo_slug = format!("{}/{}", github_owner, github_repo);
    let auth_value = get_auth();
    let client = reqwest::Client::new();
    debug!("Getting GitHub issues from {}", URL);
    let res: Vec<Issue> = client.get(URL)
        .header(AUTHORIZATION, auth_value)
        .send()?
        .json()?;
    info!("Got {} GitHub issues including already existing tasks", res.len());
    let non_task_issues: Vec<Issue> = res.into_iter()
        .filter(|issue| !issue.html_url.contains(&tasks_repo_slug))
        .collect();;
    info!("Got {} non task repo GitHub issues", non_task_issues.len());
    Ok(non_task_issues)
}

pub fn create_issue_from_bugzilla(bug: &bugzilla::Bug) {
    info!("Creating issue for {}", bug.id);
    let issue = TaskIssue {
        title: format!("{}", bug.summary), // TODO: fix ownership for real...
        body: format!("https://bugzilla.mozilla.org/show_bug.cgi?id={}", bug.id),
    };
    create_issue(issue).unwrap();
}

pub fn create_issue_from_github(issue: &Issue) {
    info!("Creating issue for {}", issue.html_url);
    let issue = TaskIssue {
        title: format!("{}", issue.title), // TODO: fix ownership for real...
        body: format!("https://bugzilla.mozilla.org/show_bug.cgi?id={}", issue.html_url),
    };
    create_issue(issue).unwrap();
}

fn create_issue(issue: TaskIssue) -> Result<Issue, Box<std::error::Error>> {
    let (github_owner, github_repo) = get_env();
    let auth_value = get_auth();
    debug!("Creating issue in {}/{}", github_owner, github_repo);
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/issues", github_owner, github_repo);
    let mut params = HashMap::new();
    params.insert("title", issue.title);
    params.insert("body", issue.body);
    let res: Issue = client.post(&url)
        .header(AUTHORIZATION, auth_value)
        .json(&params)
        .send()?
        .json()?;
    info!("Created GitHub issue: {}", res.html_url);
    Ok(res)
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

fn get_auth() -> String {
    let token = match env::var(&GITHUB_TOKEN_ENV_NAME) {
      Ok(val) => val,
      Err(_e) => {
        error!("No GITHUB_TOKEN set");
        String::new()
      },
    };

    let auth_value = format!("Bearer {}", token);
    return auth_value;
}