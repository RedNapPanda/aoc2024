pub trait Contains<T> {
    fn contains(&self, other: T) -> bool;
}
