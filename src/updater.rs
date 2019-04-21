use crate::database;
use crate::github;
use crate::models::Taskable;

use std::collections::HashMap;

pub fn update(all_issues: Vec<impl Taskable>, accessor: &str) {
    let known_entries = database::get_data().unwrap();
    info!("Found {} known entries", known_entries[accessor].len());
    let mut created_issues = HashMap::new();
    for issue in all_issues.iter() {
        if known_entries[accessor].contains(&issue.get_id()) {
            continue;
        }

        info!("Untracked Issue {}: {}", issue.get_id(), issue.get_title());
        let created_issue = github::create_issue(issue).unwrap();
        created_issues.insert(created_issue.id, created_issue.body);
    }

    if created_issues.len() > 0 {
        database::add_created_issues(created_issues, accessor);
    }
}
