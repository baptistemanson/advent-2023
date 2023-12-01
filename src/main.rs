#![feature(iter_array_chunks)]
#![feature(int_roundings)]

use std::time::Instant;
mod day1;
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
        "playground" => playground::main(),
        _ => panic!("unknown problem"),
    };
    println!(
        "took {} ms",
        now.elapsed().as_millis().separate_with_commas()
    );
}