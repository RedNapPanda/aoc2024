use itertools::Itertools;
use num::Zero;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::hash::Hash;

pub fn astar<N, IN, FN, C, FC, FH, FE>(
    start: &N,
    neighbors_fn: FN,
    cost_fn: FC,
    heuristic_fn: FH,
    end_fn: FE,
    find_all: bool,
) -> Option<AStarResult<N, C>>
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
    let mut parent_nodes: FxHashMap<N, PathNode<N, C>> = FxHashMap::default();
    heap.push(LowestCostNode::new(start.clone(), num::zero(), num::zero()));
    parent_nodes.insert(
        start.clone(),
        PathNode::new(num::zero(), FxHashSet::default()),
    );
    while let Some(LowestCostNode { node, cost, .. }) = heap.pop() {
        if min_cost.is_some_and(|min_cost| min_cost < cost) {
            break;
        }
        let path_node = parent_nodes.get(&node).unwrap();
        if path_node.cost < cost {
            continue;
        }
        if end_fn(&node) {
            min_cost = Some(cost);
            end_nodes.push(node);
            if find_all {
                continue;
            } else {
                break;
            }
        }
        for neighbor in neighbors_fn(&node) {
            let new_cost = cost + cost_fn(&node, &neighbor);
            let heuristic = heuristic_fn(&node, &neighbor);
            match parent_nodes.entry(neighbor.clone()) {
                Entry::Occupied(mut entry) => {
                    let path_node = entry.get_mut();
                    if new_cost < path_node.cost {
                        path_node.cost = new_cost;
                        path_node.parents.clear();
                    }
                    if new_cost <= path_node.cost {
                        path_node.parents.insert(node.clone());
                    }
                    if new_cost >= path_node.cost {
                        continue;
                    }
                }
                Entry::Vacant(entry) => {
                    let mut parents = FxHashSet::default();
                    parents.insert(node.clone());
                    let path_node = PathNode::new(new_cost, parents);
                    entry.insert(path_node);
                }
            };
            heap.push(LowestCostNode::new(neighbor.clone(), new_cost, heuristic));
        }
    }
    min_cost.map(|cost| AStarResult {
        cost,
        end_nodes,
        parent_nodes,
    })
}

#[derive(Debug, Clone)]
pub struct AStarResult<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    pub cost: C,
    pub end_nodes: Vec<N>,
    pub parent_nodes: FxHashMap<N, PathNode<N, C>>,
}

impl<N, C> AStarResult<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    pub fn collect(&self) -> Vec<Vec<N>> {
        let sink = &mut vec![self.end_nodes.clone()];
        let mut paths = Vec::new();
        while sink.last().map(Vec::len) >= Some(1) {
            self.fill(sink);
            let path = sink
                .iter()
                .rev()
                .map(|v| v.last().unwrap().clone())
                .collect_vec();
            paths.push(path);
            self.drain(sink);
        }
        paths
    }

    fn fill(&self, sink: &mut Vec<Vec<N>>) {
        loop {
            let parents = match sink.last() {
                Some(nodes) => nodes.last(),
                _ => return,
            }
            .and_then(|node| self.parent_nodes.get(node))
            .map(move |path_node| &path_node.parents);
            if parents.is_none_or(|parents| parents.is_empty()) {
                return;
            }
            sink.push(parents.unwrap().iter().cloned().collect_vec());
        }
    }

    fn drain(&self, sink: &mut Vec<Vec<N>>) {
        while sink.last().map(Vec::len) == Some(1) {
            sink.pop();
        }
        sink.last_mut().map(Vec::pop);
    }
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
