use std::{fs, path::Path};

use itertools::Itertools;
use num::abs;

const EXPANSION_FACTOR: usize = 1_000_000;

// For part 2, instead of actually expanding the Vec<Vec<char>>'s rows/cols in memory,
// we can compute the galaxies' coordinates by keeping track of "clear" rows/cols indicies'.
// Then, with the newly expanded coords, compute the manhattan distance in the same way as pt 1.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-11/input.txt"))?;

    let image: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // indicies of all the columns with only '.' characters
    let clear_row_indicies: Vec<usize> = (0..image.len())
        .filter(|&row_i| image[row_i].iter().all(|&col| col == '.'))
        .collect();
    let clear_col_indicies: Vec<usize> = (0..image[0].len())
        .filter(|&col_i| image.iter().all(|row| row[col_i] == '.'))
        .collect();

    // find all galaxy (x, y) coordinates
    let mut galaxies: Vec<(usize, usize)> = vec![];
    for (row_i, row) in image.iter().enumerate() {
        for (col_i, char) in row.iter().enumerate() {
            if *char == '#' {
                let num_clear_rows = clear_row_indicies.iter().filter(|&&i| i < row_i).count();
                let num_clear_cols = clear_col_indicies.iter().filter(|&&i| i < col_i).count();

                // calculate the actual galaxy coordinates based on how
                // many clear rows and columns are before row_i and col_i
                let expanded_x = col_i + (num_clear_cols * EXPANSION_FACTOR) - num_clear_cols;
                let expanded_y = row_i + (num_clear_rows * EXPANSION_FACTOR) - num_clear_rows;

                galaxies.push((expanded_x, expanded_y));
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
