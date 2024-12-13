use std::cmp;
use std::collections::HashSet;
use crate::utils::point::Point;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
struct ClawMachine {
    a_button: Point,
    b_button: Point,
    prize: Point,
}

pub fn solve1(lines: &[String]) -> i64 {
    let claw_machines = claw_machines(lines);
    cramer(&claw_machines, 0)
/*
~120x slower than cramer's rule
    claw_machines
        .iter()
        .flat_map(|machine| {
            let seen = &mut HashSet::new();
            _dfs(machine, 0, 0, seen)
        })
        .sum()
 */
}

pub fn solve2(lines: &[String]) -> i64 {
    let claw_machines = claw_machines(lines);
    cramer(&claw_machines, 10_000_000_000_000)
}

fn claw_machines(lines: &[String]) -> Vec<ClawMachine> {
    let regex: Regex = Regex::new(".+: X.(\\d+), Y.(\\d+)").unwrap();
    lines
        .iter()
        .chunk_by(|l| !l.is_empty())
        .into_iter()
        .filter(|(b, _)| *b)
        .map(|(_, chunk)| {
            chunk
                .map(|l| {
                    regex
                        .captures(l)
                        .map(|c| {
                            Point::from((
                                c[1].parse::<usize>().unwrap(),
                                c[2].parse::<usize>().unwrap(),
                            ))
                        })
                        .unwrap()
                })
                .collect_tuple::<(_, _, _)>()
                .map(|points| ClawMachine {
                    a_button: points.0,
                    b_button: points.1,
                    prize: points.2,
                })
                .unwrap()
        })
        .collect_vec()
}

/*
Cramer's
|ax bx||a_count| = |x|
|ay by||b_count| = |y|
*/
fn cramer(claw_machines: &[ClawMachine], shift: i64) -> i64 {
    claw_machines
        .iter()
        .map(|machine| {
            let x = machine.prize.x + shift;
            let y = machine.prize.y + shift;
            let a = &machine.a_button;
            let b = &machine.b_button;
            let div = a.x * b.y - a.y * b.x;
            let a_count = (x * b.y - y * b.x) / div;
            let b_count = (y * a.x - x * a.y) / div;
            let px = a.x * a_count + b.x * b_count;
            let py = a.y * a_count + b.y * b_count;
            if x != px || y != py {
                return 0
            }
            3 * a_count + b_count
        })
        .sum()
}

fn _dfs(machine: &ClawMachine, a_count: i64, b_count: i64, seen: &mut HashSet<(i64, i64)>) -> Option<i64> {
    if !seen.insert((a_count, b_count)) {
        return None
    }
    let px = machine.a_button.x * a_count + machine.b_button.x * b_count;
    let py = machine.a_button.y * a_count + machine.b_button.y * b_count;
    if px == machine.prize.x && py == machine.prize.y {
        return Some(a_count * 3 + b_count)
    }
    if a_count > 100 || b_count > 100 || px > machine.prize.x || py > machine.prize.y {
        return None
    }
    let a_tokens = _dfs(machine, a_count+1, b_count, seen);
    let b_tokens = _dfs(machine, a_count, b_count+1, seen);
    a_tokens.or(b_tokens).min(b_tokens.or(a_tokens))
}