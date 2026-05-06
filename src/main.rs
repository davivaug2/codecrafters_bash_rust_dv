use std::path::PathBuf;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
use std::env;
use std::ffi::OsString;
use std::io::Read;

fn read_env_os_string(key: &str)  ->  OsString   {
    match env::var_os(key) {
        Some(paths) => { //for path in env::split_paths(&paths) {//println!("'{}'", path.display())
            return paths; // env::var_os(input_key).map(|paths| env::split_paths(&paths).collect()).unwrap_or_default() }
        }
        None => {println!("{key} is not defined in the environment.");
            return OsString::new()
        }
    }
}


fn main() {
    //let args: Vec<String> = env::args().collect();
    let key = "PATH";
    let path_dir_list_os_string = read_env_os_string(key);
    let path_split_paths = env::split_paths(&path_dir_list_os_string);
    //println!("{:?}",tes);
    //println!("{:?}",paths_dir_list);
    println!("{:?} Paths dir size",path_dir_list_os_string.len());
    let predefined_commands = vec!["type","echo","exit"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let inp_lower = input.to_lowercase();
        let commands: Vec<&str> = inp_lower.trim().split_whitespace().collect();
        let paths_v: Vec<_> = env::split_paths(&path_dir_list_os_string).collect();
        match commands.as_slice() {
            ["add",_] => {} // _ is 1 value
            ["echo", after @ ..] => {
                println!("{}:", after.join(" ")); //  let output = after.join(" ");//String better print
            }
             ["type", second_command, after @ ..] => {
                 let output = second_command;//after.join(" "); // after[0] // .iter().enumerate() finds inex then  then
                 let output2 = OsString::from(&output);
                 if (predefined_commands.contains(second_command)) {
                     println!("{} is a shell builtin", second_command);
                 } else if let Some(path_name) = paths_v.iter().find(|&path| {
                     path.file_name() == Some(output2.as_os_str()) })
                    {
                     println!("{} is {}", output, path_name.display());
                    }
                 else { println!("{}: not found", second_command); };
             }//type
            ["exit",..]  => exit(0),
            _ =>  println!("{0}: command not found", commands.join(" ")) // default for match
        }// match
         }//loop
     }//main




/*

// rust ways to get env vars
//  env::var("PATH") vs std::env::var("PATH").unwrap_or_default();  vs  env::var_os("PATH")
// let lorem = env!("LOREM_IPSUM");
-------------
more specific website learn specific par
https://www.thorsten-hans.com/working-with-environment-variables-in-rust/
https://www.w3tutorials.net/blog/rust-pattern-matching-over-a-vector/
https://buildsoftwaresystems.com/post/rust-match-slice-pattern-matching/
https://dev.to/sgchris/reading-environment-variables-with-dotenv-and-stdenv-51ok
https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str

-------------
general learning rust
https://rust-book.cs.brown.edu
https://doc.rust-lang.org
https://docs.rs/
https://github.com/rust-lang/rust

look into
https://dev-doc.rust-lang.org/std/env/index.html

//  Rostover autocomplete
 */


