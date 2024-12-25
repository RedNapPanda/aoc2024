use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn read_input(day: u8) -> Vec<String> {
    let path = Path::new("day_data").join(format!("day{:02}", day));
    read_lines(&path)
}

fn read_lines(path: &PathBuf) -> Vec<String> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map_while(Result::ok)
        .collect_vec()
}
