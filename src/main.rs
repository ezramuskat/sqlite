use std::str::FromStr;

use rustyline::{DefaultEditor, Result, error::ReadlineError};
use strum::{IntoEnumIterator, EnumMessage};
use strum_macros::{Display, EnumIter, EnumString, EnumMessage};



fn main() -> Result<()>{
    //Initialize file handling/BTree stuff

    //start REPL
    let mut rl = DefaultEditor::new()?;
    
    loop {
        let readline = rl.readline("db > ");
        match readline {
            Ok(line) => {
                //rl.add_history_entry(line.as_str());
                
                //if line starts with ., do meta command
                if line.starts_with('.') {
                    match MetaCommand::from_str(line.as_str()) {
                        Ok(MetaCommand::Exit) => break,
                        Ok(MetaCommand::Help) => {
                            for command in MetaCommand::iter() {
                                println!("{} {:^32} {}", command.to_string(), " ", command.get_message().unwrap())
                            }
                        },
                        Err(_) => {
                            println!("Unknown command {}", line);
                        }
                    }
                }
                //else process statement
                else {
                    parse_statement(line);
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }

    Ok(())
}

#[derive(Display, EnumIter, EnumMessage,EnumString)]
enum MetaCommand {
    #[strum(message="Exit this program")]
    #[strum(serialize = ".exit")]
    Exit,

    #[strum(message="Show help text")]
    #[strum(serialize = ".help")]
    Help,
}

const SELECT_SYNTAX_STRING: &str = "Syntax: SELECT column1, column2, ... FROM table_name; or SELECT * FROM table_name;";
const INSERT_SYNTAX_STRING: &str = "Syntax: INSERT INTO table_name VALUES (value1, value2, ...); or INSERT INTO table_name (column1, column2, ...) VALUES (value1, value2, ...);";

fn parse_statement(statement: String) {
    let mut args= statement.split_ascii_whitespace();

    match args.next() {
        Some("SELECT") => {
            let mut columns: Vec<&str> = vec![];
            while let Some(column) = args.next() {
                //handle wildcards
                if column == "*" {
                    if !columns.is_empty() {
                        println!("The wildcard operator must be used on its own");
                        return;
                    }
                    else if args.next() != Some("FROM") {
                        println!("{}", SELECT_SYNTAX_STRING);
                        return;
                    }
                    else {
                        columns.push("*");
                        break;
                    }
                } else if column == "FROM" {
                    if columns.is_empty() {
                        println!("{}", SELECT_SYNTAX_STRING);
                        return;
                    }
                    break;
                } else {
                    columns.push(column);
                }
            }
            if args.next() == None {
                println!("{}", SELECT_SYNTAX_STRING);
                return;
            }
            println!("SELECT functionality is not currently supported");
        },
        Some("UPDATE") => {
            println!("UPDATE functionality is not currently supported");
        },
        Some("DELETE") => {
            println!("DELETE functionality is not currently supported");
        },
        Some("INSERT") => {
            if args.next() != Some("INTO") {
                println!("{}", INSERT_SYNTAX_STRING);
                return;
            }
            println!("INSERT functionality is not currently supported");
        },
        Some("CREATE") => {
            if args.next() == Some("TABLE") {
                
            } else if args.next() == Some("INDEX") {
                
            } else {
                println!("Unknown statement");
                return;
            }
            println!("CREATE functionality is not currently supported");
        },
        Some(_) => {
            println!("Unknown statement");
        },
        None => {
            return;
        }
    }
}