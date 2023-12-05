#!/bin/bash

# Create 31 folders
for ((i = 1; i <= 25; i++)); do
  # Create folder with the name day-i
  folder_name=$(printf "day-%02d" $i)
  mkdir -p "$folder_name"

  # Create input.txt with the specified text
  echo "Replace this file with the input data." >"$folder_name/input.txt"

  # Create part-01 file
  echo "use std::{fs, path::Path};" >"$folder_name/part-01.rs"
  echo "" >>"$folder_name/part-01.rs"
  echo "fn main() -> Result<(), Box<dyn std::error::Error>> {" >>"$folder_name/part-01.rs"
  echo "  let input = fs::read_to_string(Path::new(\"./challenges/$folder_name/input.txt\"))?;" >>"$folder_name/part-01.rs"
  echo "" >>"$folder_name/part-01.rs"
  echo "  println!(\"Hello from day $i part 1\");" >>"$folder_name/part-01.rs"
  echo "  println!(\"Input file: {}\", &input);" >>"$folder_name/part-01.rs"
  echo "" >>"$folder_name/part-01.rs"
  echo "  Ok(())" >>"$folder_name/part-01.rs"
  echo "}" >>"$folder_name/part-01.rs"

  # Create part-02 file
  echo "use std::{fs, path::Path};" >"$folder_name/part-02.rs"
  echo "" >>"$folder_name/part-02.rs"
  echo "fn main() -> Result<(), Box<dyn std::error::Error>> {" >>"$folder_name/part-02.rs"
  echo "  let input = fs::read_to_string(Path::new(\"./challenges/$folder_name/input.txt\"))?;" >>"$folder_name/part-02.rs"
  echo "" >>"$folder_name/part-02.rs"
  echo "  println!(\"Hello from day $i part 2\");" >>"$folder_name/part-02.rs"
  echo "  println!(\"Input file: {}\", &input);" >>"$folder_name/part-02.rs"
  echo "" >>"$folder_name/part-02.rs"
  echo "  Ok(())" >>"$folder_name/part-02.rs"
  echo "}" >>"$folder_name/part-02.rs"

  # Replace placeholders with the current day in the created files
  sed -i "s/{{CURRENT-DAY}}/$i/g" "$folder_name/part-01.rs"
  sed -i "s/{{CURRENT-DAY}}/$i/g" "$folder_name/part-02.rs"
done
