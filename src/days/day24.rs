use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use strum_macros::Display;

type Gate = (String, Op, String);

pub fn solve1(lines: &[String]) -> i64 {
    let (inputs, gates) = parse(lines);
    run_adder(&inputs, &gates)
}

pub fn solve2(lines: &[String]) -> i64 {
    // X XOR Y -> A
    // X AND Y -> B
    // Carry XOR A -> Z
    // Carry AND A -> C
    // B OR C -> Carry
    // x ^ y -> AND + XOR
    // x & y -> OR
    let (inputs, gates) = parse(lines);
    let broken_gates = gates.clone()
        .into_iter()
        .filter(|(c, (a, op, b))| {
            if a.ends_with("00") || b.ends_with("00") {
                return false;
            }
            // Z gates must follow an XOR op except the last bit as it's the carry bit
            let kz = c.starts_with("z");
            if op == &Op::XOR {
                let axy = a.starts_with("x") || a.starts_with("y");
                let bxy = b.starts_with("x") || b.starts_with("y");
                if !axy && !bxy && !kz {
                    return true;
                } else if !kz {
                    //should be used in an AND
                    return !gates.iter().any(|(_, gate)| gate.1 == Op::AND && (gate.0 == *c || gate.2 == *c));
                }
                return !kz && !axy && !bxy;
            } else if op == &Op::AND {
                return !gates.iter().any(|(_, gate)| gate.1 == Op::OR && (gate.0 == *c || gate.2 == *c));
            }
            kz && *c != "z45"
        })
        .map(|(k, _)| k);
    println!("Solution: {}", broken_gates.sorted_unstable().join(","));
    0
}

fn parse(lines: &[String]) -> (FxHashMap<String, usize>, FxHashMap<String, Gate>) {
    let inputs = lines.iter().take_while(|l| !l.is_empty())
        .map(|l| {
            let (wire, val) = l.split_once(": ").unwrap();
            (wire.to_string(), val.parse::<usize>().unwrap())
        })
        .collect::<FxHashMap<_, _>>();
    let wires = lines.iter().skip(inputs.len() + 1)
        .map(|l| {
            // a OR b -> c
            let split = l.split(" ").collect_vec();
            (split[4].to_string(), (split[0].to_string(), Op::from(split[1]), split[2].to_string()))
        }).collect::<FxHashMap<_, _>>();
    (inputs, wires)
}

fn run_adder<'a>(inputs: &'a FxHashMap<String, usize>, gates: &'a FxHashMap<String, Gate>) -> i64 {
    let mut resolved = inputs.clone();
    let mut queue = gates.clone().into_iter().collect::<VecDeque<_>>();
    while let Some((c, (a, op, b))) = queue.pop_front() {
        if a == b || b == c || a == c {
            println!("{} {} {} | {}", a, op, b, c);
            break;
        }
        if !resolved.contains_key(&a) || !resolved.contains_key(&b) {
            queue.push_back((c, (a, op, b)));
            continue;
        }
        let a = resolved.get(&a).unwrap();
        let b = resolved.get(&b).unwrap();
        match op {
            Op::AND => { resolved.insert(c, a & b); }
            Op::OR => { resolved.insert(c, a | b); }
            Op::XOR => { resolved.insert(c, a ^ b); }
        }
    }
    resolved
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted_unstable_by_key(|(k, _)| *k)
        .enumerate()
        .map(|(i, (_, &v))| v << i)
        .sum::<usize>() as i64
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Display)]
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