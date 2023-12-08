use std::{collections::HashMap, fs, path::Path};

use num::Integer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-08/input.txt"))?;

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut initial_locations: Vec<String> = vec![];
    let turn_string = input.lines().take(1).collect::<String>();

    for line in input.lines().skip(2) {
        // populate the map
        let split_line: String = line
            .replace(" = (", " ")
            .replace(", ", " ")
            .replace(")", "");
        let mut l: Vec<String> = split_line
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect();
        let (key, left, right) = (l.remove(0), l.remove(0), l.remove(0));
        map.insert(key.clone(), (left.clone(), right.clone()));

        // also create keep track of all the keys that end with 'A'
        if key.ends_with('A') {
            initial_locations.push(key);
        }
    }

    // get how many steps each path takes to get to a 'xxZ' location
    let path_lengths = initial_locations.into_iter().map(|loc| {
        let mut turns_iter = turn_string.chars().into_iter().cycle();
        let mut curr_loc = loc.clone();
        let mut steps: usize = 0;
        while !curr_loc.ends_with('Z') {
            let turn: char = turns_iter.next().unwrap();
            let crossroad = map.get(&curr_loc).unwrap();
            curr_loc = if turn == 'L' {
                crossroad.0.clone()
            } else {
                crossroad.1.clone()
            };
            steps += 1;
        }
        steps
    });

    // LCM of path_lengths is where they converge
    let lcm: usize = path_lengths.fold(1, |acc, cur| acc.lcm(&cur));

    println!("Steps: {lcm}");

    Ok(())
}
