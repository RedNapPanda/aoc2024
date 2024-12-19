use std::fmt::Debug;
use indexmap::{IndexMap, IndexSet};
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::hash::{BuildHasherDefault, Hash};
use num::Zero;

pub mod direction;
pub mod grid;
pub mod input;
pub mod macros;
pub mod node;
pub mod algo;

type _BuildFxHasherDefault = BuildHasherDefault<FxHasher>;
type _FxIndexMap<K, V> = IndexMap<K, V, _BuildFxHasherDefault>;
type _FxIndexSet<K> = IndexSet<K, _BuildFxHasherDefault>;

#[derive(Debug, Clone)]
pub struct PathNode<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    pub cost: C,
    pub parents: FxHashSet<N>,
}

impl<N, C> PathNode<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    fn new(cost: C, parents: FxHashSet<N>) -> Self {
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