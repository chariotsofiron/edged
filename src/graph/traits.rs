/// Marker type for a directed graph.
#[derive(Copy, Clone, Debug)]
pub enum Directed {}

/// Marker type for an undirected graph.
#[derive(Copy, Clone, Debug)]
pub enum Undirected {}

/// A graph's edge type determines whether it has directed edges or not.
pub trait Direction {
    fn is_directed() -> bool;
}

impl Direction for Directed {
    #[inline]
    fn is_directed() -> bool {
        true
    }
}

impl Direction for Undirected {
    #[inline]
    fn is_directed() -> bool {
        false
    }
}

pub trait Children {
    type Iter: Iterator<Item = usize>;
    /// Returns an iterator over the children for a vertex
    fn children(self, vertex: usize) -> Self::Iter;
}

pub trait Parents {
    type Iter: Iterator<Item = usize>;
    /// Returns an iterator over the children for a vertex
    fn parents(self, vertex: usize) -> Self::Iter;
}
