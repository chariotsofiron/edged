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

/// A trait for graphs where a node's children can be iterated over.
pub trait Children: GraphRef {
    /// The type of the iterator returned by `children`.
    type Iter: Iterator<Item = usize>;
    /// Returns an iterator over the children for a vertex
    fn children(self, vertex: usize) -> Self::Iter;
}

/// A trait for graphs where a node's parents can be iterated over.
pub trait Parents: GraphRef {
    /// The type of the iterator returned by `parents`.
    type Iter: Iterator<Item = usize>;
    /// Returns an iterator over the children for a vertex
    fn parents(self, vertex: usize) -> Self::Iter;
}

/// A trait for graphs where a node's outgoing edges can be iterated over.
pub trait Outgoing<E>: GraphRef {
    /// The type of the iterator returned by `outgoing`.
    type Iter: Iterator<Item = (usize, E)>;
    /// Returns an iterator over the outgoing edges for a vertex
    fn outgoing(self, vertex: usize) -> Self::Iter;
}

/// A trait for graphs where a node's incoming edges can be iterated over.
pub trait Incoming<E>: GraphRef {
    /// The type of the iterator returned by `incoming`.
    type Iter: Iterator<Item = (usize, E)>;
    /// Returns an iterator over the incoming edges for a vertex
    fn incoming(self, vertex: usize) -> Self::Iter;
}

/// A trait for graphs that have a known number of vertices.
pub trait VertexCount: GraphRef {
    /// Returns the number of vertices in the graph.
    fn vertex_count(self) -> usize;
}
