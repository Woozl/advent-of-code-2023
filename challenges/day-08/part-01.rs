use std::{collections::HashMap, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-08/input.txt"))?;

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let turn_string = input.lines().take(1).collect::<String>();

    // populate the map
    for line in input.lines().skip(2) {
        let split_line: String = line
            .replace(" = (", " ")
            .replace(", ", " ")
            .replace(")", "");
        let mut l: Vec<String> = split_line
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        map.insert(l.remove(0), (l.remove(0), l.remove(0)));
    }

    let mut current_location = String::from("AAA");
    let mut num_steps: usize = 0;
    let mut turns = turn_string.chars().cycle();
    while current_location != String::from("ZZZ") {
        let crossroad = map.get(&current_location).unwrap().clone();
        let turn = turns.next().unwrap();
        current_location = if turn == 'L' {
            crossroad.0
        } else {
            crossroad.1
        };
        num_steps += 1;
    }

    println!("Number of steps: {num_steps}");

    Ok(())
}
