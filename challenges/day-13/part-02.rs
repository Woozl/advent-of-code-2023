use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-13/input.txt"))?;

    let patterns = input
        .split("\n\n")
        .map(|p| {
            p.lines()
                .map(|l| l.chars().collect())
                .collect::<Vec<Vec<char>>>()
        })
        .map(|pattern| {
            let (w, h) = (pattern[0].len(), pattern.len());
            let rows: Vec<String> = pattern.iter().map(|r| r.iter().collect()).collect();
            let cols: Vec<String> = (0..w)
                .map(|col_i| {
                    (0..h).fold(String::new(), |mut acc, row_i| {
                        acc.push(pattern[row_i][col_i]);
                        acc
                    })
                })
                .collect();
            (rows, cols)
        });

    let mut sum: usize = 0;
    for (rows, cols) in patterns {
        let row_mirror = find_mirror(&rows);
        let col_mirror = find_mirror(&cols);

        if let Some(i) = row_mirror {
            sum += i * 100;
        }
        if let Some(i) = col_mirror {
            sum += i;
        }
    }

    println!("Sum: {}", sum);

    Ok(())
}

fn find_mirror(strings: &Vec<String>) -> Option<usize> {
    let mut stack_pointer: usize = 0;
    for (i, string) in strings.into_iter().enumerate() {
        let comparison = compare(string, &strings[stack_pointer]);
        if stack_pointer != i
            && (comparison == Compare::Smudge || comparison == Compare::PerfectMatch)
        {
            let mut left = stack_pointer as isize;
            let mut right = i as isize;

            let mut num_smudges: usize = 0;
            while left >= 0 && right < strings.len() as isize {
                match compare(&strings[left as usize], &strings[right as usize]) {
                    Compare::NoMatch => {
                        break;
                    }
                    Compare::Smudge => {
                        num_smudges += 1;
                        if num_smudges > 1 {
                            break;
                        }
                    }
                    Compare::PerfectMatch => (),
                };

                right += 1;
                left -= 1;
            }

            if num_smudges == 1 && (left == -1 || right == strings.len() as isize) {
                return Some(stack_pointer + 1);
            }
        }

        stack_pointer = i;
    }

    None
}

#[derive(Debug, PartialEq)]
enum Compare {
    PerfectMatch,
    Smudge,
    NoMatch,
}
fn compare(s1: &String, s2: &String) -> Compare {
    if s1.len() != s2.len() {
        return Compare::NoMatch;
    }

    let mut has_smudge = false;

    for i in 0..s1.len() {
        if s1.as_bytes()[i] != s2.as_bytes()[i] {
            if has_smudge {
                return Compare::NoMatch;
            } else {
                has_smudge = true;
            }
        }
    }

    if has_smudge {
        Compare::Smudge
    } else {
        Compare::PerfectMatch
    }
}
