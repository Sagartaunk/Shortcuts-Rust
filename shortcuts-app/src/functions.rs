use std::{fs , io};
use crate::{cli , Shortcut};
use serde_json;

pub fn load() -> Vec<Shortcut> {
    let mut shortcuts  = Vec::new();
    let file =  match fs::File::open("shortcuts.json") {
        Ok(file) => file ,
        Err(_) => {
            cli::first_run();
            fs::File::create("shortcuts.json").unwrap()
        }, 

    };
    let reader = io::BufReader::new(file);

    let data : Vec<Shortcut> = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(_) => {
            Vec::new()
        },
    };
    data
}

pub fn save(shortcuts: &Vec<Shortcut>) {
    let file = fs::File::open("shortcuts.json").unwrap();
    let writer = io::BufWriter::new(file);
    match serde_json::to_writer(writer, shortcuts) {
        Ok(_) => println!("Shortcuts saved successfully."),
        Err(e) => eprintln!("Error saving shortcuts: {}", e),
    }
}

pub fn add(){
    let mut shortcuts = load();
    let name =  cli::input();
    let function =  cli::input();
    let shortcut = Shortcut {
        name : name,
        command : function,
    };
    shortcuts.push(shortcut);
    save(&shortcuts);
    println!("Shortcut added successfully.");
}

pub fn remove(){
    let mut shortcut = load();
    let name = cli::input();
    let index = shortcut.iter().position(|s| s.name == name);
    match index {
        Some(i) => {
            shortcut.remove(i);
            save(&shortcut);
            println!("Shortcut removed successfully.");
        },
        None => {
            println!("Shortcut not found.");
        },
    }
}

pub fn run(command : String){
    
}