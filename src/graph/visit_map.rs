//! A simple data structure to keep track of visited nodes.
use crate::util::ensure_len;

/// A map of visited nodes.
#[derive(Default, Clone, Debug)]
pub struct VisitMap {
    /// The map of discovered nodes
    discovered: Vec<bool>,
}

impl VisitMap {
    /// Creates a new `VisitMap` with the given capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            discovered: Vec::with_capacity(capacity),
        }
    }

    /// Returns `true` if this is the first visit to this node and
    /// marks it as visited.
    #[must_use]
    pub fn visit(&mut self, node: usize) -> bool {
        ensure_len(&mut self.discovered, node.wrapping_add(1));
        if self.discovered[node] {
            false
        } else {
            self.discovered[node] = true;
            true
        }
    }
    /// Returns `true` if this node has been visited.
    /// Returns `false` even for invalid nodes.
    #[must_use]
    pub fn is_visited(&self, node: usize) -> bool {
        self.discovered.get(node).map_or(false, |&x| x)
    }
}
