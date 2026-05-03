#[allow(unused_imports)]
use std::io::{self, Write};
//use std::io::Read;

fn main() {
     loop {
          print!("$ ");
          io::stdout().flush().unwrap();
          let mut command = String::new();
          io::stdin().read_line(&mut command).unwrap();
          println!("{0}: command not found", command.trim());
     }
}
