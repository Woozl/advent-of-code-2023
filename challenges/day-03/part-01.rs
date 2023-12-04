use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-03/input.txt"))?;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum: usize = 0;
    for (ri, row) in grid.iter().enumerate() {
        let mut discovered_number: Vec<char> = vec![];
        let mut start: Option<usize> = None;

        for (ci, col) in row.iter().enumerate() {
            if col.is_ascii_digit() {
                discovered_number.push(*col);
                if start.is_none() {
                    start = Some(ci);
                }
            }

            if (!col.is_ascii_digit() || ci == row.len() - 1) && discovered_number.len() > 0 {
                let str = discovered_number.iter().collect::<String>();
                let digits = str.len();
                let num = str.parse::<usize>()?;

                if has_surrounding_symbol(&grid, ri, start.unwrap(), start.unwrap() + digits - 1) {
                    sum += num;
                }

                discovered_number.clear();
                start = None;
            }
        }
    }

    println!("Sum: {sum}");

    Ok(())
}

fn has_surrounding_symbol(
    grid: &Vec<Vec<char>>,
    row: usize,
    col_start: usize,
    col_end: usize,
) -> bool {
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;
    let r = row as isize;
    let cs = col_start as isize;
    let ce = col_end as isize;

    fn is_symbol(c: char) -> bool {
        !c.is_ascii_digit() && c != '.'
    }

    #[rustfmt::skip]
    let directions: Vec<(isize, isize)> = vec![
        (-1, -1), ( 0,-1), ( 1,-1),
        (-1,  0),          ( 1, 0),
        (-1,  1), ( 0, 1), ( 1, 1),
    ];

    for (dx, dy) in directions {
        let new_r = r + dy;
        let new_cs = cs + dx;
        let new_ce = ce + dx;

        if new_r >= 0 && new_r < h && new_cs >= 0 && new_ce < w {
            for col in new_cs..=new_ce {
                if is_symbol(grid[new_r as usize][col as usize]) {
                    return true;
                }
            }
        }
    }

    false
}
