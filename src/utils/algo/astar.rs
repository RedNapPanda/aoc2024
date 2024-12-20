use itertools::Itertools;
use num::Zero;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::hash::Hash;
use crate::utils::{PathNode, PathResult};

pub fn astar<N, IN, FN, C, FC, FH, FE>(
    start: &N,
    neighbors_fn: FN,
    cost_fn: FC,
    heuristic_fn: FH,
    end_fn: FE,
    find_all: bool,
) -> Option<PathResult<N, C>>
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
    let mut visited: FxHashMap<N, PathNode<N, C>> = FxHashMap::default();
    heap.push(LowestCostNode::new(start.clone(), num::zero(), num::zero()));
    visited.insert(
        start.clone(),
        PathNode::new(num::zero(), Vec::new()),
    );
    while let Some(LowestCostNode { node, cost, .. }) = heap.pop() {
        if min_cost.is_some_and(|min_cost| min_cost < cost) {
            break;
        }
        let path_node = visited.get(&node).unwrap();
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
            match visited.entry(neighbor.clone()) {
                Entry::Occupied(mut entry) => {
                    let path_node = entry.get_mut();
                    if new_cost < path_node.cost {
                        path_node.cost = new_cost;
                        path_node.parents.clear();
                    }
                    if new_cost <= path_node.cost {
                        path_node.parents.push(node.clone());
                    }
                    if new_cost >= path_node.cost {
                        continue;
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(PathNode::new(new_cost, vec![node.clone()]));
                }
            };
            heap.push(LowestCostNode::new(neighbor.clone(), new_cost, heuristic));
        }
    }
    min_cost.map(|cost| PathResult {
        cost,
        end_nodes,
        visited,
    })
}

impl<N, C> PathResult<N, C>
where
    N: Eq + Hash + Clone + Debug,
    C: Zero + Copy + Ord,
{
    pub fn first(&self) -> Vec<N> {
        let mut sink = if let Some(node) = self.end_nodes.last() {
            vec![node.clone()]
        } else {
            vec![]
        };
        while let Some(parent) = sink.last()
                .and_then(|node| self.visited.get(node))
                .and_then(move |path_node| path_node.parents.first()) {
            sink.push(parent.clone());
        }
        sink.reverse();
        sink
    }
    
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
            .and_then(|node| self.visited.get(node))
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
