fn main() {
    eprintln!("{MESSAGE}");
    std::process::exit(1);
}

const MESSAGE: &'static str = r#"
To use the runner, specify the day-part binary:

Day 5, part 1    ->    cargo r --bin 5-1
Day 21, part 2   ->    cargo r --bin 21-2
"#;
