use std::io;
pub fn input() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read command");
    let input = input.trim().to_string();
    input
}
pub fn first_run(){
    println!("Welcome to the Shortcuts CLI!");
    println!("This is the first time you are running this program.");
    println!("Type '.help' to see a list of commands.");
    println!("Type '.exit' to exit the program.");
}
pub fn help(){
    println!("App commands : ");
    println!("  .help : Show this help message");
    println!("  .exit : Exit the program");
    println!("  .add : Add a new shortcut");
    println!("  .list : List all shortcuts");
    println!("  .remove : Delete a shortcut");
    println!("  Shortcut_name : Run a shortcut");

}