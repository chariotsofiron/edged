//! Preorder traversal

use crate::graph::{traits::Children, visit_map::VisitMap};

/// Preorder traversal.
#[derive(Clone, Debug)]
pub struct PreOrder<G> {
    /// Reference to the graph
    graph: G,
    /// The stack of nodes to visit
    stack: Vec<usize>,
    /// The map of discovered nodes
    discovered: VisitMap,
}

impl<G> PreOrder<G> {
    /// Create a new `PreOrder` iterator.
    pub fn new(graph: G, start: usize) -> Self {
        let mut discovered = VisitMap::default();
        discovered.visit(start);
        Self {
            graph,
            stack: vec![start],
            discovered,
        }
    }
}

impl<G: Children> Iterator for PreOrder<G> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let node = self.stack.pop()?;
        for succ in self.graph.children(node) {
            if self.discovered.visit(succ) {
                self.stack.push(succ);
            }
        }
        Some(node)
    }
}
