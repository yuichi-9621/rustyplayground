use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Yuichi";
    let stat = "100%";

    println!("Args: {:?}", args);
    println!("Args: {}", command);

    if command == "hello" {
        println!("Hello {}", name);
    } else if command == "status" {
        println!("Status is {}", stat);
    } else {
        println!("Not the right command");
    }
}