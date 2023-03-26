//! Topological traversal
//! <https://en.wikipedia.org/wiki/Topological_sorting>

use crate::graph::traits::{Children, NodeCount};

/// Topological traversal.
/// Works for directed, acyclic graphs. Uses Kahn's algorithm.
/// Time complexity: O(|V| + |E|)
/// Space complexity: O(|V|)
#[derive(Clone, Debug)]
pub struct Topological<G> {
    /// Reference to the graph
    graph: G,
    /// The in-degree of each node
    in_degree: Vec<usize>,
    /// The stack of nodes with no parents
    stack: Vec<usize>,
}

impl<G> Topological<G>
where
    G: NodeCount + Children,
{
    /// Create a new `Topological` iterator.
    pub fn new(graph: G) -> Self {
        let mut in_degree = vec![0; graph.node_count()];
        for node in 0..graph.node_count() {
            for child in graph.children(node) {
                in_degree[child] += 1;
            }
        }
        let stack = in_degree
            .iter()
            .enumerate()
            .filter(|&(_, &degree)| degree == 0)
            .map(|(i, _)| i)
            .collect();

        Self {
            graph,
            in_degree,
            stack,
        }
    }
}

impl<G: Children> Iterator for Topological<G> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let node = self.stack.pop()?;
        for child in self.graph.children(node) {
            self.in_degree[child] -= 1;
            if self.in_degree[child] == 0 {
                self.stack.push(child);
            }
        }
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        graph::{matrix::Graph, traits::Directed},
        traversal::topological::Topological,
    };

    #[test]
    fn test_topo() {
        let graph =
            Graph::<_, Directed>::from([(0, 1), (1, 2), (0, 3), (3, 1), (3, 5), (3, 4), (4, 5)]);
        let order = Topological::new(&graph).collect::<Vec<_>>();
        assert_eq!(order, [0, 3, 4, 5, 1, 2]);
    }
}
