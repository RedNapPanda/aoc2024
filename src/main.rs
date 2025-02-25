#![allow(clippy::zero_prefixed_literal)]

mod days;
mod utils;

use crate::utils::input;
use color_eyre::eyre::Result;
use days::*;
use paste::paste;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc")]
struct Opt {
    #[structopt(short, long)]
    day: u16,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    let range = if opt.day == 0 {
        1..=25
    } else {
        opt.day..=opt.day
    };
    let start = Instant::now();
    for day in range {
        let lines = input::read_input(day);
        let func1 = day_fn(day, true);
        let time1 = Instant::now();
        let result1 = func1(&lines);
        let elapsed1 = time1.elapsed();
        println!("Day {} | P1: {:?} in {:?}", day, result1, elapsed1);

        let func2 = day_fn(day, false);
        let time2 = Instant::now();
        let result2 = func2(&lines);
        let elapsed2 = time2.elapsed();
        println!("Day {} | P2: {:?} in {:?}", day, result2, elapsed2);
    }
    let elapsed = start.elapsed();
    println!("Completed in {:?}", elapsed);
    Ok(())
}


day_fn!(01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25);
