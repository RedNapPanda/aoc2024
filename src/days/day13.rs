use crate::utils::node::Node;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

pub fn solve1(lines: &[String]) -> i64 {
    let claw_machines = claw_machines(lines);
    play_all(&claw_machines, 0)
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
    play_all(&claw_machines, 10_000_000_000_000)
}

#[derive(Debug, Clone)]
struct ClawMachine {
    a_button: Node<i64>,
    b_button: Node<i64>,
    prize: Node<i64>,
}

impl ClawMachine {
    /*
    Cramer's
    |ax bx||a_count| = |x|
    |ay by||b_count| = |y|
    */
    fn cramers_rule(&self, shift: i64) -> i64 {
        let px = self.prize.y + shift;
        let py = self.prize.x + shift;
        let a = &self.a_button;
        let b = &self.b_button;
        let det = a.y * b.x - a.x * b.y; // determinant of matrix
        let a_count = (px * b.x - py * b.y) / det;
        let b_count = (py * a.y - px * a.x) / det;
        let x = a.y * a_count + b.y * b_count;
        let y = a.x * a_count + b.x * b_count;
        if px != x || py != y {
            return 0;
        }
        3 * a_count + b_count
    }

    fn _dfs(&self, a_count: i64, b_count: i64, seen: &mut HashSet<(i64, i64)>) -> Option<i64> {
        if !seen.insert((a_count, b_count)) {
            return None;
        }
        let px = self.a_button.y * a_count + self.b_button.y * b_count;
        let py = self.a_button.x * a_count + self.b_button.x * b_count;
        if px == self.prize.y && py == self.prize.x {
            return Some(a_count * 3 + b_count);
        }
        if a_count > 100 || b_count > 100 || px > self.prize.y || py > self.prize.x {
            return None;
        }
        let a_tokens = self._dfs(a_count + 1, b_count, seen);
        let b_tokens = self._dfs(a_count, b_count + 1, seen);
        a_tokens.or(b_tokens).min(b_tokens.or(a_tokens))
    }
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
                            Node::from((c[1].parse::<i64>().unwrap(), c[2].parse::<i64>().unwrap()))
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

fn play_all(claw_machines: &[ClawMachine], shift: i64) -> i64 {
    claw_machines
        .iter()
        .map(|machine| machine.cramers_rule(shift))
        .sum()
}
