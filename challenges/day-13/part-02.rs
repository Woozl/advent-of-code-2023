use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let input = fs::read_to_string(Path::new("./challenges/day-13/input.txt"))?;

  println!("Hello from day 13 part 2");
  println!("Input file: {}", &input);

  Ok(())
}