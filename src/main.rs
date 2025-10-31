use std::io::{self, Write};
use mini_db::engine::Database;
use mini_db::parser::handle_command;

fn main() {

    let path = "data.json";
    let mut db = Database::new(path).expect("failed to initialize database");
      

    loop {
        print!("mini_db> ");
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        if !handle_command(&input, &mut db) {
            break;
        }
    }
}



