use std::{fs , io};
use crate::{cli , Shortcut};
use serde_json;

pub fn load() -> Vec<Shortcut> {
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
    fs::write("shortcuts.json", serde_json::to_string(shortcuts).unwrap()).expect("Unable to write file");
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
    let shortcuts = load();
    let shortcut = shortcuts.iter().find(|s| s.name.to_string() == command);
    match shortcut {
        Some(s) => {
            
        },
        None => {
            println!("Shortcut not found. Type .list to list all existing shortcuts.");
        },
    }
}
pub fn list(){
    let shortcuts = load();
    for i in shortcuts.iter(){
        println!("Name : {} , Command : {}", i.name, i.command);
    }
}