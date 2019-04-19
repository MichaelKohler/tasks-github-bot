use crate::github;
use crate::bugzilla;
use crate::updater;

pub fn run() {
    let issues = github::get_issues().unwrap();
    updater::update_github(issues);

    let bugs = bugzilla::get_bugs().unwrap();
    let assigned_bugs = bugs.get(&String::from("bugs")).unwrap();
    updater::update_bugzilla(assigned_bugs);
}