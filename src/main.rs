extern crate tasks_github_bot;

use std::env;
use tasks_github_bot::github;

fn main() {
    let token = match env::var("GITHUB_TOKEN") {
      Ok(val) => val,
      Err(_e) => String::new(),
    };

    println!("Getting GitHub issues...");
    let auth_value = format!("Bearer {}", token);
    github::get_issues(auth_value);
}
