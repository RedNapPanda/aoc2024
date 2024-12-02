mod days;
mod utils;

use crate::utils::input;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc 2024")]
struct Opt {
    #[structopt(short, long)]
    day: u8,
}

fn main() {
    let opt = Opt::from_args();
    let lines = input::read_input(opt.day);
    let func1 = get_day_fn(opt.day, true);
    let time1 = Instant::now();
    let result1 = func1(&lines).unwrap();
    let elapsed1 = time1.elapsed().as_micros();
    let func2 = get_day_fn(opt.day, false);
    let time2 = Instant::now();
    let result2 = func2(&lines).unwrap();
    let elapsed2 = time2.elapsed().as_micros();
    println!("P1: {} | {}µs", result1, elapsed1);
    println!("P2: {} | {}µs", result2, elapsed2);
}

fn get_day_fn(day: u8, part1: bool) -> impl Fn(&Vec<String>) -> Option<i64> {
    match day {
        1 if part1 => days::day1::solve1,
        1 => days::day1::solve2,
        _ => panic!("Invalid day"),
    }
}