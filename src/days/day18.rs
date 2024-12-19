use crate::utils::astar::astar;
use crate::utils::grid::Grid;
use crate::utils::node::Node;

pub fn solve1(lines: &[String]) -> i64 {
    let mut grid = Grid::with_default(71, 71, '.');
    for line in lines[..1024].iter() {
        if let Some(pos) = line
            .split_once(',')
            .map(|(c, r)| (r.parse::<i64>().unwrap(), c.parse::<i64>().unwrap()))
        {
            grid.set(&Node::from(pos), '#');
        }
    }
    if let Some(result) = astar(
        &Node::new(0, 0),
        |node| {
            grid.neighbors_cardinal(node)
                .into_iter()
                .filter(|node| grid.get(node).is_some_and(|&c| c == '.'))
        },
        |_, _| 1,
        |_, _| 0,
        |node| node.x == 70 && node.y == 70,
        false,
    ) {
        return result.cost
    }
    0
}

pub fn solve2(lines: &[String]) -> i64 {
    0
}
