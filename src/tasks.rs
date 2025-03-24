use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;

#[derive(Serialize, Deserialize)]

pub struct Task{
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

impl Task{
    pub fn new(id:u32, description:String) -> Self{
        Self{
            id,
            description,
            completed:false,
        }
    }
}

pub fn save_tasks(tasks:&Vec<Task>) -> io::Result<()>{
    let json = serde_json::to_string_pretty(tasks)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_tasks() -> Vec<Task>{
    if Path::new(FILE_PATH).exists(){
        let mut file  = File::open(FILE_PATH).expect("Failed to Open File");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read File");
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}