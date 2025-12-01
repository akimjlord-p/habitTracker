use crate::service::habit::Habit;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};


#[derive(Deserialize, Serialize)]
pub struct Storage{
    habits: Vec<Habit>
}


impl Storage{
    pub fn generate() -> Self{
        let file = fs::read_to_string("storage.json");
        match file {
            Ok(file) => match serde_json::from_str(&file) {
                Ok(storage) => storage,
                Err(error) => panic!("{error}")
            },
            Err(_) => {
                Self{
                    habits: Vec::new()
                }
            }
        }
    }


    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write("storage.json", json)?;
        Ok(())
}
}