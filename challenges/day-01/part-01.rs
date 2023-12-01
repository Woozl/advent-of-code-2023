use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-1/input.txt"))?;

    let mut calibration: usize = 0;

    for line in input.lines() {
        let mut first_digit: char = 'x';
        let mut second_digit: char = 'x';
        for char in line.chars() {
            if char.is_ascii_digit() {
                if first_digit == 'x' {
                    first_digit = char;
                }
                else {
                    second_digit = char;
                }

                // if there's only one digit in the line, the output is that
                // digit twice. This code sets both variables to the first
                // digit found.
                if second_digit == 'x' {
                    second_digit = char;
                }
            }
        }

        let combination = format!("{first_digit}{second_digit}").parse::<usize>()?;

        calibration += combination;
    }

    println!("Calibration: {calibration}");

    Ok(())
}