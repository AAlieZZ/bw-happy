use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Person {
    names: Vec<String>,
}

impl Person {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let person = serde_json::from_reader(reader)?;
        Ok(person)
    }
    pub fn get_names(&self) -> &Vec<String> {
        &self.names
    }
}