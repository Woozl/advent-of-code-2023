use std::{collections::HashSet, fs, path::Path};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Laser {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Laser {
    fn redirect(&self, velocity: (isize, isize)) -> Self {
        Laser {
            position: (self.position.0 + velocity.0, self.position.1 + velocity.1),
            velocity,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-16/input.txt"))?;

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut energized: HashSet<(isize, isize)> = HashSet::new();

    // create a stack of Laser structs to perform a depth-first search on,
    // starting with the first laser in the top left corner
    let mut lasers: Vec<Laser> = vec![Laser {
        position: (0, 0), // top left
        velocity: (1, 0), // travelling east
    }];

    let mut visited: HashSet<Laser> = HashSet::new();

    while lasers.len() > 0 {
        let laser = lasers.pop().unwrap();

        // if we've already explored the paths for this exact laser,
        // no need to recalculate and potentially end up in an infinite loop
        if visited.contains(&laser) {
            continue;
        }
        visited.insert(laser.clone());

        energized.insert(laser.position);

        let char = grid[laser.position.1 as usize][laser.position.0 as usize];
        let next_lasers: Vec<Laser> = match char {
            '-' => {
                match laser.velocity {
                    (0, -1) | (0, 1) => vec![laser.redirect((-1, 0)), laser.redirect((1, 0))], // moving N or S -> out W and E
                    v => vec![laser.redirect(v)], // same dir if moving parallel
                }
            }
            '|' => {
                match laser.velocity {
                    (1, 0) | (-1, 0) => vec![laser.redirect((0, -1)), laser.redirect((0, 1))], // moving E or W -> out N and S
                    v => vec![laser.redirect(v)], // same dir if moving parallel
                }
            }
            '\\' => {
                match laser.velocity {
                    (0, -1) => vec![laser.redirect((-1, 0))], // moving N -> out W
                    (0, 1) => vec![laser.redirect((1, 0))],   // moving S -> out E
                    (1, 0) => vec![laser.redirect((0, 1))],   // moving E -> out S
                    (-1, 0) => vec![laser.redirect((0, -1))], // moving W -> out N
                    v => panic!("Bad velocity: {:?}", v),
                }
            }
            '/' => {
                match laser.velocity {
                    (0, -1) => vec![laser.redirect((1, 0))], // moving N -> out E
                    (0, 1) => vec![laser.redirect((-1, 0))], // moving S -> out W
                    (1, 0) => vec![laser.redirect((0, -1))], // moving E -> out N
                    (-1, 0) => vec![laser.redirect((0, 1))], // moving W -> out S
                    v => panic!("Bad velocity: {:?}", v),
                }
            }
            '.' => vec![laser.redirect(laser.velocity)], // no effect on velocity
            _ => panic!("Bad char {char}"),
        };

        // remove lasers that went out of bounds
        let mut filtered_lasers: Vec<Laser> = next_lasers
            .into_iter()
            .filter(|laser| {
                let (x, y) = laser.position;
                x >= 0 && x < grid[0].len() as isize && y >= 0 && y < grid.len() as isize
            })
            .collect();

        lasers.append(&mut filtered_lasers);
    }

    println!("Num energized cells: {}", energized.len());

    Ok(())
}
