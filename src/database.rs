use std::fs;
use std::fs::File;
use std::io::Read;
use serde_json::Error;

use std::collections::HashMap;

const DB_NAME: &str = "db.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryMapping {
    pub bugzilla: Vec<String>,
    pub github: Vec<String>
}

pub fn get_file_content() -> std::io::Result<String> {
    debug!("Opening {}", DB_NAME);
    let mut file = File::open(DB_NAME)?;
    let mut contents = String::new();
    debug!("Reading {}", DB_NAME);
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_data() -> Result<EntryMapping, Error> {
    debug!("Getting Data from Database");
    let contents = get_file_content().unwrap();
    let data: EntryMapping = serde_json::from_str(&contents)?;
    debug!("{:?}", data);
    Ok(data)
}

fn write_data(data: EntryMapping) {
    debug!("Converting to pretty JSON");
    let json = json!(data);
    let pretty_json: String = serde_json::to_string_pretty(&json).unwrap();
    fs::write("db.json", pretty_json).expect("Unable to write file");
    debug!("Successfully wrote to database")
}

pub fn add_created_bugzilla_issues(issues: HashMap<u32, String>) {
    debug!("Updating created Bugzilla issues in Database");
    let mut data: EntryMapping = get_data().unwrap();
    let mut issue_urls: Vec<String> = issues.iter().map(|(_, issue)| issue.clone()).collect();
    data.bugzilla.append(&mut issue_urls);
    write_data(data);
}

pub fn add_created_github_issues(issues: HashMap<u32, String>) {
    debug!("Updating created GitHub issues in Database");
    let mut data: EntryMapping = get_data().unwrap();
    let mut issue_urls: Vec<String> = issues.iter().map(|(_, issue)| issue.clone()).collect();
    data.github.append(&mut issue_urls);
    write_data(data);
}