use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let input = fs::read_to_string(Path::new("./challenges/day-09/input.txt"))?;

  println!("Hello from day 9 part 1");
  println!("Input file: {}", &input);

  Ok(())
}