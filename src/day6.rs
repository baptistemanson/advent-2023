use std::fs::read_to_string;

use itertools::Itertools;

pub fn pb1() {
    let (ts, ds) = parse("./src/day6.input.txt");
    let mut tot = 1;
    for i in 0..ts.len() {
        // -x^2 +tx - d
        tot *= resolve(-1, ts[i], -ds[i]);
    }
    dbg!(tot);
}

fn resolve(a: i64, b: i64, c: i64) -> i64 {
    let det = f64::sqrt((b * b - 4 * a * c) as f64);
    let min = (-b as f64 + det) / (2. * a as f64);
    let max = (-b as f64 - det) / (2. * a as f64);
    // double check
    let extrema_delta = match (max.fract() > 0., min.fract() > 0.) {
        (true, true) => 1,
        (false, true) => 0,
        (true, false) => 0,
        (false, false) => -1,
    };
    let nb_better = (max).floor() as i64 - (min).ceil() as i64 + extrema_delta;
    nb_better
}

// 39132886 good
pub fn pb2() {
    let (ts, ds) = parse("./src/day6.input2.txt");
    dbg!(resolve(-1, ts[0], -ds[0]));
}

// too low
fn parse(path: &str) -> (Vec<i64>, Vec<i64>) {
    let file = read_to_string(path).unwrap();
    let mut lines = file.lines();
    let times = lines.next().unwrap()[6..]
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect_vec();
    let distances = lines.next().unwrap()[10..]
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect_vec();
    (times, distances)
}
