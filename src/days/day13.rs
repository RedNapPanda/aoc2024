use crate::utils::point::Point;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

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
            let px = machine.prize.x + shift;
            let py = machine.prize.y + shift;
            let a = &machine.a_button;
            let b = &machine.b_button;
            let det = a.x * b.y - a.y * b.x; // determinant of matrix
            let a_count = (px * b.y - py * b.x) / det;
            let b_count = (py * a.x - px * a.y) / det;
            let x = a.x * a_count + b.x * b_count;
            let y = a.y * a_count + b.y * b_count;
            if px != x || py != y {
                return 0;
            }
            3 * a_count + b_count
        })
        .sum()
}

fn _dfs(
    machine: &ClawMachine,
    a_count: i64,
    b_count: i64,
    seen: &mut HashSet<(i64, i64)>,
) -> Option<i64> {
    if !seen.insert((a_count, b_count)) {
        return None;
    }
    let px = machine.a_button.x * a_count + machine.b_button.x * b_count;
    let py = machine.a_button.y * a_count + machine.b_button.y * b_count;
    if px == machine.prize.x && py == machine.prize.y {
        return Some(a_count * 3 + b_count);
    }
    if a_count > 100 || b_count > 100 || px > machine.prize.x || py > machine.prize.y {
        return None;
    }
    let a_tokens = _dfs(machine, a_count + 1, b_count, seen);
    let b_tokens = _dfs(machine, a_count, b_count + 1, seen);
    a_tokens.or(b_tokens).min(b_tokens.or(a_tokens))
}
