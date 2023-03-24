//! Dijkstra algorithm.
// use std::collections::BinaryHeap;

// use crate::graph::traits::{Children, VertexCount};

// /// Dijkstra's algorithm.
// ///
// /// # Panics
// ///
// /// Panics if `weights.len() != self.edge_count()`.
// #[must_use]
// pub fn dijkstra<G>(graph: G, start: usize) -> Vec<u64>
// where
//     G: Children + VertexCount,
// {
//     let mut dist = vec![u64::max_value(); graph.vertex_count()];
//     let mut heap = BinaryHeap::new();

//     dist[start] = 0;
//     heap.push((Reverse(0), start));
//     while let Some((Reverse(dist_u), u)) = heap.pop() {
//         if dist[u] == dist_u {
//             for (v, e) in self.neighbors(u) {
//                 let alt_cost = dist_u.saturating_add(weights[e]);
//                 if alt_cost < dist[v] {
//                     dist[v] = alt_cost;
//                     heap.push((Reverse(alt_cost), v));
//                 }
//             }
//         }
//     }
//     dist
// }
