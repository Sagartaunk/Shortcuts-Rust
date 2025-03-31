use std::io;
use std::fs::{OpenOptions , File};
fn main() {
    println!("Welcome to the Shortcuts App !");
    loop{
        functions::load_file();
        let option = cli::input();
        match option{
            ".help" => {
                cli::help();
            }
            ".exit" => {
                println!("Exiting the app ....");
                break;
            }
            ".add" => {
                functions::add();
            }
            ".remove" => {
                functions::remove();
            }
            ".edit" => {
                functions::edit();
            }
            ".list" => {
                functions::list();
            }
        }
    }
}

pub fn first_run(){
}