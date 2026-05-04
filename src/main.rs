#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
//use std::io::Read;

fn main() {
     let predefined_commands = vec!["type","echo","exit"];
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
               ["type", second_command, after @ ..] => {
                    if (predefined_commands.contains(second_command)){
                         println!("{} is a shell builtin", second_command);
                    }
                    else { println!("{}: not found", second_command); }
               }
               ["exit",..]  => exit(0),
               _ =>  println!("{0}: command not found", commands.join(" ")) // default for match
          }

     }
}

// https://www.w3tutorials.net/blog/rust-pattern-matching-over-a-vector/
// https://buildsoftwaresystems.com/post/rust-match-slice-pattern-matching/


