//! Postorder traversal

use crate::graph::{traits::Children, visit_map::VisitMap};
/// Post order traversal.
#[derive(Clone, Debug)]
pub struct PostOrder<G> {
    /// Reference to the graph
    graph: G,
    /// The stack of nodes to visit
    stack: Vec<usize>,
    /// The map of discovered nodes
    discovered: VisitMap,
    /// The map of finished nodes
    finished: VisitMap,
}

impl<G> PostOrder<G> {
    /// Create a new `PostOrder` iterator.
    pub fn new(graph: G, start: usize) -> Self {
        Self {
            graph,
            stack: vec![start],
            discovered: VisitMap::default(),
            finished: VisitMap::default(),
        }
    }
}

impl<G: Children> Iterator for PostOrder<G> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        while let Some(&node) = self.stack.last() {
            if self.discovered.visit(node) {
                for succ in self.graph.children(node) {
                    if !self.discovered.is_visited(succ) {
                        self.stack.push(succ);
                    }
                }
            } else {
                self.stack.pop();
                if self.finished.visit(node) {
                    // Second time: All reachable nodes must have been finished
                    return Some(node);
                }
            }
        }
        None
    }
}
