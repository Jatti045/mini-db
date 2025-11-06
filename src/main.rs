use std::io::{self, Write};
use mini_db::engine::{DatabaseHandle};
use mini_db::parser::handle_command;

fn main() {

    let path = "mini_db.snapshot";
    let db = DatabaseHandle::new(path).expect("Failed to initialize db.");
      

    loop {
        print!("mini_db> ");
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        if !handle_command(&input, &db) {
            break;
        }
    }
}



