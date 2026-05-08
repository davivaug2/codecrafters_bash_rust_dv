#[allow(unused_imports)]
use std::io::{self, Write};
//use std::path::Path; // use std::fs::exists;
use std::process::Command;
use std::process::exit;
use pathsearch;//use pathsearch::PathSearcher;
fn main() -> Result<(), Box<dyn std::error::Error>> { //let args: Vec<String> = env::args().collect()
    let predefined_commands = ["echo","pwd","cd","type", "exit"]; //vec!["type", "echo", "exit"];
    loop {
        print!("$ ");
        io::stdout().flush()?;// io::stdout().flush().unwrap() , ? encouraged more
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;//io::stdin().read_line(&mut input).unwrap(); ide use ?
        let inp_lower = input.to_lowercase();
        let commands: Vec<&str> = input.split_whitespace().collect();// input.trim().split_whitespace().collect() same
        let commands_lower: Vec<&str> = inp_lower.split_whitespace().collect(); // For case-insensitive command matching, create lowercase version
        match commands_lower.as_slice() {
            // ["add", _] => {} // _ is 1 value at a position using match //[_,"add", _]
            ["echo", after @ ..] => {
                println!("{}", after.join(" ")); //  let output = _after.join(" ");//String better print
            }
            ["pwd", _after @ ..] => {
                //std::env::home_dir()
                let path = std::env::current_dir()?; // env::getcwd().unwrap().to_str().unwrap());
                println!("{}", path.display());//The current directory is.  // current_dir.to_str().unwrap()
            }
            ["cd", path_parts @ ..] => {
                let path_dir = path_parts.join(" ");
                std::env::set_current_dir(&path_dir).unwrap_or_else(|_| println!("cd: {path_dir}: No such file or directory"));


            }
            ["type", second_command, _after @ ..] => {
                if predefined_commands.contains(second_command) {
                    println!("{} is a shell builtin", second_command); // full_path.exists() == true ? same as full_path.exists()
                } else if let Some(exe_name_path) =
                    pathsearch::find_executable_in_path(&second_command)
                {
                    println!("{} is {}", second_command, exe_name_path.display());
                } else {
                    println!("{}: not found", second_command);
                };
            } //type
            [program_exe, _after @ ..]
                if let Some(_exe_name_path) = pathsearch::find_executable_in_path(&program_exe) =>
            {
                //println!("{:?} is an EXE", _exe_name_path.display());
                let output = Command::new(commands[0])
                    .args(&commands[1..])
                    .output()
                    .expect("should be able to execute `"); //let output = Command::new(&commands[0]).args(&commands[1..]).output().unwrap();
                assert!(output.status.success()); // println!("status: {}", output.status);

                /* println!("Program was passed {} args (including program name).",commands.len());//println!("Arg #0 (program name): {}",commands[0]);
                commands.iter().skip(1).enumerate().for_each(|(index,arg)|//println!("Arg #{}: {}",index+1,arg));// skip_while */
                print!("{}", String::from_utf8_lossy(&output.stdout)); //println!("stderr: {}", String::from_utf8_lossy(&output.stderr));//errors
            }
            /*
            FIX [spaces]  if !spaces.chars().any(|c| c != ' ')  => {
                println!("sss");
                continue;
            }
             */

            ["exit",..] => exit(0),
            _ => println!("{0}: command not found", commands.join(" ")), // default for match
        } // match
    } //loop
} //main

/*


more specific website learn specific parts
https://www.thorsten-hans.com/working-with-environment-variables-in-rust/
https://www.w3tutorials.net/blog/rust-pattern-matching-over-a-vector/
https://buildsoftwaresystems.com/post/rust-match-slice-pattern-matching/
https://dev.to/sgchris/reading-environment-variables-with-dotenv-and-stdenv-51ok
https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str
https://oneuptime.com/blog/post/2026-01-25-rust-match-expressions/view
https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output
https://doc.rust-lang.org/std/process/struct.Command.html
https://doc.rust-lang.org/std/env/fn.current_dir.html
https://doc.rust-lang.org/std/env/index.html
-------------
general learning rust
https://rust-book.cs.brown.edu
https://doc.rust-lang.org
https://docs.rs/
https://github.com/rust-lang/rust
https://doc.rust-lang.org/rust-by-example


 */

/*
Cargo fmt
cargo clippy
---------

-------------
read var_os key 1 line
// env::var_os(input_key).map(|paths| env::split_paths(&paths).collect()).unwrap_or_default() }
------

Way to check Path Linux Specific.
//use std::os::unix::fs::PermissionsExt;// Cannot find `Unix` in `os` [E0433]
//`std_internals` is unstable [E0658]
else if let Some(path_name) = paths_v.iter().find(|path| {
 let full_path = path.join(second_command);
 full_path.exists()
     && std::fs::metadata(&full_path).unwrap().permissions().mode() & 0o111 != 0
     //metadata(path).unwrap().permissions().
})//
{
 println!("{} is {}", output, path_name.join(second_command).display());
    let fdfs =  std::fs::metadata(path_name).unwrap().permissions();
                    }
-------




 */
