#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
//use std::io::Read;

fn main() {
     loop {
          print!("$ ");
          io::stdout().flush().unwrap();
          let mut input = String::new();
          io::stdin().read_line(&mut input).unwrap();
          let commands: Vec<&str> = input.trim().split_whitespace().collect();
          match commands.as_slice() {
               ["add",_] => {} // _ is 1 value
               ["echo", after @ ..] => {
                    let output = after.join(" ");
                    println!("{}", output); //{:?} {:#?}
               }
               ["exit",..]  => exit(0),
               _ =>  println!("{0}: command not found", input) // default for match
          }

     }
}

// https://www.w3tutorials.net/blog/rust-pattern-matching-over-a-vector/
// https://buildsoftwaresystems.com/post/rust-match-slice-pattern-matching/


