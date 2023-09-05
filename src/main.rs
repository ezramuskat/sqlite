use std::str::FromStr;

use executer::execute_statement;
use nom_sql::parser;
use rustyline::{DefaultEditor, Result, error::ReadlineError};
use strum::{IntoEnumIterator, EnumMessage};
use strum_macros::{Display, EnumIter, EnumString, EnumMessage};

mod executer;
mod dbtree;
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
                    match parser::parse_query(line) {
                        Ok(query) => execute_statement(query),
                        Err(e) => println!("{}", e)
                    }
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