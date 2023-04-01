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
        Ok(
            serde_json::from_reader(
                BufReader::new(
                    File::open(path)?
                )
            )?
        )
    }
    pub fn get_names(&self) -> &Vec<String> {
        &self.names
    }
}
