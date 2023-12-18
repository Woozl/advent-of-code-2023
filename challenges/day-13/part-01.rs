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
        if stack_pointer != i && *string == strings[stack_pointer] {
            let mut left = stack_pointer as isize;
            let mut right = i as isize;

            while left >= 0 && right < strings.len() as isize && strings[left as usize] == strings[right as usize] {
                right += 1;
                left -= 1;
            }

            if left == -1 || right == strings.len() as isize {
                return Some(stack_pointer + 1);
            }
        }

        stack_pointer = i;
    }

    None
}
