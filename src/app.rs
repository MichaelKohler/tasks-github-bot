use crate::github;
use crate::bugzilla;
use crate::updater;

pub fn run() {
    let issues = github::get_issues().unwrap();
    updater::update(issues, "github");

    let bugs = bugzilla::get_bugs().unwrap();
    updater::update(bugs, "bugzilla");
}