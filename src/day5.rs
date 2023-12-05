use std::fs::read_to_string;

use itertools::Itertools;

// 39296 too high
// 24542
type Correspondance = Vec<Vec<u64>>;

pub fn pb1() {
    let (mut seeds, correspondances) = parse("./src/day5.input.txt");
    for correspondance in correspondances {
        for i in 0..seeds.len() {
            let seed = seeds[i];
            let mut dest = seed;
            for line in &correspondance {
                if seed >= line[1] && seed <= line[1] + line[2] {
                    dest = line[0] + (seed - line[1]);
                }
            }
            seeds[i] = dest;
        }
    }
    dbg!(seeds.iter().min());
}

pub fn pb2() {}

fn parse(path: &str) -> (Vec<u64>, Vec<Correspondance>) {
    let file = read_to_string(path).unwrap();
    let mut lines = file.lines();
    let seeds = &lines.next().unwrap()[7..];
    let seeds = seeds
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();
    let mut correspondances = vec![];
    let mut curr: Correspondance = vec![];
    while let Some(l) = lines.next() {
        if l.is_empty() {
            lines.nth(0);
            correspondances.push(curr);
            curr = vec![];
            continue;
        }
        curr.push(
            l.split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec(),
        );
    }
    correspondances.push(curr);
    (seeds, correspondances)
}
