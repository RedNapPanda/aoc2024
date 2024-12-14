use std::ops::{Deref, DerefMut};
use crate::utils::point::Point;
use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;

#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Deref for Robot {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.position
    }
}

impl DerefMut for Robot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.position
    }
}

pub fn solve1(lines: &[String]) -> i64 {
    let (height, width) = (103, 101);
    let mut robots = parse(lines);
    for _ in 0..100 {
        for robot in &mut robots {
            robot.position += &robot.velocity;
            if robot.position.x < 0 {
                robot.position.x += height;
            } else if robot.position.x >= height {
                robot.position.x %= height;
            }
            if robot.position.y < 0 {
                robot.position.y += width;
            } else if robot.position.y >= width {
                robot.position.y %= width;
            }
        }
    }
    let mid_height = height / 2;
    let mid_width = width / 2;
    let mut quadrants = FxHashMap::default();
    for robot in &robots {
        println!("{:?}", robot);
        let pos = &robot.position;
        if pos.x == mid_height || pos.y == mid_width {
            continue
        }
        let quadrant = if pos.x < mid_height && pos.y < mid_width {
            1
        } else if pos.x < mid_height && pos.y > mid_width {
            2
        } else if pos.x > mid_height && pos.y < mid_width {
            3
        } else {
            4
        };
        quadrants.insert(quadrant, quadrants.get(&quadrant).unwrap_or(&0) + 1);
    }
    quadrants
        .values()
        .product::<i64>()
}

pub fn solve2(lines: &[String]) -> i64 {
    0
}

fn parse(lines: &[String]) -> Vec<Robot> {
    let regex = Regex::new("p=(-?\\d+),(-?\\d+) v=(-?\\d+),(-?\\d+)").unwrap();
    lines.iter()
        .flat_map(move |l| 
            regex.captures_iter(l)
                .map(|captures| {
                    let py = captures[1].parse::<i64>().unwrap();
                    let px = captures[2].parse::<i64>().unwrap();                    
                    let vy = captures[3].parse::<i64>().unwrap();
                    let vx = captures[4].parse::<i64>().unwrap();
                    let position = Point::from((px, py));
                    let velocity = Point::from((vx, vy));
                    Robot {
                        position, velocity
                    }
                }).collect_vec()
        ).collect_vec()
}