use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        let input = lines.flatten().collect();
        // part_one(input);
        part_two(input);
    }
}

fn part_one(input: Vec<String>) {
    // We will treat the input as a single line
    // just to simplify index look up
    let line_offset = input.first().unwrap().len() as i32;
    let finput = input.join("");
    let finput_array = finput.chars().collect::<Vec<char>>();
    let mut sum = 0;

    // Avoid checking the same neighbors twice
    let neighbors_3: Vec<i32> = vec![
        -line_offset + -1,
        -line_offset,
        -line_offset + 1,
        -line_offset + 2,
        -line_offset + 3,
        -1,
        3,
        line_offset + -1,
        line_offset,
        line_offset + 1,
        line_offset + 2,
        line_offset + 3,
    ];
    let neighbors_2: Vec<i32> = vec![
        -line_offset + -1,
        -line_offset,
        -line_offset + 1,
        -line_offset + 2,
        -1,
        2,
        line_offset + -1,
        line_offset,
        line_offset + 1,
        line_offset + 2,
    ];
    let neighbors_1: Vec<i32> = vec![
        -line_offset + -1,
        -line_offset,
        -line_offset + 1,
        -1,
        1,
        line_offset + -1,
        line_offset,
        line_offset + 1,
    ];
    let pattern = Regex::new(r"\d+").unwrap();

    for mat in pattern.find_iter(&finput) {
        let start_offset = mat.start();
        let end_offset = mat.end();
        let value = &finput[start_offset..end_offset];

        let n = match value.len() {
            3 => neighbors_3.clone(),
            2 => neighbors_2.clone(),
            _ => neighbors_1.clone(),
        };

        let mut is_valid = false;
        for neighbor in n.clone() {
            let index = (start_offset as i32 + neighbor) as usize;

            // Using some here to handle out of bounds
            // when close to edges
            if let Some(value) = finput_array.get(index) {
                if !value.is_numeric() && value != &'.' {
                    is_valid = true;
                    continue;
                }
            }
        }

        if is_valid {
            sum += value.parse::<i32>().unwrap();
        }
    }

    println!("result: {sum}");
}

fn part_two(input: Vec<String>) {
    // We will treat the input as a single line
    // just to simplify index look up
    let line_offset = input.first().unwrap().len() as i32;
    let finput = input.join("");
    let finput_array = finput.chars().collect::<Vec<char>>();

    // In part two, we will be using match_indices to find
    // all gears in the schematic, then store how many part numbers they have
    let mut gears: HashMap<usize, (i32, i32)> = finput
        .match_indices('*')
        .map(|(e, _)| (e, (0, 1)))
        .collect();

    // Avoid checking the same neighbors twice
    let neighbors_3: Vec<i32> = vec![
        -line_offset + -1,
        -line_offset,
        -line_offset + 1,
        -line_offset + 2,
        -line_offset + 3,
        -1,
        3,
        line_offset + -1,
        line_offset,
        line_offset + 1,
        line_offset + 2,
        line_offset + 3,
    ];
    let neighbors_2: Vec<i32> = vec![
        -line_offset + -1,
        -line_offset,
        -line_offset + 1,
        -line_offset + 2,
        -1,
        2,
        line_offset + -1,
        line_offset,
        line_offset + 1,
        line_offset + 2,
    ];
    let neighbors_1: Vec<i32> = vec![
        -line_offset + -1,
        -line_offset,
        -line_offset + 1,
        -1,
        1,
        line_offset + -1,
        line_offset,
        line_offset + 1,
    ];

    let pattern = Regex::new(r"\d+").unwrap();
    for mat in pattern.find_iter(&finput) {
        let start_offset = mat.start();
        let end_offset = mat.end();
        let value_str = &finput[start_offset..end_offset];

        let n = match value_str.len() {
            3 => neighbors_3.clone(),
            2 => neighbors_2.clone(),
            _ => neighbors_1.clone(),
        };

        for neighbor in n.clone() {
            let index = (start_offset as i32 + neighbor) as usize;

            // Using some here to handle out of bounds
            // when close to edges
            if let Some(value) = finput_array.get(index) {
                if !value.is_numeric() && value != &'.' && value == &'*' {
                    gears.entry(index).and_modify(|(n_of_part_number, holder)| {
                        *n_of_part_number += 1;
                        *holder *= value_str.parse::<i32>().unwrap();
                    });
                    continue;
                }
            }
        }
    }

    let sum = gears
        .iter()
        .fold(0, |acc, (_, &(n_of_part_number, holder))| {
            if n_of_part_number == 2 {
                acc + holder
            } else {
                acc
            }
        });

    println!("result: {sum}");
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
