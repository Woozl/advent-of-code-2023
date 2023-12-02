use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-01/input.txt"))?;

    let needles: Vec<(&str, usize)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let mut calibration: usize = 0;

    for line in input.lines() {
        // Create a vector with tuples for each (number, location)
        let mut matches: Vec<(usize, usize)> = needles
            .iter()
            .map(|(word, value)| {
                line.match_indices(word)
                    .map(move |(location, _)| (*value, location))
            })
            .flatten()
            .collect();

        // order by the location
        matches.sort_by_key(|el| el.1);

        let first_digit = matches.first().unwrap().0;
        let second_digit = matches.last().unwrap().0;
        let combination = format!("{first_digit}{second_digit}").parse::<usize>()?;

        calibration += combination;
    }

    println!("Calibration: {calibration}");

    Ok(())
}
