use crate::github;
use crate::bugzilla;

pub fn run(token: String) {
    println!("Getting GitHub issues...");
    let auth_value = format!("Bearer {}", token);
    let issues = github::get_issues(auth_value).unwrap();
    println!("{:?}", issues);

    println!("Getting Bugzilla bugs...");
    let bugs = bugzilla::get_bugs().unwrap();
    let assigned_bugs = bugs.get(&String::from("bugs"));
    println!("{:?}", assigned_bugs);
}