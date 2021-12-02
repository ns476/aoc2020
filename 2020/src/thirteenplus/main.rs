use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/thirteen/input")?;

    let elems: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();

    let leaving: Vec<Option<i64>> = elems[1]
        .split(',')
        .map(|x| x.to_string().parse::<i64>().ok())
        .collect();

    let mut buses_by_index: BTreeMap<i64, i64> = BTreeMap::new();

    for (i, x) in leaving.iter().enumerate() {
        if let Some(bus) = x {
            buses_by_index.insert(i as i64, *bus);
        }
    }

    let eqns: Vec<(i64, i64)> = buses_by_index
        .iter()
        .map(|(&index, &bus)| ((bus - index) % bus, bus))
        .collect();

    dbg!(&eqns);

    #[allow(non_snake_case)]
    let M: i64 = eqns.iter().map(|(_, m_i)| m_i).product();

    let result: i64 = eqns
        .iter()
        .map(|(a_i, m_i)| {
            let b_i = M / m_i;
            let b_i_tick = modinverse::modinverse(b_i, *m_i).unwrap();

            a_i * b_i * b_i_tick
        })
        .sum();

    dbg!(result % M);
    Ok(())
}
