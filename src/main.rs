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
