use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-05/input.txt"))?;

    let mut input_parts = input.split("\n\n");
    let mut current_values: Vec<usize> = input_parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1) // the "seeds:" text
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    // skip the "seeds" string section, pulled out above in `current_values`
    for mapping_group in input_parts {
        // for each line in the group (skipping the first heading line), create a
        // tuple of the 3 values (dest, source, range)
        let parsed_mapping_group: Vec<(usize, usize, usize)> = mapping_group
            .lines()
            .skip(1)
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
            })
            .map(|mut v| (v.next().unwrap(), v.next().unwrap(), v.next().unwrap()))
            .collect();

        // get the next values iteration based on this map
        current_values = current_values
            .iter()
            .map(|v| map_value(*v, &parsed_mapping_group))
            .collect();
    }

    println!("Min: {}", current_values.iter().min().unwrap());

    Ok(())
}

/// Takes a single value and tries to map it using any of the range maps in the vector.
/// If it doesn't match any of the maps, returns the input value.
fn map_value(value: usize, map: &Vec<(usize, usize, usize)>) -> usize {
    for (dest, source, range) in map {
        let source_range = *source..=(*source + *range);

        if source_range.contains(&value) {
            return (value - source) + dest;
        }
    }

    value
}
