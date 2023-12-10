use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-09/input.txt"))?;

    let lines = input.lines().map(|l| {
        l.split_ascii_whitespace()
            .map(|n| n.parse::<isize>().unwrap())
    });

    let sum: isize = lines.fold(0, |acc, line| acc + get_preceding_val(line.collect()));

    println!("Sum: {sum}");

    Ok(())
}

fn get_preceding_val(level: Vec<isize>) -> isize {
    if level.iter().all(|&n| n == 0) {
        return 0;
    }

    let next_level = level.windows(2).fold(Vec::new(), |mut acc, window| {
        acc.push(window[1] - window[0]);
        acc
    });

    return level.first().unwrap() - get_preceding_val(next_level);
}
