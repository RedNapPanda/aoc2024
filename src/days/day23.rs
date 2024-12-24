use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeSet;

pub fn solve1(lines: &[String]) -> i64 {
    let connections = build_connections(lines);
    let mut seen = FxHashSet::default();
    let mut count = 0;
    for (comp1, comp_connections) in &connections {
        for comp2 in comp_connections {
            for comp3 in comp_connections.intersection(connections.get(comp2).unwrap()) {
                if comp1.starts_with("t") || comp2.starts_with("t") || comp3.starts_with("t") {
                    let mut vec = vec![comp1.clone(), comp2.clone(), comp3.clone()];
                    vec.sort();
                    if !seen.insert(vec) {
                        continue;
                    }
                    count += 1;
                }
            }
        }
    }
    count
}

pub fn solve2(lines: &[String]) -> i64 {
    let connections = build_connections(lines);
    let mut seen = BTreeSet::default();
    let mut largest = BTreeSet::new();
    for c in connections.keys() {
        if seen.contains(c) {
            continue;
        }
        // forgot to add the key to the set for the search lol
        let mut set = connections.get(c).unwrap().clone();
        set.insert(c.clone());
        let result = bron_kerbosch(&connections, set);
        if result.len() > largest.len() {
            largest = result.clone();
        }
        seen.extend(result);
    }
    println!("Solution: {}", largest.iter().sorted().join(","));
    0
}

fn build_connections(lines: &[String]) -> FxHashMap<String, BTreeSet<String>> {
    let mut connections = FxHashMap::<String, BTreeSet<String>>::default();
    lines.iter()
        .flat_map(|line|
            line.split_once('-')
                .map(|(s1, s2)| (s1.to_string(), s2.to_string())))
        .for_each(|(a, b)| {
            connections.entry(a.clone()).or_insert_with(BTreeSet::default).insert(b.clone());
            connections.entry(b).or_insert_with(BTreeSet::default).insert(a);
        });
    connections
}

fn bron_kerbosch(connections: &FxHashMap<String, BTreeSet<String>>, remaining: BTreeSet<String>) -> BTreeSet<String> {
    let mut stack = vec![(BTreeSet::default(), remaining, BTreeSet::default())];
    while let Some((mut result, mut remaining, mut seen)) = stack.pop() {
        if remaining.is_empty() && seen.is_empty() {
            return result;
        }
        if let Some(c) = remaining.pop_first() {
            let n = connections.get(&c).unwrap();
            let p_intersection_n = remaining.intersection(n).cloned().collect::<BTreeSet<_>>();
            let x_intersection_n = seen.intersection(n).cloned().collect::<BTreeSet<_>>();
            seen.insert(c.clone());
            result.insert(c.clone());
            stack.push((result.clone(), remaining, seen));
            stack.push((result, p_intersection_n, x_intersection_n));
        }
    }
    BTreeSet::default()
}