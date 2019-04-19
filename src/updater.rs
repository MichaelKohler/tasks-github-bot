use crate::bugzilla;
use crate::database;
use crate::github;

use std::collections::HashMap;

pub fn update_bugzilla(all_bugs: &Vec<bugzilla::Bug>) {
    let known_entries = database::get_data().unwrap();
    info!("Found {} known Bugzilla entries", known_entries.bugzilla.len());
    let mut created_issues = HashMap::new();
    for bug in all_bugs.iter() {
        let bug_url = format!("https://bugzilla.mozilla.org/show_bug.cgi?id={}", bug.id);
        if known_entries.bugzilla.contains(&bug_url) {
            continue;
        }

        info!("Untracked Bug: {:?}", bug);
        let created_issue = github::create_issue_from_bugzilla(bug).unwrap();
        created_issues.insert(created_issue.id, created_issue.body);
    }

    if created_issues.len() > 0 {
        database::add_created_bugzilla_issues(created_issues);
    }
}

pub fn update_github(all_issues: Vec<github::Issue>) {
    let known_entries = database::get_data().unwrap();
    info!("Found {} known GitHub entries", known_entries.github.len());
    let mut created_issues = HashMap::new();
    for issue in all_issues.iter() {
        if known_entries.github.contains(&issue.html_url) {
            continue;
        }

        info!("Untracked Issue: {:?}", issue);
        let created_issue = github::create_issue_from_github(issue).unwrap();
        created_issues.insert(created_issue.id, created_issue.body);
    }

    if created_issues.len() > 0 {
        database::add_created_github_issues(created_issues);
    }
}
