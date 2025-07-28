use std::{fs::File, io::{Read, Write}, path::Path, sync::atomic::AtomicBool};

use crate::scanner::Scanner;

mod scanner;
mod expr;
mod token;
mod generate_ast;



pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);
///扫描代码
/// 扫描代码的入口函数

pub fn  read_code_file(path: &str) {
    // 这里可以添加代码扫描的逻辑
    let path = Path::new(path);

    debug_assert!(
        path.is_file(),
        "Expected a file, but got a directory or non-existent path: {}",
        path.display()
    );

    if !path.exists() || !path.is_file() {
        // println!("Path does not exist: {}", path.display());
        eprintln!("Expected a file,path: {}", path.display());
        return;
    }

    // 读取文件内容
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file {}: {}", path.display(), e);
            return;
        }
    };
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    run(content);

    if HAD_ERROR.load(std::sync::atomic::Ordering::SeqCst) {
        // 如果有错误，退出
        std::process::exit(65);
    }
}

pub fn run_prompt() {
    println!("Running in interactive mode...");
    loop {
        //这个宏不会自动刷新输出缓冲区，因此需要手动刷新
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();

        if std::io::stdin().read_line(&mut input).is_err() {
            break; // 如果读取失败，退出循环
        }
        run(input);
        // 重置错误状态
        HAD_ERROR.store(false, std::sync::atomic::Ordering::SeqCst);
    }
}

fn run(code: String) {
    let mut scanner = Scanner::new(code);
    let tokens = scanner.scan_tokens();

    for token in tokens.iter() {
        println!("{:?}", token);
    }
}

pub fn err(line: u32, msg: &str) {
    // eprintln!("Error: {}", msg);
    report(line, "", msg);
}

fn report(line: u32, wher: &str, msg: &str) {
    eprintln!("[line {}] Error{}: {}", line, wher, msg);
    // Ordering::SeqCst：

    // 最严格的内存顺序，保证所有线程观察到的操作顺序一致。
    // 适用于需要全局一致性的场景（如错误标志）。
    HAD_ERROR.store(true, std::sync::atomic::Ordering::SeqCst);
}
