/// Marker type for a directed graph.
#[derive(Copy, Clone, Debug)]
pub enum Directed {}

/// Marker type for an undirected graph.
#[derive(Copy, Clone, Debug)]
pub enum Undirected {}

/// A graph's edge type determines whether it has directed edges or not.
pub trait GraphType {
    fn is_directed() -> bool;
}

impl GraphType for Directed {
    #[inline]
    fn is_directed() -> bool {
        true
    }
}

impl GraphType for Undirected {
    #[inline]
    fn is_directed() -> bool {
        false
    }
}