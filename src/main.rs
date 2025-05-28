use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            //REPL
            println!("no file");
        }
        2 => {
            println!("a file");
            // 执行文件
        }
        _ => {
            println!("usage: jlox[script]");
        }
    }
}
