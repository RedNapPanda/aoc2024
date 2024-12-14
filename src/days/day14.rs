use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn walk(&mut self, height: i64, width: i64) {
        self.position += &self.velocity;
        if self.position.x < 0 {
            self.position.x += height;
        } else if self.position.x >= height {
            self.position.x %= height;
        }
        if self.position.y < 0 {
            self.position.y += width;
        } else if self.position.y >= width {
            self.position.y %= width;
        }
    }
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
    let (height, width) = (103i64, 101i64);
    let mut robots = parse(lines);
    for _ in 0..100 {
        for robot in &mut robots {
            robot.walk(height, width);
        }
    }
    let mid_height = height / 2;
    let mid_width = width / 2;
    let mut quadrants = FxHashMap::default();
    for robot in &robots {
        let pos = &robot.position;
        if pos.x == mid_height || pos.y == mid_width {
            continue;
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
    quadrants.values().product::<i64>()
}

pub fn solve2(lines: &[String]) -> i64 {
    let (height, width) = (103i64, 101i64);
    let mut grid = Grid::<usize>::with_dimensions(height as usize, width as usize);
    let mut robots = parse(lines);
    let mut res = 0;
    for x in 0..10_000 {
        grid.reset_defaults();
        for robot in &mut robots {
            robot.walk(height, width);
            grid.set(&robot.position, grid.get(&robot.position).unwrap_or(&0) + 1);
        }
        // printing all grids with no overlap.
        // hintword was 'most' robots (though they could have technically overlapped anyways...)
        // this was clearly a trick question... to find a cycle
        if !grid.iter().flatten().any(|&v| v > 1) {
            println!("{}\n{}", x + 1, grid);
            res = x + 1
        }
    }
    res
}

fn parse(lines: &[String]) -> Vec<Robot> {
    let regex = Regex::new("p=(-?\\d+),(-?\\d+) v=(-?\\d+),(-?\\d+)").unwrap();
    lines
        .iter()
        .flat_map(move |l| {
            regex
                .captures_iter(l)
                .map(|captures| {
                    let py = captures[1].parse::<i64>().unwrap();
                    let px = captures[2].parse::<i64>().unwrap();
                    let vy = captures[3].parse::<i64>().unwrap();
                    let vx = captures[4].parse::<i64>().unwrap();
                    let position = Point::from((px, py));
                    let velocity = Point::from((vx, vy));
                    Robot { position, velocity }
                })
                .collect_vec()
        })
        .collect_vec()
}
