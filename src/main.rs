use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("{}", MESSAGE);
        std::process::exit(1);
    }

    let binary_name = &args[1];

    let status = Command::new(format!("target/debug/{}", binary_name))
        .status()
        .expect("Failed to execute process");

    if !status.success() {
        eprintln!("Error executing the specified binary");
        std::process::exit(1);
    }
}

const MESSAGE: &'static str = r#"
To use the runner, specify the day-part binary:

Day 5, part 1    ->    cargo r --bin 5-1
Day 21, part 2   ->    cargo r --bin 21-2
"#;
