use std::io;


fn main() {
    let pattern_string = std::env::args()
        .nth(1)
        .expect("missing required command-line argument: <pattern>");

    let pattern = &pattern_string;

    // TODO: Replace the following with your code:
   

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("wrong input");
        let proccessed_input = input.trim();
        if input.contains(pattern){
            print!("{}", input);
        }
        if proccessed_input.is_empty(){
            break;
        }
    
    }
    
}
