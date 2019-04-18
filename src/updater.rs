use crate::bugzilla;
use crate::database;
use crate::github;

pub fn update_bugzilla(all_bugs: &Vec<bugzilla::Bug>) {
    let known_entries = database::get_data().unwrap();
    info!("Found {} known Bugzilla entries", known_entries.bugzilla.len());

    for bug in all_bugs.iter() {
        if known_entries.bugzilla.contains(&bug.id) {
            continue;
        }

        info!("Untracked Bug: {:?}", bug);
        github::create_issue_from_bugzilla(bug);
    }
}

pub fn update_github(all_issues: Vec<github::Issue>) {
    let known_entries = database::get_data().unwrap();
    info!("Found {} known GitHub entries", known_entries.github.len());

    for issue in all_issues.iter() {
        if known_entries.github.contains(&issue.html_url) {
            continue;
        }

        info!("Untracked Issue: {:?}", issue);
        github::create_issue_from_github(issue);
    }
}
