//! Traits for graphs.

/// Marker type for a directed graph.
#[derive(Copy, Clone, Debug)]
pub enum Directed {}

/// Marker type for an undirected graph.
#[derive(Copy, Clone, Debug)]
pub enum Undirected {}

/// A graph's edge type determines whether it has directed edges or not.
pub trait Direction {
    /// Returns `true` if the graph is directed.
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

/// A copyable reference to a graph.
pub trait GraphRef: Copy {}

impl<'graph, G> GraphRef for &'graph G {}

/// A trait for graphs that can be iterated over.
pub trait Children: GraphRef {
    /// The type of the iterator returned by `children`.
    type Iter: Iterator<Item = usize>;
    /// Returns an iterator over the children for a vertex
    fn children(self, vertex: usize) -> Self::Iter;
}

/// A trait for graphs that can be iterated over in reverse.
pub trait Parents: GraphRef {
    /// The type of the iterator returned by `parents`.
    type Iter: Iterator<Item = usize>;
    /// Returns an iterator over the children for a vertex
    fn parents(self, vertex: usize) -> Self::Iter;
}
