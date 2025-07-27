use std::{
    env,
    fs::File,
    io::{Read, Write},
    os::windows::process,
    path::Path,
    sync::atomic::AtomicBool,
};

use lox_r::{HAD_ERROR, read_code_file, run_prompt};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        read_code_file(&args[1]);
    } else {
        run_prompt();
    }
}
