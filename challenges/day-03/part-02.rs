use std::{cell::RefCell, collections::HashMap, fs, path::Path, rc::Rc};

#[derive(Debug)]
struct Cell {
    /// the original character of the grid
    char: char,

    /// If the Cell is part of a number, contains a reference to the number associated
    /// with this island. For example, if this is the grid of Cells:
    ///
    ///     .......
    ///     ..234..
    ///     .......
    ///
    /// The cells for 2, 3, and 4 all point to the same value (usize) 234 on the heap
    island: Option<Rc<RefCell<usize>>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-03/input.txt"))?;

    let mut grid: Vec<Vec<Cell>> = vec![];
    for line in input.lines() {
        let mut pointer: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
        let mut discovered_number: Vec<char> = vec![];

        let mut row: Vec<Cell> = vec![];

        for (ci, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                discovered_number.push(char);
            }

            row.push(Cell {
                char,
                island: if !char.is_ascii_digit() {
                    None
                } else {
                    Some(Rc::clone(&pointer))
                },
            });

            if (!char.is_ascii_digit() || ci == line.len() - 1) && discovered_number.len() > 0 {
                let parsed_num = discovered_number
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()?;
                *pointer.borrow_mut() = parsed_num;

                discovered_number.clear();
                pointer = Rc::new(RefCell::new(0));
            }
        }

        grid.push(row);
    }

    let mut sum_of_ratios: usize = 0;
    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    // scan the entire grid again. If we find a '*', scan its (up to) 9 surrounding
    // `Cell`s and create a set from their `island` pointers. If there are two items
    // in the set, create a ratio and multiple it by the current product_of_ratios to
    // get the output
    for (ri, row) in grid.iter().enumerate() {
        for (ci, cell) in row.iter().enumerate() {
            if cell.char == '*' {
                #[rustfmt::skip]
                let directions: Vec<(isize, isize)> = vec![
                    (-1, -1), ( 0,-1), ( 1,-1),
                    (-1,  0),          ( 1, 0),
                    (-1,  1), ( 0, 1), ( 1, 1),
                ];

                // create Vec `Cell`s surrounding the '*'
                let surrounding: Vec<&Cell> =
                    directions
                        .iter()
                        .fold(vec![], |mut acc: Vec<&Cell>, (dx, dy)| {
                            let new_ri = ri as isize + dy;
                            let new_ci = ci as isize + dx;

                            if new_ci >= 0 && new_ci < w && new_ri >= 0 && new_ri < h {
                                acc.push(&grid[new_ri as usize][new_ci as usize])
                            }

                            acc
                        });

                // set of the unique numbers integers surrounding the asterisk
                let mut surrounding_nums: HashMap<*mut usize, usize> = HashMap::new();
                for cell in surrounding {
                    if let Some(island) = &cell.island {
                        let ptr = island.as_ref().as_ptr();
                        surrounding_nums.insert(ptr, *island.borrow());
                    }
                }

                // if the set has two numbers, multiply them and add them to the sum
                match surrounding_nums.len() {
                    0..=1 => (),
                    2 => sum_of_ratios += surrounding_nums.values().fold(1, |acc, &c| acc * c),
                    _ => panic!("3 or more nums adjacent to a '*'"),
                }
            }
        }
    }

    println!("Sum: {}", sum_of_ratios);

    Ok(())
}
