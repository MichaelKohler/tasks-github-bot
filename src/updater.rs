use std::fs::File;
use std::io::Read;
use serde_json::Error;
use crate::bugzilla;
use crate::github;

#[derive(Serialize, Deserialize, Debug)]
struct EntryMapping {
    bugzilla: Vec<u32>,
    github: Vec<String>
}

pub fn update_bugzilla(all_bugs: &Vec<bugzilla::Bug>) {
    let known_entries = get_data().unwrap();
    info!("Found {} known Bugzilla entries", known_entries.bugzilla.len());

    for bug in all_bugs.iter() {
        if known_entries.bugzilla.contains(&bug.id) {
            continue;
        }

        info!("Untracked Bug: {:?}", bug);
    }
}

pub fn update_github(all_issues: Vec<github::Issue>) {
    let known_entries = get_data().unwrap();
    info!("Found {} known GitHub entries", known_entries.github.len());

    for issue in all_issues.iter() {
        if known_entries.github.contains(&issue.html_url) {
            continue;
        }

        info!("Untracked Issue: {:?}", issue);
    }
}

fn get_file_content() -> std::io::Result<String> {
    debug!("Opening db.json");
    let mut file = File::open("db.json")?;
    let mut contents = String::new();
    debug!("Reading db.json");
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_data() -> Result<EntryMapping, Error> {
    debug!("Getting Data from Database");
    let contents = get_file_content().unwrap();
    let data: EntryMapping = serde_json::from_str(&contents)?;
    debug!("{:?}", data);
    Ok(data)
}