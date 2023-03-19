//! Dijkstra's algorithm
use alloc::collections::BinaryHeap;
use core::cmp::Reverse;

use crate::graph::Graph;

impl Graph {
    /// Dijkstra's algorithm.
    ///
    /// # Panics
    ///
    /// Panics if `weights.len() != self.edge_count()`.
    #[must_use]
    pub fn dijkstra(&self, weights: &[u64], start: usize) -> Vec<u64> {
        assert_eq!(self.edge_count(), weights.len());
        let mut dist = vec![u64::max_value(); self.len()];
        let mut heap = BinaryHeap::new();

        dist[start] = 0;
        heap.push((Reverse(0), start));
        while let Some((Reverse(dist_u), u)) = heap.pop() {
            if dist[u] == dist_u {
                for (v, e) in self.neighbors(u) {
                    let alt_cost = dist_u.saturating_add(weights[e]);
                    if alt_cost < dist[v] {
                        dist[v] = alt_cost;
                        heap.push((Reverse(alt_cost), v));
                    }
                }
            }
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = Graph::from([(0, 1), (1, 2), (2, 0)]);
        let weights = [7, 3, 5];

        let dist = graph.dijkstra(&weights, 0);
        assert_eq!(dist, vec![0, 7, 10]);

        let graph = Graph::from([
            (1, 2),
            (1, 3),
            (1, 5),
            (2, 4),
            (2, 5),
            (3, 2),
            (4, 1),
            (4, 3),
            (5, 4),
        ]);
        let weights = [3, 8, 2, 1, 7, 4, 2, 2, 6];
        let dist = graph.dijkstra(&weights, 1);
        assert_eq!(dist, vec![u64::max_value(), 0, 3, 6, 4, 2]);
    }
}
