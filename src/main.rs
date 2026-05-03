#[allow(unused_imports)]
use std::io::{self, Write};
//use std::io::Read;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
     print!("$ ");
     io::stdout().flush().unwrap();
     let mut command = String::new();
     io::stdin().read_line(&mut command).unwrap();
     //let d3 = command.trim();
     println!("{0}: command not found", command.trim()); // println!("{}: command not found", command);
     // io::stdout().flush().unwrap();
     //io::stdin().read_exact()
     // windows, the newline delimiter is "\r\n" instead of "\n".
}
