use rustyline::{DefaultEditor, Result, error::ReadlineError};


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
                    match line.as_str() {
                        ".exit" => break,
                        _ => {
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