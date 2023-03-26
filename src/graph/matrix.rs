//! Adjacency matrix graph implementation.
use core::marker::PhantomData;

use super::{
    traits::{Children, Directed, Direction, NodeCount, Outgoing, Parents},
    util::{extend_linearized_matrix, to_linear_matrix_position},
};

/// A graph represented using an adjacency matrix.
pub struct Graph<E, Ty = Directed> {
    /// The node adjacencies.
    adjacencies: Vec<Option<E>>,
    /// The number of nodes that can be stored in the graph without reallocating.
    n_nodes: usize,
    /// The number of edges in the graph.
    n_edges: usize,
    /// Whether the graph is directed or undirected.
    ty: PhantomData<Ty>,
}

impl<E, Ty> Graph<E, Ty>
where
    Ty: Direction,
{
    /// Constructs a new graph.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            adjacencies: Vec::new(),
            n_edges: 0,
            n_nodes: 0,
            ty: PhantomData,
        }
    }

    /// Creates a new graph with the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        let mut adjacencies = Vec::new();
        extend_linearized_matrix::<Ty, Option<E>>(&mut adjacencies, 0, capacity);
        Self {
            adjacencies,
            n_edges: 0,
            n_nodes: capacity,
            ty: PhantomData,
        }
    }

    /// Returns `true` if the graph contains no edges.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.n_edges == 0
    }

    /// Adds an edge from `a` to `b` to the graph.
    ///
    /// # Panics
    ///
    /// Panics if either of the nodes don't exist.
    pub fn add_edge(&mut self, a: usize, b: usize, weight: E) {
        // allocate if needed
        let max_node = (a + 1).max(b + 1);
        if max_node > self.n_nodes {
            extend_linearized_matrix::<Ty, Option<E>>(
                &mut self.adjacencies,
                self.n_nodes,
                max_node,
            );
            self.n_nodes = max_node;
        }

        let index = to_linear_matrix_position::<Ty>(a, b, self.n_nodes);
        self.adjacencies[index] = Some(weight);
    }
}

/// Constructs a weighted graph from an array of edges.
impl<const N: usize, E, Ty: Direction> From<[(usize, usize, E); N]> for Graph<E, Ty> {
    /// Constructs a graph from an array of edges.
    fn from(edges: [(usize, usize, E); N]) -> Self {
        let mut graph = Self::new();
        for (from, to, weight) in edges {
            graph.add_edge(from, to, weight);
        }
        graph
    }
}

/// Constructs an unweighted graph from an array of edges.
impl<const N: usize, Ty: Direction> From<[(usize, usize); N]> for Graph<(), Ty> {
    fn from(edges: [(usize, usize); N]) -> Self {
        let mut graph = Self::new();
        for (from, to) in edges {
            graph.add_edge(from, to, ());
        }
        graph
    }
}

impl FromIterator<(usize, usize)> for Graph<(), Directed> {
    fn from_iter<I: IntoIterator<Item = (usize, usize)>>(iter: I) -> Self {
        let mut graph = Self::new();
        for (from, to) in iter {
            graph.add_edge(from, to, ());
        }
        graph
    }
}

/// The direction of the iterator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterDirection {
    /// Iterate over rows i.e. children.
    Rows,
    /// Iterate over columns i.e. parents.
    Columns,
}

/// Iterator over the edges of from or to a node
#[derive(Debug, Clone)]
pub struct Edges<'graph, Ty: Direction, E: 'graph> {
    /// The direction of the iterator.
    iter_direction: IterDirection,
    /// The node adjacencies.
    adjacencies: &'graph [Option<E>],
    /// The number of nodes that can be stored in the graph without reallocating.
    node_capacity: usize,
    /// The row of the next edge to be returned.
    row: usize,
    /// The column of the next edge to be returned.
    column: usize,
    /// The type of the graph.
    ty: PhantomData<Ty>,
}

impl<'graph, Ty: Direction, E: 'graph> Iterator for Edges<'graph, Ty, E> {
    type Item = (usize, &'graph E);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.row.max(self.column) >= self.node_capacity {
                return None;
            }

            let (row, column) = (self.row, self.column);

            match self.iter_direction {
                IterDirection::Rows => self.row += 1,
                IterDirection::Columns => self.column += 1,
            }

            let p = to_linear_matrix_position::<Ty>(row, column, self.node_capacity);
            if let Some(e) = self.adjacencies[p].as_ref() {
                let b = match self.iter_direction {
                    IterDirection::Rows => row,
                    IterDirection::Columns => column,
                };
                return Some((b, e));
            }
        }
    }
}

/// Iterator over the edges of from or to a node
#[derive(Debug, Clone)]
pub struct Neighbors<'graph, Ty: Direction, E: 'graph> {
    /// The direction of the iterator.
    iter_direction: IterDirection,
    /// The node adjacencies.
    adjacencies: &'graph [Option<E>],
    /// The number of nodes that can be stored in the graph without reallocating.
    node_capacity: usize,
    /// The row of the next edge to be returned.
    row: usize,
    /// The column of the next edge to be returned.
    column: usize,
    /// The type of the graph.
    ty: PhantomData<Ty>,
}

impl<'graph, Ty: Direction, E: 'graph> Iterator for Neighbors<'graph, Ty, E> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.row.max(self.column) >= self.node_capacity {
                return None;
            }

            let (row, column) = (self.row, self.column);

            match self.iter_direction {
                IterDirection::Rows => self.row += 1,
                IterDirection::Columns => self.column += 1,
            }

            let p = to_linear_matrix_position::<Ty>(row, column, self.node_capacity);
            if self.adjacencies[p].is_some() {
                let neighbor = match self.iter_direction {
                    IterDirection::Rows => row,
                    IterDirection::Columns => column,
                };
                return Some(neighbor);
            }
        }
    }
}

impl<'graph, E, Ty> Children for &'graph Graph<E, Ty>
where
    Ty: Direction,
{
    type Iter = Neighbors<'graph, Ty, E>;

    fn children(self, node: usize) -> Neighbors<'graph, Ty, E> {
        Neighbors {
            iter_direction: IterDirection::Columns,
            adjacencies: &self.adjacencies,
            node_capacity: self.n_nodes,
            row: node,
            column: 0,
            ty: PhantomData,
        }
    }
}

impl<'graph, E, Ty> Parents for &'graph Graph<E, Ty>
where
    Ty: Direction,
{
    type Iter = Neighbors<'graph, Ty, E>;

    fn parents(self, node: usize) -> Neighbors<'graph, Ty, E> {
        Neighbors {
            iter_direction: IterDirection::Rows,
            adjacencies: &self.adjacencies,
            node_capacity: self.n_nodes,
            row: 0,
            column: node,
            ty: PhantomData,
        }
    }
}

impl<'graph, E, Ty> NodeCount for &'graph Graph<E, Ty> {
    fn node_count(self) -> usize {
        self.n_nodes
    }
}

impl<'graph, E, Ty> Outgoing<&'graph E> for &'graph Graph<E, Ty>
where
    Ty: Direction,
{
    type Iter = Edges<'graph, Ty, E>;

    fn outgoing(self, node: usize) -> Edges<'graph, Ty, E> {
        Edges {
            iter_direction: IterDirection::Columns,
            adjacencies: &self.adjacencies,
            node_capacity: self.n_nodes,
            row: node,
            column: 0,
            ty: PhantomData,
        }
    }
}
