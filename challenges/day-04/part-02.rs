use std::{collections::HashSet, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-04/input.txt"))?;

    // copies of each card #, starts with 1 copy of each
    let mut card_copies = vec![1; input.lines().count()];

    for (card_number, line) in input.lines().enumerate() {
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

        let card_points = scratched
            .iter()
            .filter(|num| winning_set.contains(num))
            .count();

        let copies_of_current_card = card_copies[card_number];
        for i in (card_number + 1)..=(card_number + card_points) {
            card_copies[i] += copies_of_current_card
        }
    }

    println!("Sum: {}", card_copies.iter().fold(0, |acc, c| acc + c));

    Ok(())
}
