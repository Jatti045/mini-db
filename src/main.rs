use std::io::{self, Write};

use mini_db::engine::Database;
use mini_db::model::Row;
use mini_db::parser;

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

        Ok(parser::Command::SelectById { id }) => {
            match db.select_by_id(id) {
                Ok(Some(row)) => println!("{:?}", row),
                Ok(None) => println!("Row with id {} not found.", id),
                Err(e) => eprintln!("Error fetching row by id: {}", e)
            }
            true
        },

        Ok(parser::Command::DeleteById { id }) => {
            match db.delete_by_id(id) {
                Ok(true) => println!("Row with id {} deleted.", id),
                Ok(false) => println!("No row found with id {}.", id),
                Err(e)    => eprintln!("Error deleting row: {}", e),
            }   
            true
        },

        Ok(parser::Command::Select) => {
            let rows: &Vec<Row> = db.select_all();

            if rows.len() == 0 {
                println!("(no rows)");
                return true;
            }

            for row in rows.iter() {
                println!("{:?}", row)
            }
            true
        }, 

        Ok(parser::Command::Exit) => {
            if let Err(e) = db.shutdown() {
                println!("Warning: could not flush data: {}", e);
            }
            println!("Exiting mini_db... Goodbye!");
            false
        },

        Ok(parser::Command::Help) => {
            println!("\nAvailable commands:\ninsert <id> <name> <age>\nselect\nreset\nexit\n");
            true
        },

        Ok(parser::Command::Reset) => {
            match db.reset_db() {
                Ok(_) => println!("All data cleared."),
                Err(_) => println!("Database could not be reset."),
            }
            true
        }

        Err(_) => {
            println!("Enter a valid command");
            true
        },
    }


}

