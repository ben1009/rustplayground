use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("value: {:?}, len: {}", args, args.len());

    let config = minigrep::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("exit with error: {}", e);
        process::exit(1);
    });

    minigrep::run_grep(&config);
}
