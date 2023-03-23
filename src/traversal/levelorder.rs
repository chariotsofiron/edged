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

impl<'graph, G> LevelOrder<&'graph G> {
    /// Create a new `LevelOrder` iterator.
    pub fn new(graph: &'graph G, start: usize) -> Self
    where
        &'graph G: Children,
    {
        Self {
            graph,
            queue: VecDeque::from(vec![start]),
            discovered: VisitMap::default(),
        }
    }
}

impl<G> Iterator for LevelOrder<G>
where
    G: Children,
{
    type Item = usize;

    fn next(&mut self) -> Option<usize>
    where
        G: Children,
    {
        let node = self.queue.pop_front()?;
        for succ in self.graph.children(node) {
            if self.discovered.visit(succ) {
                self.queue.push_back(succ);
            }
        }
        Some(node)
    }
}
