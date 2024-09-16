use std::io::{self, Write};

fn main() {
    print!("What is your name?");
    let _ = io::stdout().flush(); //line-buffered what is this mean?
    let mut name = String::from("");

    std::io::stdin().read_line(&mut name).expect("Wrong input");

    let trimed_name = name.trim();

    if trimed_name.is_empty(){
        println!("No name entered :(, goodbye.");
    } else{
        println!("Hello, {trimed_name}, nice to meet you!");
    }

}
