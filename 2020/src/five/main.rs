use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/five/input")?;

    let max_seat_index = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| to_seat_id(&x))
        .max()
        .unwrap();

    println!("{}", max_seat_index);

    Ok(())
}

fn to_seat_id(seat_str: &str) -> u32 {
    let binary_row_str = String::from(&seat_str[0..7])
        .replace('F', "0")
        .replace('B', "1");

    let binary_row = u32::from_str_radix(&binary_row_str, 2).unwrap();

    let binary_seat_idx = String::from(&seat_str[7..10])
        .replace('R', "1")
        .replace('L', "0");

    let binary_seat = u32::from_str_radix(&binary_seat_idx, 2).unwrap();

    (binary_row * 8) + binary_seat
}
