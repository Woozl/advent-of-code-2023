use regex::Regex;
use std::{fs, path::Path};

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

    /// Returns true if this Round goes over another Round `limit`.
    /// `limit` represents the true number of blocks in the bag.
    /// "Goes over" if any color is greater than `limit`s colors.
    fn is_over_limit(&self, limit: &Self) -> bool {
        self.0 > limit.0 || self.1 > limit.1 || self.2 > limit.2
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-02/input.txt"))?;
    let limit = Round(12, 13, 14);
    let mut sum: usize = 0;

    for (game, line) in input.lines().enumerate() {
        let game_id = game + 1;
        let rounds = line.split(";").map(Round::new);

        let mut over = false;
        for round in rounds {
            if round.is_over_limit(&limit) {
                over = true;
                break;
            }
        }

        if over {
            continue;
        } else {
            sum += game_id
        }
    }

    println!("Game IDs sum: {}", sum);

    Ok(())
}
