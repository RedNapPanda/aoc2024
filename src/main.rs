mod days;
mod utils;

use crate::utils::input;
use color_eyre::eyre::Result;
use days::{day1, day2, day3, day4, day5, day6, day7, day8, day9};
use std::time::{Duration, Instant};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc 2024")]
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
    println!("P1: {:?} | {}", result1, time_str(&elapsed1));

    let func2 = get_day_fn(opt.day, false);
    let time2 = Instant::now();
    let result2 = func2(&lines);
    let elapsed2 = time2.elapsed();
    println!("P2: {:?} | {}", result2, time_str(&elapsed2));

    Ok(())
}

fn get_day_fn(day: u8, part1: bool) -> impl Fn(&Vec<String>) -> i64 {
    match day {
        1 if part1 => day1::solve1,
        1 => day1::solve2,
        2 if part1 => day2::solve1,
        2 => day2::solve2,
        3 if part1 => day3::solve1,
        3 => day3::solve2,
        4 if part1 => day4::solve1,
        4 => day4::solve2,
        5 if part1 => day5::solve1,
        5 => day5::solve2,
        6 if part1 => day6::solve1,
        6 => day6::solve2,
        7 if part1 => day7::solve1,
        7 => day7::solve2,
        8 if part1 => day8::solve1,
        8 => day8::solve2,
        9 if part1 => day9::solve1,
        9 => day9::solve2,
        _ => panic!("Invalid day"),
    }
}

fn time_str(elasped: &Duration) -> String {
    let mut time_elapsed = elasped.as_micros();
    let mut decimals = 0;
    let mut suffix = "µs";
    if time_elapsed > 1000 {
        decimals = time_elapsed % 1000;
        time_elapsed = elasped.as_millis();
        suffix = "ms"
    }
    if time_elapsed > 1000 {
        decimals = time_elapsed % 1000;
        time_elapsed = elasped.as_secs() as u128;
        suffix = "s"
    }
    if decimals > 0 {
        format!("{:?}.{}{}", time_elapsed, decimals, suffix)
    } else {
        format!("{:?}{}", time_elapsed, suffix)
    }
}
