use std::fs::read_to_string;

use itertools::Itertools;

// 39296 too high
// 24542
type TransformationRule = Vec<u64>;
type TransformationStep = Vec<TransformationRule>;

pub fn pb1() {
    let (mut seeds, transformation_steps) = parse("./src/day5.input.txt");
    for step in transformation_steps {
        for i in 0..seeds.len() {
            let seed = seeds[i];
            let mut dest = seed;
            for line in &step {
                if seed >= line[1] && seed <= line[1] + line[2] {
                    dest = line[0] + (seed - line[1]);
                }
            }
            seeds[i] = dest;
        }
    }
    dbg!(seeds.iter().min());
}

fn is_included(p: u64, rule: &TransformationRule) -> bool {
    p >= rule[1] && p <= rule[1] + rule[2]
}

fn map(p: u64, rule: &TransformationRule) -> u64 {
    println!("{} {}", p, rule[1]);
    rule[0] + (p - rule[1])
}
pub fn pb2() {
    let (seeds, transformation_steps) = parse("./src/day5.input.txt");
    // make intervals out of the seeds
    let mut seed_ranges = seeds
        .chunks(2)
        .map(|w| (0, w[0], w[0] + w[1]))
        .collect::<Vec<_>>();
    let mut res = vec![];
    while let Some((iteration, low, high)) = seed_ranges.pop() {
        if iteration == transformation_steps.len() {
            res.push(low);
        } else {
            let mut found_one = false;
            for rule in &transformation_steps[iteration] {
                if is_included(low, rule) {
                    let cutoff = high.min(rule[1] + rule[2]);
                    seed_ranges.push((iteration + 1, map(low, rule), map(cutoff, rule)));
                    if cutoff < high {
                        // not fully included in
                        seed_ranges.push((iteration, cutoff + 1, high));
                    }
                    found_one = true;
                    break;
                } else if is_included(high, rule) {
                    let cutoff = rule[1];
                    // necessarily not included in, as the fully included in is handled above
                    seed_ranges.push((iteration + 1, map(cutoff, rule), map(high, rule)));
                    if cutoff > low {
                        seed_ranges.push((iteration, low, cutoff - 1));
                    }
                    found_one = true;
                    break;
                }
            }
            if !found_one {
                // we simply pass along if we have found no mapping
                seed_ranges.push((iteration + 1, low, high));
            }
        }
    }
    dbg!(res.iter().min());
}

fn parse(path: &str) -> (Vec<u64>, Vec<TransformationStep>) {
    let file = read_to_string(path).unwrap();
    let mut lines = file.lines();
    let seeds = &lines.next().unwrap()[7..];
    let seeds = seeds
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();
    let mut correspondances = vec![];
    let mut curr: TransformationStep = vec![];
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
