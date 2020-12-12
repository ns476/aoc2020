use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/three/input")?;

    let lines = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let trees = vec![
        count_trees(&lines, 1, 1),
        count_trees(&lines, 3, 1),
        count_trees(&lines, 5, 1),
        count_trees(&lines, 7, 1),
        count_trees(&lines, 1, 2),
    ];

    println!("{:?}", trees);

    println!("{}", trees.iter().fold(1, |x, y| x * y));

    Ok(())
}

fn count_trees(lines: &Vec<Vec<char>>, x_step: usize, y_step: usize) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut tree_count = 0;

    while y < lines.len() {
        match effective(&lines[y], x) {
            '#' => tree_count += 1,
            _ => (),
        }

        y += y_step;
        x += x_step;
    }

    tree_count
}

fn effective(chars: &Vec<char>, x: usize) -> char {
    let wrapped = x % chars.len();

    chars[wrapped]
}
