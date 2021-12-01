use std::io::Write;
use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_] => run_prompt(),
        [_, file] => run_file(file),
        _ => println!("Usage: jlox [script]"),
    }
}

fn run_prompt() {
    let mut line = String::new();
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    loop {
        print!("> ");
        stdout.flush().unwrap();
        let len = stdin.read_line(&mut line);

        match len {
            Ok(0) | Err(_) => {
                println!();
                break;
            },
            Ok(_) => run(&line),
        }
        line.clear();
    }
}

fn run_file(source_file: &str) {
    let source_code = fs::read_to_string(source_file).expect("Unable to read file");
    run(&source_code);
}
  
fn run(source_code: &str) {
    println!("{}", source_code);
}
