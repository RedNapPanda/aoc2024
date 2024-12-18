use indexmap::{IndexMap, IndexSet};
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;

pub mod astar;
pub mod direction;
pub mod grid;
pub mod input;
pub mod macros;
pub mod node;

type _BuildFxHasherDefault = BuildHasherDefault<FxHasher>;
type _FxIndexMap<K, V> = IndexMap<K, V, _BuildFxHasherDefault>;
type _FxIndexSet<K> = IndexSet<K, _BuildFxHasherDefault>;
