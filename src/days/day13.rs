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
    claw_machines
        .iter()
        .map(|machine| {
            let a = &machine.a_button;
            let b = &machine.b_button;
            let p = &machine.prize;
            let num = p.x * b.y - p.y * b.x;
            let div = a.x * b.y - a.y * b.x;
            if num % div == 0 { // (prize x b) / (a x b) == (a * ai) + (b * bi)
                let a_count = num / div;
                let b_count = (p.x - a.x * a_count) / b.x;
                return a_count * 3 + b_count
            }
            0
        }).sum()
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
|ax bx||x|=|px|
|ay by||y|=|py|
*/
fn cramer(claw_machines: &[ClawMachine], shift: i64) -> i64 {
    claw_machines
        .iter()
        .map(|machine| {
            let px = machine.prize.x + shift;
            let py = machine.prize.y + shift;
            let a = &machine.a_button;
            let b = &machine.b_button;
            let div = a.x * b.y - a.y * b.x;
            let x = (px * b.y - py * b.x) / div;
            let y = (py * a.x - px * a.y) / div;
            if x * a.x + y * b.x == px && x * a.y + y * b.y == py {
                return x * 3 + y;
            }
            0
        })
        .sum()
}
