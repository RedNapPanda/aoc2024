use crate::utils::astar::astar;
use crate::utils::grid::Grid;
use crate::utils::node::Node;

pub fn solve1(lines: &[String]) -> i64 {
    let size = 71;
    let cutoff = 1024;
    let mut grid = Grid::with_default(size, size, '.');
    for line in lines[..cutoff].iter() {
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
        |node| node.x == (size - 1) as i64 && node.y == (size - 1) as i64,
        false,
    ) {
        return result.cost
    }
    0
}

pub fn solve2(lines: &[String]) -> i64 {
    let size = 71;
    let cutoff = 1024;
    let mut grid = Grid::with_default(size, size, '.');
    for line in lines[..cutoff].iter() {
        if let Some(pos) = line
            .split_once(',')
            .map(|(c, r)| (r.parse::<i64>().unwrap(), c.parse::<i64>().unwrap()))
        {
            grid.set(&Node::from(pos), '#');
        }
    }
    for i in cutoff..lines.len() {
        let (row, col) = lines[i].split_once(',')
            .map(|(c, r)| (r.parse::<i64>().unwrap(), c.parse::<i64>().unwrap()))
            .unwrap();
        grid.set(&Node::new(row, col), '#');
        if astar(
            &Node::new(0, 0),
            |node| {
                grid.neighbors_cardinal(node)
                    .into_iter()
                    .filter(|node| grid.get(node).is_some_and(|&c| c == '.'))
            },
            |_, _| 1,
            |_, _| 0,
            |node| node.x == (size - 1) as i64 && node.y == (size - 1) as i64,
            false,
        ).is_none() {
            println!("Solution: {},{}", col, row);
            return 0
        }
    }
    0
}
