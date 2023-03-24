//! Dominance algorithms

use crate::{
    graph::traits::{Children, Parents, VertexCount},
    traversal::postorder::PostOrder,
};

/// Finds the nearest common dominator of two nodes.
/// Walks up the dominator tree from two different nodes until a common parent is reached.
const fn nearest_common_dominator(
    dominators: &[usize],
    postorder: &[usize],
    mut finger1: usize,
    mut finger2: usize,
) -> usize {
    while finger1 != finger2 {
        while postorder[finger1] < postorder[finger2] {
            finger1 = dominators[finger1];
        }
        while postorder[finger2] < postorder[finger1] {
            finger2 = dominators[finger2];
        }
    }
    finger1
}

/// Returns the immediate dominators of all nodes of a `Graph`.
///
/// Except for `start`, the immediate dominators are the parents of their
/// corresponding nodes in the dominator tree.
#[must_use]
pub fn immediate_dominators<G>(graph: G, start: usize) -> Vec<usize>
where
    G: Children + Parents + VertexCount,
{
    let mut order = PostOrder::new(graph, start).collect::<Vec<_>>();
    debug_assert!(order.last() == Some(&start));

    // Maps a node to its index in a postorder traversal
    let mut postorder_idx = vec![0; graph.vertex_count()];
    for (i, &node) in order.iter().enumerate() {
        postorder_idx[node] = i;
    }
    order.pop(); // remove the start node
    order.reverse(); // reverse the postorder traversal

    let mut dominators: Vec<usize> = vec![usize::MAX; graph.vertex_count()];
    dominators[start] = start;
    let mut changed = true;
    while changed {
        changed = false;
        for &node in &order {
            let predecessors = graph
                .parents(node)
                .filter(|&predecessor| dominators[predecessor] != usize::MAX);
            #[allow(clippy::expect_used)]
            let new_idom = predecessors
                .reduce(|finger1, finger2| {
                    nearest_common_dominator(&dominators, &postorder_idx, finger1, finger2)
                })
                .expect(
                    "the root is initialized to dominate itself, and is the first \
                    node in every path so there must exist a predecessor to this node that \
                    also has a dominator",
                );

            debug_assert!(new_idom != usize::MAX);
            if dominators[node] != new_idom {
                dominators[node] = new_idom;
                changed = true;
            }
        }
    }
    dominators
}

/// Returns the dominance frontiers of all nodes of a directed graph.
pub fn frontiers<G>(graph: G, start: usize) -> Vec<Vec<usize>>
where
    G: Children + Parents + VertexCount,
{
    let mut frontiers = vec![Vec::new(); graph.vertex_count()];
    let idoms = immediate_dominators(graph, start);

    for &node in &idoms {
        let predecessors = graph.parents(node).collect::<Vec<_>>();
        if predecessors.len() >= 2 {
            for &predecessor in &predecessors {
                if predecessor == usize::MAX {
                    continue;
                }
                let mut finger = predecessor;
                while finger != idoms[node] {
                    frontiers[finger].push(node);
                    finger = idoms[finger];
                }
            }
        }
    }
    frontiers
}

// fn dominance_frontier(&self, start: usize) -> Vec<Vec<usize>> {
//     let idoms = self.compute_dominator_tree(start);
//     let mut dom_frontiers: Vec<Vec<usize>> = vec![Vec::new(); self.nodes.len()];
//     for node in 0..self.nodes.len() {
//         let allpreds = self.get_predecessors(node);
//         if allpreds.len() >= 2 {
//             for &pred in &allpreds {
//                 let mut runner = pred;
//                 while runner != idoms[node] {
//                     dom_frontiers[runner].push(node);
//                     runner = idoms[runner];
//                 }
//             }
//         }
//     }
//     dom_frontiers
// }

#[cfg(test)]
mod tests {
    use crate::{
        dominance::{frontiers, immediate_dominators},
        graph::{matrix::Graph, traits::Directed},
    };

    #[test]
    fn test_dominators() {
        // from cooper et al paper
        let graph = Graph::<_, Directed>::from([
            (6, 5),
            (6, 4),
            (5, 1),
            (4, 2),
            (4, 3),
            (1, 2),
            (2, 3),
            (2, 1),
            (3, 2),
        ]);

        let idoms = immediate_dominators(&graph, 6);
        assert_eq!(idoms, vec![usize::MAX, 6, 6, 6, 6, 6, 6]);

        // from wikipedia <https://en.wikipedia.org/wiki/Dominator_(graph_theory)>
        let graph =
            Graph::<_, Directed>::from([(1, 2), (2, 3), (2, 4), (2, 6), (3, 5), (4, 5), (5, 2)]);
        let idoms = immediate_dominators(&graph, 1);
        // TODO, this is wrong
        assert_eq!(idoms, vec![usize::MAX, 1, 1, 2, 2, 2, 2]);

        let graph = Graph::<_, Directed>::from([
            (1, 2),
            (2, 3),
            (2, 4),
            (3, 5),
            (4, 6),
            (5, 3),
            (5, 6),
            (6, 2),
            (6, 7),
        ]);
        let idoms = immediate_dominators(&graph, 1);
        assert_eq!(idoms, vec![usize::MAX, 1, 1, 2, 2, 3, 2, 6]);

        let graph = Graph::<_, Directed>::from([(1, 2), (1, 3), (2, 5), (3, 4), (4, 5)]);
        let idoms = immediate_dominators(&graph, 1);
        assert_eq!(idoms, vec![usize::MAX, 1, 1, 1, 3, 1]);
    }

    #[test]
    fn test_frontiers() {
        let graph =
            Graph::<_, Directed>::from([(0, 1), (1, 2), (1, 3), (2, 4), (3, 4), (4, 5), (0, 5)]);
        let idoms = immediate_dominators(&graph, 0);
        assert_eq!(idoms, vec![0, 0, 1, 1, 1, 0]);
        let frontier = frontiers(&graph, 0);
        assert_eq!(
            frontier,
            vec![
                Vec::new(),
                Vec::new(),
                vec![1],
                vec![2],
                vec![2],
                vec![3],
                vec![2],
                vec![6]
            ]
        );
    }
}
