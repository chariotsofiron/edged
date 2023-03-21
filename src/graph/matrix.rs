use std::marker::PhantomData;

use super::{
    graph_type::{Directed, GraphType},
    util::{extend_linear_matrix, to_linearized_matrix_position},
};

pub trait Nullable: Default + Into<Option<<Self as Nullable>::Wrapped>> {
    type Wrapped;
    fn new(value: Self::Wrapped) -> Self;
    fn is_null(&self) -> bool;
    fn unwrap(&self) -> Self::Wrapped;
    fn as_ref(&self) -> Option<&Self::Wrapped>;
}

pub struct Graph<E, Ty = Directed> {
    /// The node adjacencies.
    adjacencies: Vec<Option<E>>,
    /// The number of vertices in the graph.
    len: usize,
    /// The number of vertices that can be stored in the graph without reallocating.
    capacity: usize,
    /// The number of edges in the graph.
    n_edges: usize,
    ty: PhantomData<Ty>,
}

impl<E, Ty> Graph<E, Ty>
where
    Ty: GraphType,
{
    // Tiny Vecs are dumb. Skip to:
    // - 8 if the element size is 1, because any heap allocators is likely
    //   to round up a request of less than 8 bytes to at least 8 bytes.
    // - 4 if elements are moderate-sized (<= 1 KiB).
    const MIN_NON_ZERO_CAP: usize = if std::mem::size_of::<Option<E>>() == 1 {
        8
    } else {
        4
    };

    pub fn new() -> Self {
        Self {
            adjacencies: Vec::new(),
            len: 0,
            n_edges: 0,
            capacity: 0,
            ty: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut graph = Self {
            adjacencies: Vec::new(),
            len: 0,
            n_edges: 0,
            capacity: 0,
            ty: PhantomData,
        };
        graph.reserve_exact(capacity);
        graph
    }

    /// Returns the number of vertices in the graph.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the total number of vertices the `Graph` can hold without
    /// reallocating.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn update_edge(&mut self, a: usize, b: usize, weight: E) {
        // allocate if needed
        let additional = (a + 1).max(b + 1).max(self.capacity) - self.capacity;
        self.reserve(additional);

        let index = self.to_edge_position_unchecked(a, b);
        let old_weight = std::mem::replace(&mut self.adjacencies[index], Some(weight));
        if old_weight.is_none() {
            self.n_edges += 1;
        }
    }

    /// Remove the edge from `a` to `b` to the graph.
    ///
    /// **Panics** if either of the nodes don't exist.
    pub fn remove_edge(&mut self, a: usize, b: usize) -> Option<E> {
        let index = self.to_edge_position_unchecked(a, b);
        let old_weight = std::mem::replace(&mut self.adjacencies[index], None);
        if old_weight.is_some() {
            self.n_edges -= 1;
        }
        old_weight
    }

    /// Returns `true` if the buffer needs to grow to fulfill the needed extra capacity.
    /// Mainly used to make inlining reserve-calls possible without inlining `grow`.
    fn needs_to_grow(&self, additional: usize) -> bool {
        additional > self.capacity.wrapping_sub(self.len)
    }

    /// Reserves capacity for at least `additional` more vertices to be inserted
    /// in the given `Graph`. The collection may reserve more space to
    /// speculatively avoid frequent reallocations. After calling `reserve`,
    /// capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if capacity is already sufficient.
    pub fn reserve(&mut self, additional: usize) {
        if self.needs_to_grow(additional) {
            // stolen from Vec::reserve
            let required_cap = self.len.checked_add(additional).unwrap();
            let capacity = std::cmp::max(self.capacity * 2, required_cap);
            let capacity = std::cmp::max(Self::MIN_NON_ZERO_CAP, capacity);
            self.capacity = extend_linear_matrix::<Ty, Option<E>>(&mut self.adjacencies, capacity);
        }
    }

    /// Reserves the minimum capacity for at least `additional` more vertices to
    /// be inserted in the given `Graph`. Unlike [`reserve`], this will not
    /// deliberately over-allocate to speculatively avoid frequent allocations.
    /// After calling `reserve_exact`, capacity will be greater than or equal to
    /// `self.len() + additional`. Does nothing if the capacity is already
    /// sufficient.
    ///
    /// Note that the allocator may give the collection more space than it
    /// requests. Therefore, capacity can not be relied upon to be precisely
    /// minimal. Prefer [`reserve`] if future insertions are expected.
    ///
    /// [`reserve`]: Graph::reserve
    pub fn reserve_exact(&mut self, additional: usize) {
        if self.needs_to_grow(additional) {
            let capacity = self.len.checked_add(additional).unwrap();
            self.capacity = extend_linear_matrix::<Ty, Option<E>>(&mut self.adjacencies, capacity);
        }
    }

    /// Converts a pair of node indices to a linearized matrix position.
    /// Returns `None` if the indices are out of bounds.
    #[inline]
    fn to_edge_position(&self, a: usize, b: usize) -> Option<usize> {
        if std::cmp::max(a, b) >= self.capacity {
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
        to_linearized_matrix_position::<Ty>(a, b, self.capacity)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NeighborIterDirection {
    Rows,
    Columns,
}

/// Iterator over the edges of from or to a node
#[derive(Debug, Clone)]
pub struct Edges<'a, Ty: GraphType, Null: 'a + Nullable> {
    iter_direction: NeighborIterDirection,
    node_adjacencies: &'a [Null],
    node_capacity: usize,
    row: usize,
    column: usize,
    ty: PhantomData<Ty>,
}

impl<'a, Ty: GraphType, Null: 'a + Nullable> Edges<'a, Ty, Null> {
    fn on_columns(row: usize, node_adjacencies: &'a [Null], node_capacity: usize) -> Self {
        Edges {
            iter_direction: NeighborIterDirection::Columns,
            node_adjacencies,
            node_capacity,
            row,
            column: 0,
            ty: PhantomData,
        }
    }

    fn on_rows(column: usize, node_adjacencies: &'a [Null], node_capacity: usize) -> Self {
        Edges {
            iter_direction: NeighborIterDirection::Rows,
            node_adjacencies,
            node_capacity,
            row: 0,
            column,
            ty: PhantomData,
        }
    }
}

impl<'a, Ty: GraphType, Null: Nullable> Iterator for Edges<'a, Ty, Null> {
    type Item = (usize, usize, &'a Null::Wrapped);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.row.max(self.column) >= self.node_capacity {
                return None;
            }

            let (row, column) = (self.row, self.column);

            match self.iter_direction {
                Rows => self.row += 1,
                Columns => self.column += 1,
            }

            let p = to_linearized_matrix_position::<Ty>(row, column, self.node_capacity);
            if let Some(e) = self.node_adjacencies[p].as_ref() {
                let (a, b) = match self.iter_direction {
                    Rows => (column, row),
                    Columns => (row, column),
                };
                return Some((a, b, e));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::graph_type::Directed;

    use super::Graph;

    #[test]
    fn test_graph() {
        let mut g = Graph::<(), Directed>::new();

        println!("len: {}, capacity: {}", g.len(), g.capacity());
        g.update_edge(0, 1, ());
        println!("len: {}, capacity: {}", g.len(), g.capacity());
        // g.update_edge(2, 4, ());
        // println!("len: {}, capacity: {}", g.len(), g.capacity());
    }
}
