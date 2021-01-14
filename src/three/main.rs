use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/three/input")?;

    let lines = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{}", count_trees(lines, 3, 1));

    Ok(())
}

fn count_trees(lines: Vec<Vec<char>>, x_step: usize, y_step: usize) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut tree_count = 0;

    while y < lines.len() {
        if let '#' = effective(&lines[y], x) {
            tree_count += 1;
        }

        y += y_step;
        x += x_step;
    }

    tree_count
}

fn effective(chars: &[char], x: usize) -> char {
    let wrapped = x % chars.len();

    chars[wrapped]
}
