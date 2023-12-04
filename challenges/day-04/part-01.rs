use std::{collections::HashSet, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-04/input.txt"))?;

    let mut card_pile_value: usize = 0;
    for line in input.lines() {
        let mut nums: Vec<Vec<usize>> = line
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .split("|")
            .map(|nums_str| {
                nums_str
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        let (winning, scratched) = (nums.remove(0), nums.remove(0));

        let winning_set: HashSet<usize> = HashSet::from_iter(winning);

        let num_matching = scratched
            .iter()
            .filter(|num| winning_set.contains(num))
            .count() as u32;

        if num_matching > 0 {
            card_pile_value += usize::pow(2, num_matching - 1);
        }
    }

    println!("Sum: {card_pile_value}");

    Ok(())
}
