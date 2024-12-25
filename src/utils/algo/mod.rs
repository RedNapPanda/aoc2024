use std::fmt::Debug;
use std::hash::Hash;
use num::Zero;
use rustc_hash::FxHashMap;

pub mod astar;


#[derive(Debug, Clone)]
pub struct PathNode<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    pub cost: C,
    pub parents: Vec<N>,
}

impl<N, C> PathNode<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    fn new(cost: C, parents: Vec<N>) -> Self {
        Self { cost, parents }
    }
}

#[derive(Debug, Clone)]
pub struct PathResult<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    pub cost: C,
    pub end_nodes: Vec<N>,
    pub visited: FxHashMap<N, PathNode<N, C>>,
}