
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
use std::env;
use std::ffi::OsString;
use std::process::Command;
fn read_env_os_string(key: &str)  ->  OsString   {
    match env::var_os(key) {
        Some(paths) => { return paths; }
        None => { return OsString::new() }//println!("{key} is not defined in the environment.");
    }
}

fn main() {
    //let args: Vec<String> = env::args().collect();
    let key = "PATH";
    let path_dir_list_os_string = read_env_os_string(key);
    let path_split_paths = env::split_paths(&path_dir_list_os_string);
    //println!("{:?}",tes);
    //println!("{:?}",paths_dir_list);
    //println!("{:?} Paths dir size",path_dir_list_os_string.len());
    let predefined_commands = vec!["type","echo","exit"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let inp_lower = input.to_lowercase();
        let commands: Vec<&str> = input.trim().split_whitespace().collect();
        //let paths_v: Vec<_> = env::split_paths(&path_dir_list_os_string).collect();
        let commands_lower: Vec<&str> = inp_lower.trim().split_whitespace().collect(); // For case-insensitive command matching, create lowercase version
        match commands_lower.as_slice() {
            ["add",_] => {} // _ is 1 value
            ["echo", after @ ..] => {
                println!("{}", after.join(" ")); //  let output = after.join(" ");//String better print
            }
            ["type", second_command, after @ ..] => {
                if (predefined_commands.contains(second_command)) {
                    println!("{} is a shell builtin", second_command);
                // full_path.exists() == true ? same as full_path.exists()
                } else if let Some(exe_name_path) = pathsearch::find_executable_in_path(&second_command) {
                    println!("{} is {}", second_command, exe_name_path.display());;
                }
                else { println!("{}: not found", second_command); };
            }//type
            [program_exe, arguments @ .. ] if let Some(exe_name_path) =  pathsearch::find_executable_in_path(&program_exe) =>
                {
                //println!("{:?} is a EXE", exe_name_path.display());

                    let output = Command::new(&commands[0])
                        .args(&commands[1..])
                        .output()
                        .expect("should be able to execute `/bin/cat`");//let output = Command::new(&commands[0]).args(&commands[1..]).output().unwrap();
                    assert!(output.status.success()); // println!("status: {}", output.status); //
                    /*
                    println!("Program was passed {} args (including program name).",commands.len());
                    println!("Arg #0 (program name): {}",commands[0]);
                    commands.iter().skip(1).enumerate().for_each(|(index,arg)|
                        println!("Arg #{}: {}",index+1,arg));// skip_while
                     */
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                    //println!("stderr: {}", String::from_utf8_lossy(&output.stderr));//errors
                }
            ["exit",..]  => exit(0),
            _ =>  println!("{0}: command not found", commands.join(" ")) // default for match
        }// match
    }//loop
}//main




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

-------------
general learning rust
https://rust-book.cs.brown.edu
https://doc.rust-lang.org
https://docs.rs/
https://github.com/rust-lang/rust
https://doc.rust-lang.org/rust-by-example
look into
https://dev-doc.rust-lang.org/std/env/index.html

 */


/*
---------
https://github.com/cc-code-examples/good-mole-331190/blob/main/src/main.rs
other ways to get path useing args
let mut input: String = "".to_string();
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        let command: Vec<&str> = input.split(" ").collect();
        if command[0] == "exit" {
            break;
        }
        ... fn eval_command(command: &str, args: Vec<&str>) ... &args[0]
        // rust ways to get env vars
//  env::var("PATH") vs std::env::var("PATH").unwrap_or_default();  vs  env::var_os("PATH")
// let lorem = env!("LOREM_IPSUM");
-------------
read var_os key 1 line
// env::var_os(input_key).map(|paths| env::split_paths(&paths).collect()).unwrap_or_default() }
------
Way to check Path Linux Specific.
//use std::os::unix::fs::PermissionsExt;// Cannot find `unix` in `os` [E0433]
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


