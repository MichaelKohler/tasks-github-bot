use crate::github;
use crate::bugzilla;
use crate::updater;

pub fn run(token: String) {
    println!("Getting GitHub issues...");
    let auth_value = format!("Bearer {}", token);
    let issues = github::get_issues(auth_value).unwrap();
    updater::update_github(issues);

    println!("Getting Bugzilla bugs...");
    let bugs = bugzilla::get_bugs().unwrap();
    let assigned_bugs = bugs.get(&String::from("bugs")).unwrap();
    updater::update_bugzilla(assigned_bugs);
}