use crate::utils::node::Node;
use std::fmt::{Display, Formatter, Write};
use strum_macros::EnumIter;

#[derive(Clone, Debug, EnumIter, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    pub fn turn_right(&self) -> Self {
        self.turn_left().inverse()
    }

    pub fn vector(&self) -> Node<i64> {
        match self {
            Direction::North => Node { x: 0, y: -1 },
            Direction::South => Node { x: 0, y: 1 },
            Direction::East => Node { x: 1, y: 0 },
            Direction::West => Node { x: -1, y: 0 },
        }
    }
}

impl From<&Direction> for char {
    // ⭠⭡⭢⭣⭦⭧⭨⭩
    fn from(dir: &Direction) -> Self {
        match dir {
            Direction::North => '⭡',
            Direction::South => '⭣',
            Direction::East => '⭢',
            Direction::West => '⭠',
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(self))
    }
}

impl From<Node<i64>> for Direction {
    fn from(value: Node<i64>) -> Self {
        match value {
            Node { x: 0, y: 0 } => panic!("Not a vector"),
            Node { x: 0, y } if y < 0 => Direction::North,
            Node { x: 0, y } if y > 0 => Direction::South,
            Node { x, y: 0 } if x > 0 => Direction::East,
            Node { x, y: 0 } if x < 0 => Direction::West,
            _ => panic!("diagonals not implemented"),
        }
    }
}
