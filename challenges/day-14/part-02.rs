use std::{fs, path::Path, collections::HashMap};

const NUM_CYCLES: usize = 1_000_000_000;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-14/input.txt"))?;

    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let mut current_cycle: usize = 1;
    while current_cycle < NUM_CYCLES {
        cycle(&mut grid);
        
        if let Some(&cached_length) = cache.get(&grid.clone()) {
            let length = current_cycle - cached_length;
            let remaining_cycles = (NUM_CYCLES - current_cycle) % length;
            for _ in 0..remaining_cycles {
                cycle(&mut grid);
            }
            break;
        }
        
        cache.insert(grid.clone(), current_cycle);
        current_cycle += 1;
    }

    let load = calculate_load(&grid);
    println!("Load: {}", load);

    Ok(())
}

enum Direction { North, West, South, East }

fn cycle(grid: &mut Vec<Vec<char>>) {
    tilt(grid, Direction::North);
    tilt(grid, Direction::West);
    tilt(grid, Direction::South);
    tilt(grid, Direction::East);
}

fn tilt(grid: &mut Vec<Vec<char>>, direction: Direction) {
    match direction {
        Direction::North => {
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
        },
        Direction::South => {
            for ri in (0..grid.len()).rev() {
                for ci in 0..grid[0].len() {
                    if grid[ri][ci] == 'O' {
                        let mut falling_ri = ri;
                        while falling_ri < grid.len() - 1 && grid[falling_ri + 1][ci] == '.' {
                            falling_ri += 1;
                        }
                        grid[ri][ci] = '.';
                        grid[falling_ri][ci] = 'O';
                    }
                }
            }
        },
        Direction::West => {
            for ci in 0..grid[0].len() {
                for ri in 0..grid.len() {
                    if grid[ri][ci] == 'O' {
                        let mut falling_ci = ci as isize;
                        while falling_ci > 0 && grid[ri][(falling_ci - 1) as usize] == '.' {
                            falling_ci -= 1;
                        }
                        grid[ri][ci] = '.';
                        grid[ri][falling_ci as usize] = 'O';
                    }
                }
            }
        },
        Direction::East => {
            for ci in (0..grid[0].len()).rev() {
                for ri in 0..grid.len() {
                    if grid[ri][ci] == 'O' {
                        let mut falling_ci = ci;
                        while falling_ci < grid[0].len() - 1 && grid[ri][falling_ci + 1] == '.' {
                            falling_ci += 1;
                        }
                        grid[ri][ci] = '.';
                        grid[ri][falling_ci] = 'O';
                    }
                }
            }
        },
    }
}

fn calculate_load(grid: &Vec<Vec<char>>) -> usize {
    grid.iter().enumerate().fold(0, |load, (ri, row)| {
        let num_rocks = row.iter().filter(|&&c| c == 'O').count();
        let position_from_south = grid.len() - ri;
        load + num_rocks * position_from_south
    })
}
