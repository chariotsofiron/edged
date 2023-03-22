use std::{fmt::Debug, marker::PhantomData};

use super::{
    traits::{Directed, Direction},
    util::{extend_linearized_matrix, to_linear_matrix_position},
};

pub struct Graph<E, Ty = Directed> {
    /// The node adjacencies.
    adjacencies: Vec<Option<E>>,
    /// The number of vertices that can be stored in the graph without reallocating.
    n_nodes: usize,
    /// The number of edges in the graph.
    n_edges: usize,
    ty: PhantomData<Ty>,
}

impl<E, Ty> Graph<E, Ty>
where
    Ty: Direction,
{
    pub fn new() -> Self {
        Self {
            adjacencies: Vec::new(),
            n_edges: 0,
            n_nodes: 0,
            ty: PhantomData,
        }
    }

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

    /// Returns the number of vertices in the graph.
    pub fn len(&self) -> usize {
        self.n_nodes
    }

    pub fn update_edge(&mut self, a: usize, b: usize, weight: E) -> Option<E> {
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

        let index = self.to_edge_position_unchecked(a, b);
        let old_weight = std::mem::replace(&mut self.adjacencies[index], Some(weight));
        if old_weight.is_none() {
            self.n_edges += 1;
        }
        old_weight
    }

    pub fn add_edge(&mut self, a: usize, b: usize, weight: E) {
        assert!(self.update_edge(a, b, weight).is_none());
    }

    /// Remove the edge from `a` to `b` to the graph.
    ///
    /// **Panics** if either of the nodes don't exist.
    pub fn remove_edge(&mut self, a: usize, b: usize) -> E {
        let index = self.to_edge_position_unchecked(a, b);
        let old_weight = std::mem::replace(&mut self.adjacencies[index], None);
        if old_weight.is_some() {
            self.n_edges -= 1;
        }
        old_weight.expect("Edge doesn't exist")
    }

    /// Converts a pair of node indices to a linearized matrix position.
    /// Returns `None` if the indices are out of bounds.
    #[inline]
    fn to_edge_position(&self, a: usize, b: usize) -> Option<usize> {
        if std::cmp::max(a, b) >= self.n_nodes {
            return None;
        }
        Some(self.to_edge_position_unchecked(a, b))
    }

    /// Converts a pair of node indices to a linearized matrix position.
    ///
    /// # Panics
    ///
    /// Panics if the indices are out of bounds.
    #[inline]
    fn to_edge_position_unchecked(&self, a: usize, b: usize) -> usize {
        to_linear_matrix_position::<Ty>(a, b, self.n_nodes)
    }

    pub fn children(&self, node: usize) -> Edges<'_, Ty, E> {
        Edges {
            iter_direction: NeighborIterDirection::Columns,
            adjacencies: &self.adjacencies,
            node_capacity: self.n_nodes,
            row: node,
            column: 0,
            ty: PhantomData,
        }
    }

    pub fn parents(&self, node: usize) -> Edges<'_, Ty, E> {
        Edges {
            iter_direction: NeighborIterDirection::Rows,
            adjacencies: &self.adjacencies,
            node_capacity: self.n_nodes,
            row: 0,
            column: node,
            ty: PhantomData,
        }
    }
}

impl<const N: usize, E: Debug, Ty: Direction> From<[(usize, usize, E); N]> for Graph<E, Ty> {
    /// Constructs a graph from an array of edges.
    fn from(edges: [(usize, usize, E); N]) -> Self {
        let mut graph = Self::new();
        for (from, to, weight) in edges {
            graph.add_edge(from, to, weight);
        }
        graph
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NeighborIterDirection {
    Rows,
    Columns,
}

/// Iterator over the edges of from or to a node
#[derive(Debug, Clone)]
pub struct Edges<'a, Ty: Direction, E: 'a> {
    iter_direction: NeighborIterDirection,
    adjacencies: &'a [Option<E>],
    node_capacity: usize,
    row: usize,
    column: usize,
    ty: PhantomData<Ty>,
}

impl<'a, Ty: Direction, E: 'a> Iterator for Edges<'a, Ty, E> {
    type Item = (usize, usize, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.row.max(self.column) >= self.node_capacity {
                return None;
            }

            let (row, column) = (self.row, self.column);

            match self.iter_direction {
                NeighborIterDirection::Rows => self.row += 1,
                NeighborIterDirection::Columns => self.column += 1,
            }

            let p = to_linear_matrix_position::<Ty>(row, column, self.node_capacity);
            if let Some(e) = self.adjacencies[p].as_ref() {
                let (a, b) = match self.iter_direction {
                    NeighborIterDirection::Rows => (column, row),
                    NeighborIterDirection::Columns => (row, column),
                };
                return Some((a, b, e));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use crate::graph::traits::Directed;

    #[test]
    fn test_graph() {
        // let mut g = Graph::<(), Directed>::new();

        // println!("len: {}, capacity: {}", g.len(), g.capacity());
        // g.update_edge(0, 1, ());
        // println!("len: {}, capacity: {}", g.len(), g.capacity());
        // // g.update_edge(2, 4, ());
        // // println!("len: {}, capacity: {}", g.len(), g.capacity());

        let graph = Graph::<(), Directed>::from([(2, 3, ()), (2, 4, ()), (4, 1, ()), (1, 2, ())]);

        println!("{:?}", graph.children(2).collect::<Vec<_>>());
    }
}
