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

    for bug in all_bugs.iter() {
        if known_entries.bugzilla.contains(&bug.id) {
            continue;
        }

        println!("Untracked Bug: {:?}", bug);
    }
}

pub fn update_github(all_issues: Vec<github::Issue>) {
    let known_entries = get_data().unwrap();

    for issue in all_issues.iter() {
        if known_entries.github.contains(&issue.html_url) {
            continue;
        }

        println!("Untracked Issue: {:?}", issue);
    }
}

fn get_file_content() -> std::io::Result<String> {
    let mut file = File::open("db.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_data() -> Result<EntryMapping, Error> {
    println!("Getting Data from Database");
    let contents = get_file_content().unwrap();
    let data: EntryMapping = serde_json::from_str(&contents)?;
    println!("{:?}", data);
    Ok(data)
}