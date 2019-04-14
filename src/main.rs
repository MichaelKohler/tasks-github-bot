extern crate tasks_github_bot;

use std::env;
use tasks_github_bot::github;
use tasks_github_bot::bugzilla;

fn main() {
    let token = match env::var("GITHUB_TOKEN") {
      Ok(val) => val,
      Err(_e) => String::new(),
    };

    println!("Getting GitHub issues...");
    let auth_value = format!("Bearer {}", token);
    let issues = github::get_issues(auth_value).unwrap();
    println!("{:?}", issues);

    println!("Getting Bugzilla bugs...");
    let bugs = bugzilla::get_bugs().unwrap();
    let assigned_bugs = bugs.get(&String::from("bugs"));
    println!("{:?}", assigned_bugs);
}
