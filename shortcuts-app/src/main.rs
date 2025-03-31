use shortcuts_app::{cli, functions};
fn main() {
    println!("Welcome to the Shortcuts App !");
    loop{
        functions::load();
        let option = cli::input();
        match option.as_str(){
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
            ".list" => {
                functions::list();
            }
            &_ => {
                functions::run(option.to_string());
            }
        }
    }
}

pub fn first_run(){
}