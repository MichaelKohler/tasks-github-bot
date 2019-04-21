use crate::database;
use crate::github;
use crate::models::{Bug,Issue};

use std::collections::HashMap;

// TODO: use only one function here
pub fn update_bugzilla(all_bugs: &Vec<Bug>) {
    let known_entries = database::get_data().unwrap();
    info!("Found {} known Bugzilla entries", known_entries.bugzilla.len());
    let mut created_issues = HashMap::new();
    for bug in all_bugs.iter() {
        let bug_url = format!("https://bugzilla.mozilla.org/show_bug.cgi?id={}", bug.id);
        if known_entries.bugzilla.contains(&bug_url) {
            continue;
        }

        info!("Untracked Bug: {:?}", bug);
        let created_issue = github::create_issue(bug).unwrap();
        created_issues.insert(created_issue.id, created_issue.body);
    }

    if created_issues.len() > 0 {
        database::add_created_bugzilla_issues(created_issues);
    }
}

// TODO: use only one function here
pub fn update_github(all_issues: Vec<Issue>) {
    let known_entries = database::get_data().unwrap();
    info!("Found {} known GitHub entries", known_entries.github.len());
    let mut created_issues = HashMap::new();
    for issue in all_issues.iter() {
        if known_entries.github.contains(&issue.html_url) {
            continue;
        }

        info!("Untracked Issue: {:?}", issue);
        let created_issue = github::create_issue(issue).unwrap();
        created_issues.insert(created_issue.id, created_issue.body);
    }

    if created_issues.len() > 0 {
        database::add_created_github_issues(created_issues);
    }
}
