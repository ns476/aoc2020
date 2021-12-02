use cached::proc_macro::cached;
use cached::stores::UnboundCache;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let file = File::open("src/tenplus/input")?;

    let elems = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    let jolts = build_jolts(&elems);

    let target = *(jolts.iter().max().unwrap());
    let jolts_set: HashSet<i64> = jolts.iter().cloned().collect();

    dbg!(count_paths(&jolts_set, 0, target));

    Ok(())
}

fn count_paths(jolts_set: &HashSet<i64>, current: i64, target: i64) -> i64 {
    #[cached(
        type = "UnboundCache<(i64, i64), i64>",
        create = "{ UnboundCache::new() }",
        convert = r#"{ (current, target) }"#
    )]
    fn count_paths_inner(jolts_set: &HashSet<i64>, current: i64, target: i64) -> i64 {
        if target == current {
            return 1;
        }

        if !jolts_set.contains(&current) {
            return 0;
        }

        count_paths(jolts_set, current + 1, target)
            + count_paths(jolts_set, current + 2, target)
            + count_paths(jolts_set, current + 3, target)
    }

    count_paths_inner(jolts_set, current, target)
}

fn build_jolts(elems: &[i64]) -> Vec<i64> {
    let max: i64 = *(elems.iter().max().unwrap());

    let mut jolts: Vec<i64> = std::iter::once(0)
        .chain(elems.iter().cloned())
        .chain(std::iter::once(max + 3))
        .collect();

    jolts.sort_unstable();

    jolts
}
