//! A graph data structure using an adjacency list representation.

/// An adjacency list graph data structure.
/// 
/// Allows parallel edges and self-loops.
/// 
/// This data structure is append-only (except for clear), so indices
/// returned at some point for a given graph will stay valid with this same
/// graph until it is dropped or clear is called.
/// 
/// Space complexity: **O(|V| + |E|)**
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Graph {
    /// Maps a node id to the first edge in its adjacency list.
    first: Vec<Option<usize>>,
    /// Maps an edge id to the next edge in the same adjacency list.
    next_edge: Vec<Option<usize>>,
    /// Maps an edge id to the node that it points to.
    end_vertex: Vec<usize>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    /// Constructs an empty graph.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            first: Vec::new(),
            next_edge: Vec::new(),
            end_vertex: Vec::new(),
        }
    }

    /// Constructs an empty graph with hints for number of vertices and edges
    /// to reduce unnecessary allocations.
    #[must_use]
    pub fn with_capacity(vertices: usize, edges: usize) -> Self {
        Self {
            first: Vec::with_capacity(vertices),
            next_edge: Vec::with_capacity(edges),
            end_vertex: Vec::with_capacity(edges),
        }
    }

    /// Returns the max number of vertices for the graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.first.len()
    }

    /// Returns true if the graph has no edges.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edge_count() == 0
    }

    /// Returns the number of edges in the graph.
    /// Double-counts undirected edges. Includes parallel edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.end_vertex.len()
    }

    /// Adds a directed edge to the graph from `from` to `to`. Returns the edge index.
    pub fn push(&mut self, from: usize, to: usize) -> usize {
        // update length of first if necessary
        let len = self
            .first
            .len()
            .max(from.saturating_add(1))
            .max(to.saturating_add(1));
        self.first.resize_with(len, || None);

        // add the edge
        self.next_edge.push(self.first[from]);
        let edge_index = self.end_vertex.len();
        self.first[from] = Some(edge_index);
        self.end_vertex.push(to);
        edge_index
    }

    /// Returns an iterator of all node-edge tuples with an edge starting from `node`.
    /// Produces an empty iterator if `node` doesn't exist.
    #[must_use]
    pub fn neighbors(&self, node: usize) -> NeighborIterator {
        NeighborIterator {
            graph: self,
            next_edge: self.first.get(node).copied().flatten(),
        }
    }

    /// Returns an iterator over all edges in the graph.
    /// Does not return them in insertion order.
    #[must_use]
    pub fn edges(&self) -> EdgesIterator {
        EdgesIterator {
            graph: self,
            parent: 0, // start from zero and go to len()
            neighbors: self.neighbors(0),
        }
    }

    /// Returns a transposed version of the graph.
    /// <https://en.wikipedia.org/wiki/Transpose_graph>
    #[must_use]
    pub fn transpose(&self) -> Self {
        let mut graph = Self::with_capacity(self.len(), self.edge_count());
        graph.extend(self.edges().map(|(to, from)| (from, to)));
        graph
    }
}

/// An iterator for all edges in the graph.
pub struct EdgesIterator<'graph> {
    /// The graph that this iterator is iterating over.
    graph: &'graph Graph,
    /// The current parent vertex.
    parent: usize,
    /// The current neighbor iterator.
    neighbors: NeighborIterator<'graph>,
}

impl Iterator for EdgesIterator<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((neighbor, _)) = self.neighbors.next() {
            Some((self.parent, neighbor))
        } else {
            self.parent = self.parent.saturating_add(1);
            if self.parent < self.graph.len() {
                self.neighbors = self.graph.neighbors(self.parent);
                self.next()
            } else {
                None
            }
        }
    }
}

/// An iterator for convenient adjacency list traversal.
pub struct NeighborIterator<'graph> {
    /// The graph that this iterator is iterating over.
    graph: &'graph Graph,
    /// The next edge in the adjacency list.
    next_edge: Option<usize>,
}

impl<'graph> Iterator for NeighborIterator<'graph> {
    type Item = (usize, usize);

    /// Produces an outgoing edge and vertex.
    fn next(&mut self) -> Option<Self::Item> {
        let next_edge = self.next_edge?;
        let v = self.graph.end_vertex[next_edge];
        self.next_edge = self.graph.next_edge[next_edge];
        Some((v, next_edge))
    }
}

impl Extend<(usize, usize)> for Graph {
    fn extend<T: IntoIterator<Item = (usize, usize)>>(&mut self, iter: T) {
        for (from, to) in iter {
            self.push(from, to);
        }
    }
}

impl<const N: usize> From<[(usize, usize); N]> for Graph {
    /// Constructs a graph from an array of edges.
    fn from(edges: [(usize, usize); N]) -> Self {
        let mut graph = Self::new();
        for (from, to) in edges {
            graph.push(from, to);
        }
        graph
    }
}

impl From<&[(usize, usize)]> for Graph {
    /// Constructs a graph from a slice of edges.
    fn from(edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new();
        for &(from, to) in edges {
            graph.push(from, to);
        }
        graph
    }
}

impl FromIterator<(usize, usize)> for Graph {
    fn from_iter<T: IntoIterator<Item = (usize, usize)>>(iter: T) -> Self {
        let mut graph = Self::new();
        graph.extend(iter);
        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let mut graph = Graph::new();
        assert!(graph.is_empty());
        assert_eq!(graph.len(), 0);
        assert_eq!(graph.push(0, 1), 0);
        assert_eq!(graph.push(0, 1), 1);
        assert_eq!(graph.push(1, 1), 2);
        assert_eq!(graph.push(1, 0), 3);
        assert!(!graph.is_empty());
        assert_eq!(graph.len(), 2);
        assert_eq!(graph.edge_count(), 4);
    }

    #[test]
    fn test_neighbors() {
        let graph = Graph::from([(2, 3), (2, 4), (4, 1), (1, 2)]);
        assert_eq!(graph.edge_count(), 4);
        assert_eq!(graph.neighbors(2).collect::<Vec<_>>(), [(4, 1), (3, 0)]);

        let edges = graph.edges().collect::<Vec<_>>();
        assert_eq!(edges, [(1, 2), (2, 4), (2, 3), (4, 1)]);
    }

    #[test]
    fn test_transpose() {
        let graph = Graph::from([(2, 3), (2, 4), (1, 3)]);
        assert_eq!(graph.neighbors(2).collect::<Vec<_>>(), [(4, 1), (3, 0)]);
        let transpose = graph.transpose();
        assert_eq!(transpose.neighbors(3).collect::<Vec<_>>(), [(2, 2), (1, 0)]);
    }
}
