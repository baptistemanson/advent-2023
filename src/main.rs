#![feature(iter_array_chunks)]
#![feature(int_roundings)]

use std::time::Instant;
mod data2;
mod data3;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod debug;
mod playground;
use thousands::Separable;
fn main() {
    let pb = std::env::args().nth(1).expect("expected problem number");
    execute(pb);
}

fn execute(pb: String) {
    let now = Instant::now();
    match pb.as_str() {
        "11" => day1::pb1(),
        "12" => day1::pb2(),
        "21" => day2::pb1(),
        "22" => day2::pb2(),
        "31" => day3::pb1(),
        "32" => day3::pb2(),
        "41" => day4::pb1(),
        "42" => day4::pb2(),
        "51" => day5::pb1(),
        "52" => day5::pb2(),
        "61" => day6::pb1(),
        "62" => day6::pb2(),
        "playground" => playground::main(),
        _ => panic!("unknown problem"),
    };
    println!(
        "took {} us",
        now.elapsed().as_micros().separate_with_commas()
    );
}
