use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string(Path::new("./challenges/day-06/input.txt"))?;

    let mut lines = input.lines().map(|l| {
        l.split_ascii_whitespace()
            .skip(1)
            .map(|n| n.parse::<f64>().unwrap())
    });

    // zip the two lines of the input file together into a (time limit, distance) tuple
    let races = lines.next().unwrap().zip(lines.next().unwrap());

    let mut product: usize = 1;
    for (time_limit, distance) in races {
        // h - time holding the button
        // l - time limit of the race
        // d - distance
        // r - record distance
        //
        // d = -h(h - l)
        // d > rd
        // combined: -h(h - l) > r
        // rearranged: h^2 - l*h + r < 0
        // Need to find the range of integers of h that satisfy the equation
        let (root_1, root_2) = find_quadratic_roots(1.0, -1.0 * time_limit, distance);

        let lower_bound = f64::min(root_1, root_2).ceil() as usize;
        let upper_bound = f64::max(root_1, root_2).floor() as usize;

        product *= upper_bound - lower_bound + 1;
    }

    println!("{product}");

    Ok(())
}

fn find_quadratic_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let d = f64::powi(b, 2) - 4.0 * a * c;
    if d < 0.0 {
        panic!("Imaginary roots");
    }
    let root_1 = (-1.0 * b + f64::sqrt(d)) / (2.0 * a);
    let root_2 = (-1.0 * b - f64::sqrt(d)) / (2.0 * a);
    (root_1, root_2)
}
