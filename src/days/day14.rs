use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Debug)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn walk(&mut self, height: i64, width: i64) {
        self.position += &self.velocity;
        self.position.x = self.normalize(self.position.x, height);
        self.position.y = self.normalize(self.position.y, width);
    }
    
    fn normalize(&self, x: i64, bound: i64) -> i64 {
        if x < 0 {
            x + bound
        } else if x >= bound {
            x % bound
        } else {
            x
        }
    }
}

pub fn solve1(lines: &[String]) -> i64 {
    let (height, width) = (103i64, 101i64);
    let mut robots = parse(lines);
    for robot in &mut robots {
        let x100 = robot.position.x + 100 * robot.velocity.x;
        let y100 = robot.position.y + 100 * robot.velocity.y;
        robot.position.x = ((x100 % height) + height) % height; 
        robot.position.y = ((y100 % width) + width) % width;
    }
    let mid_height = height / 2;
    let mid_width = width / 2;
    let mut quadrants = [0; 4];
    for robot in &robots {
        let pos = &robot.position;
        if pos.x == mid_height || pos.y == mid_width {
            continue;
        }
        let quadrant = (pos.x > mid_height) as usize * 2 + (pos.y > mid_width) as usize;
        quadrants[quadrant] += 1;
    }
    quadrants.iter().product::<i64>()
}

pub fn solve2(lines: &[String]) -> i64 {
    let (height, width) = (103i64, 101i64);
    let mut grid = Grid::<usize>::with_dimensions(height as usize, width as usize);
    let mut robots = parse(lines);
    for x in 0..(103*101) {
        grid.reset_defaults();
        for robot in &mut robots {
            robot.walk(height, width);
            grid.set(&robot.position, grid.get(&robot.position).unwrap_or(&0) + 1);
        }
        // printing all grids with no overlap.
        // hint-word was 'most' robots (though they could have technically overlapped anyways...)
        // this was clearly a trick question... to find a cycle
        // I feel like this wasn't supposed to work...
        if !grid.iter().flatten().any(|&v| v > 1) {
            return x + 1;
        }
        // The glorious tree, flood fill or continuous blocks would have worked for the tree or outline
        // 000000000000000000000000000000000
        // 011111111111111111111111111111110
        // 010000000000000000000000000000010
        // 010000000000000000000000000000010
        // 010000000000000000000000000000010
        // 010000000000000000000000000000010
        // 010000000000000010000000000000010
        // 010000000000000111000000000000010
        // 010000000000001111100000000000010
        // 010000000000011111110000000000010
        // 010000000000111111111000000000010
        // 010000000000001111100000000000010
        // 010000000000011111110000000000010
        // 010000000000111111111000000000010
        // 010000000001111111111100000000010
        // 010000000011111111111110000000010
        // 010000000000111111111000000000010
        // 010000000001111111111100000000010
        // 010000000011111111111110000000010
        // 010000000111111111111111000000010
        // 010000001111111111111111100000010
        // 010000000011111111111110000000010
        // 010000000111111111111111000000010
        // 010000001111111111111111100000010
        // 010000011111111111111111110000010
        // 010000111111111111111111111000010
        // 010000000000000111000000000000010
        // 010000000000000111000000000000010
        // 010000000000000111000000000000010
        // 010000000000000000000000000000010
        // 010000000000000000000000000000010
        // 010000000000000000000000000000010
        // 010000000000000000000000000000010
        // 011111111111111111111111111111110
        // 000000000000000000000000000000000
    }
    unreachable!()
}

fn parse(lines: &[String]) -> Vec<Robot> {
    let regex = Regex::new("p=(-?\\d+),(-?\\d+) v=(-?\\d+),(-?\\d+)").unwrap();
    lines
        .iter()
        .flat_map(move |l| {
            regex
                .captures_iter(l)
                .map(|captures| {
                    // directions are transposed to natural layout where x=row, y=col
                    // x -> right | y -> down == array[y][x]
                    // vs
                    // x -> down | y -> right == array[x][y]
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
