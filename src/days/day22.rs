use std::collections::hash_map::Entry;
use rustc_hash::{FxHashMap, FxHashSet};

const ITERATIONS: usize = 2000;
const MODULO: i64 = 0xffffff;

pub fn solve1(lines: &[String]) -> i64 {
    lines.iter()
        .map(|line| {
            (0..ITERATIONS)
                .fold(line.parse::<i64>().unwrap(), |secret, _| next_secret(secret))
        })
        .sum::<i64>()
}

pub fn solve2(lines: &[String]) -> i64 {
    let mut sum = 0;
    let mut sequences = FxHashMap::default();
    for line in lines {
        let mut secret = line.parse::<i64>().unwrap();
        let mut prices = vec![(secret % 10, 0); ITERATIONS + 1];
        let mut seen = FxHashSet::default();
        for i in 0..ITERATIONS {
            secret = next_secret(secret);
            let price = secret % 10;
            prices[i + 1] = (price, price - prices[i].0);
            if (3..ITERATIONS).contains(&i) {
                let seq = (prices[i - 3].1, prices[i - 2].1, prices[i - 1].1, prices[i].1);
                if seen.contains(&seq) {
                    continue;
                }
                seen.insert(seq);
                let price = prices[i].0;
                match sequences.entry(seq) {
                    Entry::Occupied(mut entry) => {
                        *entry.get_mut() += price;
                        sum = sum.max(*entry.get_mut())
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(price);
                        sum = sum.max(price)
                    }
                }
            }
        }
    }
    sum
}

fn next_secret(mut secret: i64) -> i64 {
    secret ^= secret << 6;
    secret &= MODULO;
    secret ^= secret >> 5;
    secret &= MODULO;
    secret ^= secret << 11;
    secret &= MODULO;
    secret
}