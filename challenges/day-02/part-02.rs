use regex::Regex;
use std::{cmp::max, fs, path::Path};

/// (red, green, blue)
/// Each handful the elf pulls from the bag
#[derive(Debug)]
struct Round(usize, usize, usize);

impl Round {
    /// Generate a Round based on a string slice: `"3 blue, 5 green"` -> `Round(0, 3, 5)`
    fn new(round_str: &str) -> Self {
        let red_re = Regex::new(r"(\d+) red").unwrap();
        let green_re = Regex::new(r"(\d+) green").unwrap();
        let blue_re = Regex::new(r"(\d+) blue").unwrap();

        let red: usize = red_re.captures(round_str).map_or(0, |cap| {
            cap.get(1).unwrap().as_str().parse::<usize>().unwrap()
        });
        let green: usize = green_re.captures(round_str).map_or(0, |cap| {
            cap.get(1).unwrap().as_str().parse::<usize>().unwrap()
        });
        let blue: usize = blue_re.captures(round_str).map_or(0, |cap| {
            cap.get(1).unwrap().as_str().parse::<usize>().unwrap()
        });

        Self(red, green, blue)
    }

    // Given a iterator of `Round`s, return create the `Round` with the
    // minimum viable set of cubes (highest # of red, green, and blue cubes
    // seen in all `Round`s).
    fn minimum_viable_set(rounds: impl Iterator<Item = Round>) -> Self {
        rounds
            .reduce(|m, r| Round(max(m.0, r.0), max(m.1, r.1), max(m.2, r.2)))
            .expect("minimum_viable_set received no rounds")
    }

    fn power(&self) -> usize {
        self.0 * self.1 * self.2
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-02/input.txt"))?;
    let mut sum: usize = 0;

    for line in input.lines() {
        let rounds = line.split(";").map(Round::new);
        let mvs: Round = Round::minimum_viable_set(rounds);
        let power = mvs.power();
        sum += power;
    }

    println!("Sum: {}", sum);

    Ok(())
}
