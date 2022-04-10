use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("{:?}, {}", args, command);

    if command == "test" {
        println!("Testing stuff");
    } else {
        println!("Not testing stuff");
    }
}