use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-12/input.txt"))?;

    let row = input.lines().map(|l| {
        let mut split = l.split_ascii_whitespace();
        let springs = split.next().unwrap();
        let broken_ranges: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        (springs, broken_ranges)
    });

    row.for_each(|v| println!("{:?}", v));

    Ok(())
}
