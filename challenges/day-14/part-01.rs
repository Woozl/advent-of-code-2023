use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-14/input.txt"))?;

    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    for ri in 0..grid.len() {
        for ci in 0..grid[0].len() {
            if grid[ri][ci] == 'O' {
                let mut falling_ri = ri as isize;
                while falling_ri > 0 && grid[(falling_ri - 1) as usize][ci] == '.' {
                    falling_ri -= 1;
                }

                grid[ri][ci] = '.';
                grid[falling_ri as usize][ci] = 'O';
            }
        }
    }

    let load = calculate_load(&grid);
    println!("Load: {}", load);

    Ok(())
}

fn calculate_load(grid: &Vec<Vec<char>>) -> usize {
    grid.iter().enumerate().fold(0, |load, (ri, row)| {
        let num_rocks = row.iter().filter(|&&c| c == 'O').count();
        let position_from_south = grid.len() - ri;
        load + num_rocks * position_from_south
    })
}
