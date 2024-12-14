mod days;
mod utils;

use crate::utils::input;
use color_eyre::eyre::Result;
use days::*;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc")]
struct Opt {
    #[structopt(short, long)]
    day: u8,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    let lines = input::read_input(opt.day);

    let func1 = get_day_fn(opt.day, true);
    let time1 = Instant::now();
    let result1 = func1(&lines);
    let elapsed1 = time1.elapsed();
    println!("P1: {:?} in {:?}", result1, elapsed1);

    let func2 = get_day_fn(opt.day, false);
    let time2 = Instant::now();
    let result2 = func2(&lines);
    let elapsed2 = time2.elapsed();
    println!("P2: {:?} in {:?}", result2, elapsed2);

    Ok(())
}

fn get_day_fn(day: u8, part1: bool) -> impl Fn(&[String]) -> i64 {
    match day {
        1 if part1 => day01::solve1,
        1 => day01::solve2,
        2 if part1 => day02::solve1,
        2 => day02::solve2,
        3 if part1 => day03::solve1,
        3 => day03::solve2,
        4 if part1 => day04::solve1,
        4 => day04::solve2,
        5 if part1 => day05::solve1,
        5 => day05::solve2,
        6 if part1 => day06::solve1,
        6 => day06::solve2,
        7 if part1 => day07::solve1,
        7 => day07::solve2,
        8 if part1 => day08::solve1,
        8 => day08::solve2,
        9 if part1 => day09::solve1,
        9 => day09::solve2,
        10 if part1 => day10::solve1,
        10 => day10::solve2,
        11 if part1 => day11::solve1,
        11 => day11::solve2,
        12 if part1 => day12::solve1,
        12 => day12::solve2,
        13 if part1 => day13::solve1,
        13 => day13::solve2,
        14 if part1 => day14::solve1,
        14 => day14::solve2,
        _ => panic!("Invalid day"),
    }
}
