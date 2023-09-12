use std::process::exit;
use std::{env, fs};

use lexer::lex;
use matcher::find_matches;

mod lexer;
mod matcher;

fn main() {
    let files: Vec<_> = env::args().skip(1).collect();
    if files.is_empty() {
        eprintln!("Usage: cursed-lint <file> ...");
        exit(2);
    }

    let mut warning_count: u64 = 0;
    for file in files {
        let content = fs::read_to_string(&file).expect("failed to read file");
        let tokens = lex(&content);
        let warnings = find_matches(&tokens);
        for warning in warnings {
            println!("{file}:{}: {}", warning.tokens[0].line, warning.description);
            warning_count += 1;
        }
    }

    println!("{warning_count} warning(s) emitted");
    if warning_count != 0 {
        exit(1);
    } else {
        exit(0);
    }
}
