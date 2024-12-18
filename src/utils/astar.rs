use num::Zero;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::hash::Hash;
use itertools::Itertools;

pub fn astar<N, IN, FN, C, FC, FH, FE>(
    start: &N,
    neighbors_fn: FN,
    cost_fn: FC,
    heuristic_fn: FH,
    end_fn: FE,
) -> Option<(C, Vec<N>, FxHashMap<N, PathNode<N, C>>)>
where
    N: Eq + Hash + Clone + Debug,
    IN: IntoIterator<Item = N>,
    FN: Fn(&N) -> IN,
    C: Zero + Copy + Ord,
    FC: Fn(&N, &N) -> C,
    FH: Fn(&N, &N) -> C,
    FE: Fn(&N) -> bool,
{
    let mut min_cost: Option<C> = None;
    let mut end_nodes: Vec<N> = Vec::new();
    let mut heap: BinaryHeap<LowestCostNode<N, C>> = BinaryHeap::new();
    let mut paths: FxHashMap<N, PathNode<N, C>> = FxHashMap::default();
    heap.push(LowestCostNode::new(start.clone(), num::zero(), num::zero()));
    paths.insert(start.clone(), PathNode::new(num::zero(), FxHashSet::default()));
    while let Some(LowestCostNode { node, cost, .. }) = heap.pop() {
        if min_cost.is_some_and(|min_cost| min_cost < cost) {
            break;
        }
        println!("{:?}", &node);
        let path_node = paths.get(&node).unwrap();
        if path_node.cost < cost {
            continue;
        }
        if end_fn(&node) {
            min_cost = Some(cost);
            end_nodes.push(node);
            continue;
        }
        let neighbors = neighbors_fn(&node)
            .into_iter()
            .filter(|neighbor| !path_node.parents.contains(neighbor))
            .collect_vec();
        
        for neighbor in neighbors {
            let new_cost = cost + cost_fn(&node, &neighbor);
            let heuristic = heuristic_fn(&node, &neighbor);
            match paths.entry(neighbor.clone()) {
                Entry::Occupied(mut entry) => {
                    let path_node = entry.get_mut();
                    if new_cost < path_node.cost {
                        path_node.parents.clear();
                        path_node.cost = new_cost;
                    }
                    path_node.parents.insert(node.clone());
                    if new_cost >= path_node.cost {
                        continue
                    }
                }
                Entry::Vacant(entry) => {
                    let mut parents = FxHashSet::default();
                    parents.insert(node.clone());
                    let path_node = PathNode::new(new_cost, parents);
                    entry.insert(path_node);
                }
            }
            heap.push(LowestCostNode::new(neighbor.clone(), new_cost, heuristic));
        }
    }
    min_cost.map(|cost| (cost, end_nodes, paths))
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
struct LowestCostNode<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    node: N,
    cost: C,
    heuristic: C,
}

impl<N, C> LowestCostNode<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    fn new(node: N, cost: C, heuristic: C) -> Self {
        Self {
            node,
            cost,
            heuristic,
        }
    }
}

impl<N, C> PartialOrd for LowestCostNode<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N, C> Ord for LowestCostNode<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}
