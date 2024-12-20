use crate::utils::algo::astar::astar;
use crate::utils::grid::Grid;
use crate::utils::node::Node;
use crate::utils::PathResult;
use std::fmt::{Display, Formatter, Write};

pub fn solve1(lines: &[String]) -> i64 {
    solve(lines, 2)
}

pub fn solve2(lines: &[String]) -> i64 {
    solve(lines, 20)
}

fn solve(lines: &[String], max_steps: i64) -> i64 {
    let grid = &mut Grid::<Tile>::from(lines);
    let start = start(grid);
    let mut count = 0;
    if let Some(result) = astar(&start,
                                |node| grid.neighbors_cardinal(node)
                                    .into_iter()
                                    .filter(|n| grid.get(n).is_some_and(|t| t == &Tile::Empty || t == &Tile::End)),
                                |_, _| 1,
                                |_, _| 0,
                                |node| grid.get(node).is_some_and(|t| t == &Tile::End),
                                false,
    ) {
        for node in &result.first() {
            count += diamond_search(grid,
                                    &result,
                                    node,
                                    result.visited.get(node).unwrap().cost,
                                    max_steps,
                                    100,
            );
        }
    }
    count
}

fn diamond_search(grid: &Grid<Tile>, result: &PathResult<Node, i64>, start: &Node, cost: i64, max_steps: i64, min_saved: i64) -> i64 {
    let mut count = 0;
    (-max_steps..=max_steps).for_each(|x| {
        let dist_left = max_steps - x.abs();
        (-dist_left..=dist_left).for_each(|y| {
            let steps = x.abs() + y.abs();
            if steps > max_steps {
                return;
            }
            let node = Node::new(start.x + x, start.y + y);
            if let Some(tile) = grid.get(&node) {
                if tile == &Tile::Empty || tile == &Tile::End {
                    let skip_cost = result.visited.get(&node).unwrap().cost;
                    let saved = skip_cost - cost - steps;
                    if saved >= min_saved {
                        count += 1;
                    }
                }
            }
        })
    });
    count
}

fn start(grid: &Grid<Tile>) -> Node {
    grid.iter_enumerate()
        .find(|&(_, tile)| *tile == Tile::Start)
        .map(|(p, _)| p)
        .unwrap()
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Empty,
    #[default]
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> char {
        match tile {
            Tile::Start => 'S',
            Tile::End => 'E',
            Tile::Empty => '.',
            Tile::Wall => '#',
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Wall),
            _ => Err(()),
        }
    }
}
