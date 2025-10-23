use std::io::{self, Write};

use mini_db::engine::Database;
use mini_db::model::Row;
use mini_db::parser;

fn main() {

    let mut db = Database::new();

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

// Takes user input, calls our parser method to parser input, and executes the instruction
fn handle_command(input: &str, db: &mut Database) -> bool {
    match parser::parse_command(input) {
        Ok(parser::Command::Insert {id, name, age}) => {
            match db.insert(id, name, age) {
                Ok(_) => println!("Inserted row with id {id}."),
                Err(e) => eprintln!("Error inserting into db: {}", e),
            }
            true
        },

        Ok(parser::Command::Select) => {
            let rows: &Vec<Row> = db.select_all();
            for row in rows.iter() {
                println!("{:?}", row)
            }
            true
        }, 

        Ok(parser::Command::Exit) => {
            println!("Exiting mini_db... Goodbye!");
            false
        },
        Ok(parser::Command::Help) => {
            println!("\nAvailable commands:\ninsert <id> <name> <age>\nselect\nexit\n");
            true
        }

        Err(_) => {
            println!("Enter a valid command");
            true
        },
    }


}

