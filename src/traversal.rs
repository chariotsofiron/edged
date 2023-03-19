//! Different traversal algorithms for the graph.
//! <https://en.wikipedia.org/wiki/Graph_traversal>
use crate::graph::{Graph, NeighborIterator};
use alloc::collections::VecDeque;

impl Graph {
    /// Returns an iterator over the (node-edge) pairs in preorder.
    /// This is the same as [depth-first-search](https://en.wikipedia.org/wiki/Depth-first_search)
    /// Note: does not include the start node.
    #[must_use]
    pub fn pre_order(&self, start: usize) -> PreOrderIterator {
        let neighbors = (0..self.len())
            .map(|node| self.neighbors(node))
            .collect::<Vec<_>>();
        let mut visited = vec![false; self.len()];
        visited[start] = true;
        PreOrderIterator {
            stack: vec![start],
            visited,
            neighbors,
        }
    }

    /// Returns an iterator over the (node-edge) pairs in level-order.
    /// This is equivalent to a [breadth first search](https://en.wikipedia.org/wiki/Breadth-first_search)
    /// Note: does not include the start node.
    #[must_use]
    pub fn level_order(&self, start: usize) -> LevelOrderIterator {
        let mut visited = vec![false; self.len()];
        visited[start] = true;
        let queue = VecDeque::new();
        let neighbors = self.neighbors(start);
        LevelOrderIterator {
            graph: self,
            visited,
            queue,
            neighbors,
        }
    }

    /// Returns an iterator over the nodes in post-order.
    /// The vertices are listed in the order in which they are last visited by a DFS traversal.
    #[must_use]
    pub fn post_order(&self, start: usize) -> PostOrderIterator {
        let neighbors = (0..self.len())
            .map(|node| self.neighbors(node))
            .collect::<Vec<_>>();
        let mut visited = vec![false; self.len()];
        visited[start] = true;
        PostOrderIterator {
            stack: vec![start],
            visited,
            neighbors,
            tail: false,
        }
    }
}

/// Iterator over the (node-edge) pairs of a graph in pre-order.
pub struct PreOrderIterator<'graph> {
    /// The stack of nodes to be visited
    stack: Vec<usize>,
    /// `true` if the node has been visited
    visited: Vec<bool>,
    /// Neighbors left to iterate for each node
    neighbors: Vec<NeighborIterator<'graph>>,
}

impl Iterator for PreOrderIterator<'_> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let &node = self.stack.last()?;
            for (neighbor, edge) in self.neighbors[node].by_ref() {
                if !self.visited[neighbor] {
                    self.visited[neighbor] = true;
                    self.stack.push(neighbor);
                    return Some((neighbor, edge));
                }
            }
            self.stack.pop();
        }
    }
}

/// Iterator over the (node-edge) pairs of a graph in level-order.
pub struct LevelOrderIterator<'graph> {
    /// The graph that this iterator is iterating over.
    graph: &'graph Graph,
    /// Queue of nodes to visit
    queue: VecDeque<usize>,
    /// `true` if the node has been visited
    visited: Vec<bool>,
    /// Neighbors of the current node being visited
    neighbors: NeighborIterator<'graph>,
}

impl Iterator for LevelOrderIterator<'_> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        if let Some((neighbor, edge)) = self.neighbors.next() {
            if !self.visited[neighbor] {
                self.visited[neighbor] = true;
                self.queue.push_back(neighbor);
            }
            Some((neighbor, edge))
        } else {
            let node = self.queue.pop_front()?;
            self.neighbors = self.graph.neighbors(node);
            self.next()
        }
    }
}

/// Iterator over the nodes of a graph in postorder traversal order.
pub struct PostOrderIterator<'graph> {
    /// Stack of nodes to visit
    stack: Vec<usize>,
    /// `true` if the node has been visited
    visited: Vec<bool>,
    /// Neighbors left to iterate for each node
    neighbors: Vec<NeighborIterator<'graph>>,
    /// `true` if the last node popped from the stack was a tail node
    tail: bool,
}

impl Iterator for PostOrderIterator<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let &u = self.stack.last()?;
            self.tail = true;
            for (v, _) in self.neighbors[u].by_ref() {
                if !self.visited[v] {
                    self.visited[v] = true;
                    self.stack.push(v);
                    self.tail = false;
                    break;
                }
            }
            if self.tail {
                return self.stack.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn test() {
        /* tree structure
            1
           / \
          2   3
         /   / \
        4   5   6
           / \
          7   8
         */
        let graph = Graph::from([(1, 3), (1, 2), (2, 4), (3, 6), (3, 5), (5, 8), (5, 7)]);

        assert_eq!(
            graph.pre_order(1).collect::<Vec<_>>(),
            [(2, 1), (4, 2), (3, 0), (5, 4), (7, 6), (8, 5), (6, 3)]
        );
        assert_eq!(
            graph.level_order(1).collect::<Vec<_>>(),
            [(2, 1), (3, 0), (4, 2), (5, 4), (6, 3), (7, 6), (8, 5)]
        );
        assert_eq!(
            graph.post_order(1).collect::<Vec<_>>(),
            [4, 2, 7, 8, 5, 6, 3, 1]
        );

        // <https://stackoverflow.com/q/36488968>
        let graph = Graph::from([
            (1, 4),
            (1, 2),
            (2, 5),
            (3, 6),
            (3, 5),
            (4, 2),
            (5, 6),
            (5, 4),
        ]);
        assert_eq!(
            graph.pre_order(1).collect::<Vec<_>>(),
            [(2, 1), (5, 2), (4, 7), (6, 6)]
        );
        assert_eq!(graph.post_order(1).collect::<Vec<_>>(), [4, 6, 5, 2, 1]);

        let graph = Graph::from([
            (1, 2),
            (2, 3),
            (2, 1),
            (4, 3),
            (4, 2),
            (5, 1),
            (6, 4),
            (6, 5),
        ]);
        assert_eq!(graph.post_order(6).collect::<Vec<_>>(), [3, 2, 1, 5, 4, 6]);
    }
}
