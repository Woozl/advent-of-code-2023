use std::{fs, path::Path};

use itertools::Itertools;

const DEFAULT_BUCKET: Vec<(&str, usize)> = Vec::new();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-15/input.txt"))?;

    let sequence = input.split(',');

    let mut hashmap = [DEFAULT_BUCKET; 256];

    for string in sequence {
        if string.contains('=') {
            let [label, lens] = *string.split('=').collect::<Vec<&str>>().as_slice() else {
                panic!("Bad input: {}", string);
            };
            let new_kv_pair = (label, lens.parse::<usize>().unwrap());
            let bucket_id = hash(label);
            let bucket = &mut hashmap[bucket_id];

            // if the label is already in the bucket, we want to update the lens
            if let Some((preexisting_lens_index, _)) =
                bucket.iter().find_position(|&&(key, _)| key == label)
            {
                bucket[preexisting_lens_index] = new_kv_pair;
            } else {
                bucket.push(new_kv_pair);
            }
        } else if string.contains('-') {
            let label = string.replace('-', "");
            let bucket_id = hash(label.as_str());
            let bucket = &mut hashmap[bucket_id];

            if let Some((lens_to_remove, _)) = bucket
                .iter()
                .find_position(|&&(key, _)| key == label.as_str())
            {
                bucket.remove(lens_to_remove);
            }
        } else {
            panic!("Bad input: {}", string);
        }
    }

    let focusing_power = hashmap
        .into_iter()
        .enumerate()
        .fold(0, |power, (box_index, bucket)| {
            bucket.into_iter().enumerate().fold(
                0,
                |bucket_power, (slot_index, (_, focal_length))| {
                    bucket_power + (box_index + 1) * (slot_index + 1) * focal_length
                },
            ) + power
        });

    println!("Focusing power: {}", focusing_power);

    Ok(())
}

fn hash(string: &str) -> usize {
    string.chars().fold(0, |mut current_val, char| {
        current_val += char as usize;
        current_val *= 17;
        current_val %= 256;
        current_val
    })
}
