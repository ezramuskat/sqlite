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

fn parse_statement(statement: String) {
    if statement.is_empty() {
        return;
    }
    let args: Vec<&str> = statement.split_ascii_whitespace().collect();

    match args[0] {
        _ => {
            println!("Unknown statement");
        }
    }
}