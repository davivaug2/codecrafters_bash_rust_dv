#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
//use std::io::Read;

fn main() {
     loop {
          print!("$ ");
          io::stdout().flush().unwrap();
          let mut command = String::new();
          io::stdin().read_line(&mut command).unwrap();
          command = command.trim().to_string();
          if command == "exit"{
               exit(0);
          }
          println!("{0}: command not found", command);
     }
}
