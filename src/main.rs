use std::env;

use project::RustAnalyzerProject;

mod project;
mod challenges;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        error_and_exit("Please provide exactly one argument");
    }

    let subcommand = &args[1];

    if subcommand == "lsp" {
        generate_rust_project_json();
    } 
    else {
        let Ok(selector_elements) = 
            subcommand
                .split(".")
                .map(|comp| comp.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()
        else {
            error_and_exit("Input argument must contain only numbers and a dot");
            return;
        };
            
        if selector_elements.len() != 2 {
            error_and_exit("Input must contain only 2 numbers and a dot.")
        }

        let day = selector_elements[0];
        let part = selector_elements[1];

        if !(1..=31).contains(&day) || !(1..=2).contains(&part) {
            error_and_exit("Day must be between 1..=31, Part between 1..=2")
        }


    }
}

fn error_and_exit(msg: &str) {
    eprintln!("!! {msg} !!\n{MESSAGE}");
    std::process::exit(1);
}

fn generate_rust_project_json() {
    let mut project = RustAnalyzerProject::new();
    project
        .get_sysroot_src()
        .expect("Couldn't find toolchain path, do you have `rustc` installed?");
    project
        .challenges_to_json()
        .expect("Couldn't parse challenge files");

    if project.crates.is_empty() {
        println!(
            "Failed find any challenges, make sure you're in the `advent-of-code-2023` folder"
        );
    } else if project.write_to_disk().is_err() {
        println!("Failed to write rust-project.json to disk for rust-analyzer");
    } else {
        println!("Successfully generated rust-project.json");
        println!("rust-analyzer will now parse exercises, restart your language server or editor")
    }
}

// fn find_challenge(day, part)

const MESSAGE: &'static str = r#"
To use the runner, specify in the first argument the day
and part you want to run, separated with a dot, like so:

Day 5, part 1    ->    cargo r -- 5.1
Day 21, part 2   ->    cargo r -- 21.2

To generate the rust-project.json file necessary for LSP
servers, run `cargo r -- lsp` and restart your language
server.
"#;
