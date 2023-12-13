use std::{collections::HashSet, fs, path::Path};

use itertools::Itertools;
use num::abs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-11/input.txt"))?;

    let uncorrected_image: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let image = expand_image(uncorrected_image, 2);

    // find all galaxy (x, y) coordinates
    let mut galaxies: Vec<(usize, usize)> = vec![];
    for (row_i, row) in image.iter().enumerate() {
        for (col_i, char) in row.iter().enumerate() {
            if *char == '#' {
                galaxies.push((col_i, row_i));
            }
        }
    }

    // get all combinations of galaxies so we don't double count paths
    let galaxy_pairs: Vec<((usize, usize), (usize, usize))> =
        galaxies.into_iter().tuple_combinations().collect();

    // sum the manhattan distances of all galaxy pairs
    let path_sum = galaxy_pairs
        .into_iter()
        .fold(0, |sum, (start, end)| sum + manhattan_distance(start, end));

    println!("Sum of all paths: {}", path_sum);

    Ok(())
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    (abs(a.0 as isize - b.0 as isize) + abs(a.1 as isize - b.1 as isize)) as usize
}

fn expand_image(image: Vec<Vec<char>>, expansion_factor: usize) -> Vec<Vec<char>> {
    // expand rows
    let row_expanded_image: Vec<Vec<char>> = image
        .into_iter()
        .flat_map(|r| {
            let n = if r.iter().all(|&c| c == '.') {
                expansion_factor
            } else {
                1
            };
            std::iter::repeat(r).take(n)
        })
        .collect();

    // indicies of all the columns with only '.' characters
    let clear_cols_indicies: HashSet<usize> = (0..row_expanded_image[0].len())
        .filter(|&col_i| row_expanded_image.iter().all(|row| row[col_i] == '.'))
        .collect();

    // expand cols
    let expanded_image: Vec<Vec<char>> = row_expanded_image
        .into_iter()
        .map(|r| {
            r.into_iter()
                .enumerate()
                .flat_map(|(col_i, char)| {
                    let n = if clear_cols_indicies.contains(&col_i) {
                        expansion_factor
                    } else {
                        1
                    };
                    std::iter::repeat(char).take(n)
                })
                .collect()
        })
        .collect();

    expanded_image
}
