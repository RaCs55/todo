use crate::libs::models::Task;
use colored::Colorize;
use serde_json::Value;
use std::{
    fs::{metadata, read_to_string, remove_file, write, File},
    io::Result,
};

fn get_last_id() -> Result<u64> {
    let data = read_to_string("tasks.json").unwrap();

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data.as_str())?;

    let mut last_id: u64 = 0;
    let array = v.as_array().unwrap();
    for item in array {
        if let id = item["id"].as_u64().unwrap() {
            last_id = id;
        } else {
            last_id = 0;
        }
    }
    Ok(last_id)
}

pub fn remove_task(input: u64) {
    let data: String = read_to_string("tasks.json").expect("Fail");
    let v: Value = serde_json::from_str(data.as_str()).unwrap();

    let vec = v.as_array().unwrap();
    let array = vec;

    remove_file("tasks.json").expect("Error delting file");
    File::create("tasks.json").expect("Error to create the file");
    for item in array {
        let id = item["id"].as_u64().unwrap();
        let name: &str = item["name"].as_str().unwrap();
        let completed: bool = item["completed"].as_bool().expect("msg");
        if id != input {
            match add_task(name.to_string(), completed) {
                Ok(_) => (),
                Err(_) => (),
            };
        }
    }
}

pub fn complete_task(input: u64) {
    let data: String = read_to_string("tasks.json").expect("Fail");
    let v: Value = serde_json::from_str(data.as_str()).unwrap();

    let vec = v.as_array().unwrap();
    let array = vec;

    remove_file("tasks.json").expect("Error delting file");
    File::create("tasks.json").expect("Error to create the file");
    for item in array {
        let id = item["id"].as_u64().unwrap();
        let name: &str = item["name"].as_str().unwrap();
        let completed: bool = item["completed"].as_bool().expect("msg");
        if id == input {
            match add_task(name.to_string(), true) {
                Ok(_) => (),
                Err(_) => (),
            };
        } else {
            match add_task(name.to_string(), completed) {
                Ok(_) => (),
                Err(_) => (),
            }
        }
    }
}

pub fn add_task(name: String, complete: bool) -> Result<()> {
    if !metadata("tasks.json").is_ok() {
        File::create("tasks.json").unwrap();
    }
    let tasks: String = read_to_string("tasks.json").expect("Fail to read the json");

    let mut taskshelf: Vec<Task> = Vec::new();
    if metadata("tasks.json").unwrap().len() != 0 {
        taskshelf = serde_json::from_str(&tasks)?;
    }

    let last_id = match get_last_id() {
        Ok(id) => id,
        Err(_) => 0,
    };

    let task = Task {
        id: last_id + 1,
        name: name,
        completed: complete,
    };

    taskshelf.push(task);

    let json = serde_json::to_string(&taskshelf)?;
    write("tasks.json", &json).expect("Unable to write file");
    Ok(())
}

pub fn show_task() {
    let data: String = read_to_string("tasks.json").expect("Fail to read the json");
    let v: Value = serde_json::from_str(data.as_str()).unwrap();

    let array = v.as_array().unwrap();
    for item in array {
        let id = item["id"].as_u64().unwrap();
        let name = item["name"].as_str().unwrap();
        let complete = item["completed"].as_bool().unwrap();
        if complete == true {
            println!("{} {}", id, name.green());
        } else {
            println!("{} {}", id, name.red());
        }
    }
}
