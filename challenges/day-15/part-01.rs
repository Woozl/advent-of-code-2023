use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-15/input.txt"))?;

    let sum = input.split(',').fold(0, |sum, string| sum + hash(string));

    println!("Sum: {}", sum);

    Ok(())
}

fn hash(string: &str) -> usize {
    string.chars().fold(0, |mut current_val, char| {
        current_val += char as usize;
        current_val *= 17;
        current_val %= 256;
        current_val
    })
}
