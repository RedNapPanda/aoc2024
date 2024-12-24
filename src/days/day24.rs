use std::collections::VecDeque;
use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn solve1(lines: &[String]) -> i64 {
    let mut wires = lines.iter().take_while(|l| !l.is_empty())
        .map(|l| {
            let (wire, val) = l.split_once(": ").unwrap();
            (wire, val.parse::<usize>().unwrap())
        })
        .collect::<FxHashMap<_, _>>();
    let mut queue = lines.iter().skip(wires.len() + 1)
        .map(|l| {
            // a OR b -> c
            let split = l.split(" ").collect_vec();
            (split[0], Op::from(split[1]), split[2], split[4])
        }).collect::<VecDeque<_>>();
    while let Some((a, op, b, c)) = queue.pop_front() {
        if !wires.contains_key(a) || !wires.contains_key(b) {
            queue.push_back((a, op, b, c));
            continue
        }
        match op {
            Op::AND => { wires.insert(c, wires[a] & wires[b]); }
            Op::OR => { wires.insert(c, wires[a] | wires[b]); }
            Op::XOR => { wires.insert(c, wires[a] ^ wires[b]); }
        }
    }
    wires.iter()
        .filter(|(&k, _)| k.starts_with("z"))
        .sorted_unstable_by_key(|(&k, _)| k)
        .enumerate()
        .map(|(i, (_, &v))| v << i)
        .sum::<usize>() as i64
}

pub fn solve2(lines: &[String]) -> i64 {
    0
}

#[allow(clippy::upper_case_acronyms)]
enum Op {
    AND,
    OR,
    XOR,
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "AND" => Op::AND,
            "OR" => Op::OR,
            "XOR" => Op::XOR,
            _ => panic!("invalid op")
        }
    }
}