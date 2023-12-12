use std::{collections::HashMap, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-10/input.txt"))?;

    let mut starting_loc = (0, 0);
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row_i, l)| {
            if let Some(col_i) = l.find('S') {
                starting_loc = (col_i, row_i);
            }
            l.chars().collect()
        })
        .collect();
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    let map: HashMap<char, Vec<(isize, isize)>> = HashMap::from([
        ('|', vec![(0, -1), (0, 1)]),                  // N, S
        ('-', vec![(-1, 0), (1, 0)]),                  // W, E
        ('L', vec![(0, -1), (1, 0)]),                  // N, E
        ('J', vec![(0, -1), (-1, 0)]),                 // N, W
        ('7', vec![(-1, 0), (0, 1)]),                  // W, S
        ('F', vec![(1, 0), (0, 1)]),                   // E, S
        ('S', vec![(0, -1), (0, 1), (1, 0), (-1, 0)]), // N, S, E, W
    ]);

    let mut current_loc = starting_loc;
    let mut prev_step_dir: (isize, isize) = (0, 0);
    let mut num_steps: usize = 0;
    loop {
        let current_char = grid[current_loc.1][current_loc.0];
        let dirs = map.get(&current_char).unwrap();

        let valid_dirs = dirs.iter().filter(|&&d| d != opposite(prev_step_dir));

        for dir in valid_dirs {
            let x = current_loc.0 as isize + dir.0;
            let y = current_loc.1 as isize + dir.1;
            if x >= 0 && x < w && y >= 0 && y < h {
                let next_char = grid[y as usize][x as usize];
                let connecting_dir = opposite(*dir);
                if let Some(connections) = map.get(&next_char) {
                    if let Some(_) = connections.iter().find(|&&c| c == connecting_dir) {
                        current_loc = (x as usize, y as usize);
                        num_steps += 1;
                        prev_step_dir = *dir;
                        break;
                    }
                }
            }
        }

        if current_loc == starting_loc {
            break;
        }
    }

    let remainder = num_steps % 2;
    let steps_to_furthest_point = num_steps / 2 + remainder;
    println!("Num steps to furthest point: {}", steps_to_furthest_point);

    Ok(())
}

fn opposite(dir: (isize, isize)) -> (isize, isize) {
    (-dir.0, -dir.1)
}
