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
    let grid = &mut Grid::<D20Tile>::from(lines);
    let start = grid.find(D20Tile::Start).unwrap();
    let mut count = 0;
    if let Some(result) = astar(&start,
                                |node| grid.neighbors_cardinal(node)
                                    .into_iter()
                                    .filter(|n| grid.get(n).is_some_and(|t| t == &D20Tile::Empty || t == &D20Tile::End)),
                                |_, _| 1,
                                |_, _| 0,
                                |node| grid.get(node).is_some_and(|t| t == &D20Tile::End),
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

fn diamond_search(grid: &Grid<D20Tile>, result: &PathResult<Node<i64>, i64>, start: &Node<i64>, cost: i64, max_steps: i64, min_saved: i64) -> i64 {
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
                if tile == &D20Tile::Empty || tile == &D20Tile::End {
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum D20Tile {
    Start,
    End,
    Empty,
    #[default]
    Wall,
}

impl Display for D20Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

impl From<&D20Tile> for char {
    fn from(tile: &D20Tile) -> char {
        match tile {
            D20Tile::Start => 'S',
            D20Tile::End => 'E',
            D20Tile::Empty => '.',
            D20Tile::Wall => '#',
        }
    }
}

impl TryFrom<char> for D20Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        let tile = match c {
            'S' => D20Tile::Start,
            'E' => D20Tile::End,
            '.' => D20Tile::Empty,
            '#' => D20Tile::Wall,
            _ => return Err(()),
        };
        Ok(tile)
    }
}
