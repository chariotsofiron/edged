//! Level-order traversal.

use alloc::collections::VecDeque;

use crate::graph::{traits::Children, visit_map::VisitMap};
/// Level order traversal, aka breadth-first-search.
#[derive(Clone, Debug)]
pub struct LevelOrder<G> {
    /// Reference to the graph
    graph: G,
    /// The queue of nodes to visit
    queue: VecDeque<usize>,
    /// The map of discovered nodes
    discovered: VisitMap,
}

impl<G> LevelOrder<G> {
    /// Create a new `LevelOrder` iterator.
    pub fn new(graph: G, start: usize) -> Self {
        let mut discovered = VisitMap::default();
        discovered.visit(start);
        Self {
            graph,
            queue: VecDeque::from(vec![start]),
            discovered,
        }
    }
}

impl<G: Children> Iterator for LevelOrder<G> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let node = self.queue.pop_front()?;
        for succ in self.graph.children(node) {
            if self.discovered.visit(succ) {
                self.queue.push_back(succ);
            }
        }
        Some(node)
    }
}
