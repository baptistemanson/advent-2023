use std::{collections::HashSet, fs::read_to_string};

// 39296 too high
// 24542
pub fn pb1() {
    let mut sum = 0;
    for l in read_to_string("./src/day4.input.txt").unwrap().lines() {
        let numbers = l.split_once(":").unwrap().1;
        let score = nb_win(numbers);
        if score > 0 {
            sum += 1 << score - 1;
        }
    }
    dbg!(sum);
}

fn nb_win(numbers: &str) -> usize {
    let (winning_nb, played_nb) = numbers.split_once("|").unwrap();
    let winning_nb = parse(winning_nb);
    let played_nb = parse(played_nb);
    played_nb.intersection(&winning_nb).count()
}

fn parse(winning_nb: &str) -> HashSet<&str> {
    winning_nb
        .trim()
        .split(" ")
        .filter(|a| a.len() > 0)
        .collect()
}
// 229 too low
// 8736438
pub fn pb2() {
    let mut stock = vec![1; 205];
    for (i, l) in read_to_string("./src/day4.input.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        let numbers = l.split_once(":").unwrap().1;
        for x in 1..=nb_win(numbers) {
            stock[i + x] += stock[i];
        }
    }
    dbg!(stock.iter().sum::<i32>());
}
