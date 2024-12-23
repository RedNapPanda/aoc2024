use std::collections::BTreeSet;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve1(lines: &[String]) -> i64 {
    let mut connections = FxHashMap::<String, FxHashSet<String>>::default();
    lines.iter()
        .flat_map(|line|
            line.split_once('-')
                .map(|(s1, s2)| (s1.to_string(), s2.to_string())))
        .for_each(|(a, b)| {
            connections.entry(a.clone()).or_insert_with(FxHashSet::default).insert(b.clone());
            connections.entry(b).or_insert_with(FxHashSet::default).insert(a);
        });
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
    let mut connections = FxHashMap::<String, BTreeSet<String>>::default();
    lines.iter()
        .flat_map(|line|
            line.split_once('-')
                .map(|(s1, s2)| (s1.to_string(), s2.to_string())))
        .for_each(|(a, b)| {
            connections.entry(a.clone()).or_insert_with(BTreeSet::default).insert(b.clone());
            connections.entry(b).or_insert_with(BTreeSet::default).insert(a);
        });
    let mut seen = BTreeSet::default();
    let mut largest = vec![];
    for c in connections.keys() {
        if seen.contains(c) {
            continue;
        }
        // forgot to add the key to the set for the search lol
        let mut set = connections.get(c).unwrap().clone();
        set.insert(c.clone());
        let result = bron_kerbosch(&connections, set);
        if result.len() > largest.len() {
            largest = result.iter().cloned().sorted().collect_vec();
        }
        seen.extend(result);
    }

    println!("Solution: {}", largest.iter().sorted().join(","));
    0
}

fn bron_kerbosch(connections: &FxHashMap<String, BTreeSet<String>>, p: BTreeSet<String>) -> BTreeSet<String> {
    let mut stack = vec![(BTreeSet::default(), p, BTreeSet::default())];
    while let Some((r, p, x)) = stack.pop() {
        if p.is_empty() && x.is_empty() {
            return r;
        }
        for c in &p {
            let mut p_minus_c = p.clone();
            p_minus_c.remove(c);
            let mut x_union_c = x.clone();
            x_union_c.insert(c.clone());
            let mut r_union_c = r.clone();
            r_union_c.insert(c.to_owned());
            let n = connections.get(c).unwrap();
            let p_intersection_n = p.intersection(n).cloned().collect::<BTreeSet<_>>();
            let x_intersection_n = x.intersection(n).cloned().collect::<BTreeSet<_>>();
            stack.push((r.clone(), p_minus_c, x_union_c));
            stack.push((r_union_c, p_intersection_n, x_intersection_n));
        }
    }
    BTreeSet::default()
}