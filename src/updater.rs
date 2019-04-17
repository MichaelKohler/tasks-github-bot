use crate::bugzilla;
use crate::github;

pub fn update_bugzilla(all_bugs: &Vec<bugzilla::Bug>) {
    println!("{:?}", all_bugs);
}

pub fn update_github(all_issues: Vec<github::Issue>) {
    println!("{:?}", all_issues);
}