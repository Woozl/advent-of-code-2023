use std::{collections::HashMap, fs, path::Path};

// Point in Polygon, raycast winding count method
// from this paper: https://web.archive.org/web/20210302180821/http://geomalgorithms.com/a03-_inclusion.html
//
// To handle cases where the ray (+x direction in this implementation) is parallel and
// and intersecting a line of the polygon, follow these rules for counting from the paper:
//
// Edge Crossing Rules:
//   1. an upward edge includes its starting endpoint, and excludes its final endpoint;
//   2. a downward edge excludes its starting endpoint, and includes its final endpoint;
//   3. horizontal edges are excluded
//   4. the edge-ray intersection point must be strictly right of the point P.

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

    // valid connections for each char "pipe" piece
    let char_connections: HashMap<char, Vec<(isize, isize)>> = HashMap::from([
        ('|', vec![(0, -1), (0, 1)]),                  // N, S
        ('-', vec![(-1, 0), (1, 0)]),                  // W, E
        ('L', vec![(0, -1), (1, 0)]),                  // N, E
        ('J', vec![(0, -1), (-1, 0)]),                 // N, W
        ('7', vec![(-1, 0), (0, 1)]),                  // W, S
        ('F', vec![(1, 0), (0, 1)]),                   // E, S
        ('S', vec![(0, -1), (0, 1), (1, 0), (-1, 0)]), // N, S, E, W
    ]);

    // map point (x, y) on the loop to a direction vector for use in the winding raycast count
    let mut loop_directions: HashMap<(usize, usize), (isize, isize)> = HashMap::new();

    let mut current_loc = starting_loc;
    let mut prev_step_dir: (isize, isize) = (0, 0);
    let mut first_step_dir: (isize, isize) = (0, 0);
    loop {
        let current_char = grid[current_loc.1][current_loc.0];

        if prev_step_dir != (0, 0) {
            let loop_direction = match (current_char, prev_step_dir) {
                ('L', (0, 1)) => (0, 1),   // entering L pipe from N
                ('L', (-1, 0)) => (0, -1), // entering L pipe from E
                ('J', (1, 0)) => (0, -1),  // entering J pipe from W
                ('J', (0, 1)) => (0, 1),   // entering J pipe from N
                ('|', _) => prev_step_dir, // vertical pipes always travel the same direction they were entered
                _ => (0, 0)                // every other pipe/entrance dir configuration isn't counted for a +x raycast
            };

            loop_directions.insert(current_loc, loop_direction);
        }

        let dirs = char_connections.get(&current_char).unwrap();

        // don't visit the cell we just came from by filtering out the dir vector opposite the one we took to get here
        let valid_dirs = dirs.iter().filter(|&&d| d != opposite(prev_step_dir));

        for dir in valid_dirs {
            let x = current_loc.0 as isize + dir.0;
            let y = current_loc.1 as isize + dir.1;
            if x >= 0 && x < w && y >= 0 && y < h {
                let next_char = grid[y as usize][x as usize];
                let connecting_dir = opposite(*dir);
                if let Some(connections) = char_connections.get(&next_char) {
                    if let Some(_) = connections.iter().find(|&&c| c == connecting_dir) {
                        if first_step_dir == (0, 0) {
                            first_step_dir = *dir;
                        }
                        current_loc = (x as usize, y as usize);
                        prev_step_dir = *dir;
                        break;
                    }
                }
            }
        }

        // once we've completed the loop, we need to determine the winding direction for the
        // starting value. In other words, we need to figure out which pipe piece the 'S' cell
        // is acting as.
        if current_loc == starting_loc {
            println!("{:?} -> {:?}", prev_step_dir, first_step_dir);

            // this is in effect the same match statement as above, except we're determining
            // the winding direction using the input and output directions rather than the
            // character, since the 'S' cell can act as a wildcard.
            let direction: (isize, isize) = match (prev_step_dir, first_step_dir) {
                ((0, 1), (1, 0)) => (0, 1),    // 'L' piece, S in E out -> S winding
                ((-1, 0), (0, -1)) => (0, -1), // 'L' piece, W in N out -> N winding
                ((1, 0), (0, -1)) => (0, -1),  // 'J' piece, E in N out -> N winding
                ((0, 1), (-1, 0)) => (0, 1),   // 'J' piece, S in W out -> S winding
                ((0, -1), (0, -1)) => (0, -1), // '|' piece travelling north
                ((0, 1), (0, 1)) => (0, 1),    // '|' piece travelling south
                _ => (0, 0)                    // other piece configurations have no impact on winding calculation
            };

            loop_directions.insert(current_loc, direction);
            
            break;
        }
    }

    print_winding_graph(&grid, &loop_directions);

    Ok(())
}

fn print_winding_graph(grid: &Vec<Vec<char>>, loop_directions: &HashMap<(usize, usize), (isize, isize)>) {
    for ri in 0..grid.len() {
        for ci in 0..grid[ri].len() {
            let is_dir = loop_directions.get(&(ci, ri));
            if let Some(dir) = is_dir {
                let arrow = match *dir {
                    (0, 0) => 'o',
                    (0, -1) => '↑',
                    (0, 1) => '↓',
                    (1, 0) => '→',
                    (-1, 0) => '←',
                    _ => '#'
                };
                print!("{arrow}");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
}

fn opposite(dir: (isize, isize)) -> (isize, isize) {
    (-dir.0, -dir.1)
}
