//! A graph data structure using an adjacency list representation.
//!
//! The graph uses O(|V| + |E|) space, and supports O(1) edge insert.
//! It does not support node/edge deletions. It supports parallel edges.
//!
//! The data structure is not parameterized over the vertex type and just uses `usize`.
//! This leads to simpler usage, implementation, and better performance.
//!
//! Edges are numbered in order of insertion.

/// A compact directed-graph representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Graph {
    /// Maps a vertex id to the first edge in its adjacency list.
    first: Vec<Option<usize>>,
    /// Maps an edge id to the next edge in the same adjacency list.
    next_edge: Vec<Option<usize>>,
    /// Maps an edge id to the vertex that it points to.
    end_vertex: Vec<usize>,
}

impl Graph {
    /// Constructs a graph with `max_vertices` vertices and no edges.
    /// To reduce unnecessary allocations, `edge_hint` can be set close
    /// to the number of edges that will be inserted.
    #[must_use]
    pub fn new(max_vertices: usize, edge_hint: usize) -> Self {
        Self {
            first: vec![None; max_vertices],
            next_edge: Vec::with_capacity(edge_hint),
            end_vertex: Vec::with_capacity(edge_hint),
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
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.end_vertex.len()
    }

    /// Adds a directed edge to the graph from `from` to `to`. Returns the edge index.
    pub fn add_edge(&mut self, from: usize, to: usize) -> usize {
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
            next_edge: self.first[node],
        }
    }

    /// Returns a transposed version of the graph.
    /// <https://en.wikipedia.org/wiki/Transpose_graph>
    #[must_use]
    pub fn transpose(&self) -> Self {
        let mut graph = Self::new(self.len(), self.edge_count());
        for node in 0..self.len() {
            for (neighbor, _) in self.neighbors(node) {
                graph.add_edge(neighbor, node);
            }
        }
        graph
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

impl<const N: usize> From<[(usize, usize); N]> for Graph {
    /// Constructs a graph from an array of edges.
    fn from(edges: [(usize, usize); N]) -> Self {
        let vmax = edges
            .iter()
            .map(|&(u, v)| u.max(v))
            .max()
            .unwrap_or_default();
        let mut graph = Self::new(vmax.saturating_add(1), edges.len());
        for (u, v) in edges {
            graph.add_edge(u, v);
        }
        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let mut graph = Graph::new(3, 2);
        assert!(graph.is_empty());
        assert_eq!(graph.len(), 3);
        graph.add_edge(0, 1);
        graph.add_edge(0, 1);
        graph.add_edge(1, 1);
        graph.add_edge(1, 0);
        assert!(!graph.is_empty());
        assert_eq!(graph.edge_count(), 4);
    }

    #[test]
    fn test_graph() {
        let graph = Graph::from([(2, 3), (2, 4), (4, 1), (1, 2)]);

        assert_eq!(graph.len(), 5);
        assert_eq!(graph.edge_count(), 4);
        assert_eq!(graph.neighbors(2).collect::<Vec<_>>(), [(4, 1), (3, 0)]);
    }

    #[test]
    fn test_transpose() {
        let graph = Graph::from([(2, 3), (2, 4), (1, 3)]);
        assert_eq!(graph.neighbors(2).collect::<Vec<_>>(), [(4, 1), (3, 0)]);
        let transpose = graph.transpose();
        assert_eq!(transpose.neighbors(3).collect::<Vec<_>>(), [(2, 2), (1, 0)]);
    }
}
