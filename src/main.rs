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
                println!("Line: {}", line);
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
