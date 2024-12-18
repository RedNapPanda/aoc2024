use std::hash::BuildHasherDefault;
use indexmap::{IndexMap, IndexSet};
use rustc_hash::FxHasher;

pub mod grid;
pub mod input;
pub mod macros;
pub mod node;
pub mod direction;
pub mod astar;

type BuildFxHasherDefault = BuildHasherDefault<FxHasher>;
type FxIndexMap<K, V> = IndexMap<K, V, >;
type FxIndexSet<K> = IndexSet<K, BuildFxHasherDefault>;