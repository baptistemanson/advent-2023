use std::cmp;

use crate::data2::INPUT;

pub fn pb1() {
    let mut sum: i32 = 0;
    for l in INPUT.lines() {
        let (game_nb, turns) = l[5..].split_once(":").unwrap();
        if !turns.split([',', ';']).any(is_invalid) {
            sum += game_nb.parse::<i32>().unwrap();
        };
    }
    dbg!(sum);
}

// eg " 1 red, 3 blue, 4 green; 1 red, 3 blue, 4 green;"
fn is_invalid(turns: &str) -> bool {
    let (nb, color) = turns[1..].split_once(" ").unwrap();
    let nb = nb.parse::<i32>().unwrap();
    color == "red" && nb > 12 || color == "green" && nb > 13 || color == "blue" && nb > 14
}

pub fn pb2() {
    let mut sum: i32 = 0;
    for l in INPUT.lines() {
        let (_game, turns) = l.split_once(':').unwrap();
        let [r, g, b] = turns.split([',', ';']).fold([0, 0, 0], |mut acc, turn| {
            let (nb, color) = turn[1..].split_once(" ").unwrap();
            let color = match color {
                "red" => 0,
                "green" => 1,
                _ => 2,
            };
            acc[color] = cmp::max(nb.parse::<i32>().unwrap(), acc[color]);
            acc
        });
        sum += r * g * b;
    }
    dbg!(sum);
}
