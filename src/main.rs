mod lexer;
mod token;
mod token_type;

use crate::lexer::Lexer;
use crate::token_type::TokenType;
extern crate colored;

use colored::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_file(path: &str) -> std::io::Result<String> {
    let file = match File::open(path) {
        Ok(file) => Ok(file),
        Err(_) => File::open(format!("{}.vom", path)),
    }
    .expect("File could not be opened");

    let mut buf_reader = BufReader::new(file);
    let mut source = String::new();
    buf_reader
        .read_to_string(&mut source)
        .expect("File read failed");
    Ok(source)
}

fn lex_program(path: &String) {
    let source = read_file(path).expect("Source file could not be opened");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    println!("\n{} `{}`\n", "Running".bright_green().bold(), path);
    println!(
        "{:4}. {:>4} : {:<4} {:25} {}",
        "S\\No",
        "Ln".bold(),
        "Col".bold(),
        "Type".bold(),
        "Literal".bold()
    );

    let mut sn: usize = 1;
    for token in tokens {
        if let TokenType::Begin | TokenType::End = token.token_type {
            continue;
        }

        println!(
            "{:<4}  {:>4} : {:<4} {:25} {}",
            sn,
            token.line.to_string().bright_yellow(),
            token.col.to_string().bright_yellow(),
            token.token_type.to_string().bright_cyan(),
            token.literal.green(),
        );

        sn += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for file_path in args[1..].iter() {
            lex_program(file_path);
        }
    } else {
        panic!("Specify program file(s) to run");
    }
}
