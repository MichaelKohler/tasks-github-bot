use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::{Index,IndexMut};
use serde_json::Error;

use std::collections::HashMap;
use std::env;

const DATABASE_ENV_NAME: &str = "DATABASE_PATH";

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryMapping {
    pub bugzilla: Vec<String>,
    pub github: Vec<String>
}

impl Index<&str> for EntryMapping {
    type Output = Vec<String>;

    fn index(&self, accessor: &str) -> &Vec<String> {
        match accessor {
            "bugzilla" => &self.bugzilla,
            "github" => &self.github,
            &_ => &self.github,
        }
    }
}

impl IndexMut<&str> for EntryMapping {
    fn index_mut(&mut self, accessor: &str) -> &mut Vec<String> {
        match accessor {
            "bugzilla" => &mut self.bugzilla,
            "github" => &mut self.github,
            &_ => &mut self.github,
        }
    }
}

pub fn get_file_content() -> std::io::Result<String> {
    let database_file = get_path();
    debug!("Opening {}", database_file);
    let mut file = File::open(database_file)?;
    let mut contents = String::new();
    debug!("Reading database");
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
    let database_file = get_path();
    fs::write(database_file, pretty_json).expect("Unable to write file");
    debug!("Successfully wrote to database");
}

pub fn add_created_issues(issues: HashMap<u32, String>, accessor: &str) {
    debug!("Updating created issues in database");
    let mut data: EntryMapping = get_data().unwrap();
    let mut issue_urls: Vec<String> = issues.iter().map(|(_, issue)| issue.clone()).collect();
    data[accessor].append(&mut issue_urls);
    write_data(data);
}

fn get_path() -> String {
    let database_path = match env::var(&DATABASE_ENV_NAME) {
      Ok(val) => val,
      Err(_e) => {
        panic!("No DATABASE_PATH set");
      },
    };

    return database_path;
}