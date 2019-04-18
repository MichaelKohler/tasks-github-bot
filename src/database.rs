use std::fs::File;
use std::io::Read;
use serde_json::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryMapping {
    pub bugzilla: Vec<u32>,
    pub github: Vec<String>
}

pub fn get_file_content() -> std::io::Result<String> {
    debug!("Opening db.json");
    let mut file = File::open("db.json")?;
    let mut contents = String::new();
    debug!("Reading db.json");
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