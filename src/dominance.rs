//! Dominance algorithms
//!
//! <https://en.wikipedia.org/wiki/Dominator_(graph_theory)>
//! <https://pages.cs.wisc.edu/~fischer/cs701.f08/lectures/Lecture19.4up.pdf>
//! <https://www.cs.rice.edu/~keith/EMBED/dom.pdf>
//!
//!
//! Node M dominates N iff to get to N from the start node, you must pass through M.
//! N dom N is always true.
//!
//! M strictly dominates N if M dom N and M != N.
//!
//! A node can have multiple dominators. The immediate dominator of N is the dominator
//! that strictly dominates N but does not dominate any other dominators of N.

use crate::{
    graph::traits::{Children, NodeCount, Parents},
    traversal::postorder::PostOrder,
};

/// Finds the nearest common dominator of two nodes.
/// Walks up the dominator tree from two different nodes until a common parent is reached.
#[allow(clippy::expect_used)]
fn nearest_common_dominator(
    dominators: &[Option<usize>],
    postorder: &[usize],
    mut finger1: usize,
    mut finger2: usize,
) -> usize {
    while finger1 != finger2 {
        while postorder[finger1] < postorder[finger2] {
            finger1 = dominators[finger1].expect("Shouldn't happen");
        }
        while postorder[finger2] < postorder[finger1] {
            finger2 = dominators[finger2].expect("Shouldn't happen");
        }
    }
    finger1
}

/// Returns the immediate dominators of all nodes of a `Graph`.
///
/// Except for `start`, the immediate dominators are the parents of their
/// corresponding nodes in the dominator tree.
#[must_use]
pub fn immediate_dominators<G>(graph: G, start: usize) -> Vec<Option<usize>>
where
    G: Children + Parents + NodeCount,
{
    let mut order = PostOrder::new(graph, start).collect::<Vec<_>>();

    // Maps a node to its index in a postorder traversal
    let mut postorder_idx = vec![0; graph.node_count()];
    for (i, &node) in order.iter().enumerate() {
        postorder_idx[node] = i;
    }
    order.pop(); // remove the start node
    order.reverse(); // reverse the postorder traversal

    let mut dominators: Vec<Option<usize>> = vec![None; graph.node_count()];
    dominators[start] = Some(start);
    let mut changed = true;
    while changed {
        changed = false;
        for &node in &order {
            let new_idom = graph
                .parents(node)
                .filter(|&predecessor| dominators[predecessor].is_some())
                .reduce(|finger1, finger2| {
                    nearest_common_dominator(&dominators, &postorder_idx, finger1, finger2)
                });
            // The root is initialized to dominate itself, and is the first node in
            // every path so there must exist a predecessor to this node that also
            // has a dominator.
            debug_assert!(new_idom.is_some());
            if dominators[node] != new_idom {
                dominators[node] = new_idom;
                changed = true;
            }
        }
    }
    dominators
}

/// Returns the dominance frontiers of all nodes of a directed graph.
///
/// The dominance frontier of a node `b` is the set of all nodes `y`
/// such that `b` dominates a predecessor of `y` but does not strictly
/// dominate `y`.
#[allow(clippy::expect_used)]
pub fn frontiers<G>(graph: G, start: usize) -> Vec<Vec<usize>>
where
    G: Children + Parents + NodeCount,
{
    // K. D. Cooper, T. J. Harvey, and K. Kennedy.
    // A Simple, Fast Dominance Algorithm.
    // Software Practice & Experience, 4:110, 2001.
    // <https://www.cs.rice.edu/~keith/EMBED/dom.pdf>
    let mut frontiers = vec![Vec::new(); graph.node_count()];
    let idoms = immediate_dominators(graph, start);
    for node in 0..graph.node_count() {
        let predecessors = graph.parents(node).collect::<Vec<_>>();
        if predecessors.len() >= 2 {
            for &predecessor in &predecessors {
                let mut finger = predecessor;
                while Some(finger) != idoms[node] {
                    frontiers[finger].push(node);
                    finger = idoms[finger].expect("Shouldn't happen");
                }
            }
        }
    }
    frontiers
}

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
        assert_eq!(
            idoms,
            vec![None, Some(6), Some(6), Some(6), Some(6), Some(6), Some(6)]
        );

        // from wikipedia <https://en.wikipedia.org/wiki/Dominator_(graph_theory)>
        let graph =
            Graph::<_, Directed>::from([(1, 2), (2, 3), (2, 4), (2, 6), (3, 5), (4, 5), (5, 2)]);
        let idoms = immediate_dominators(&graph, 1);
        assert_eq!(
            idoms,
            vec![None, Some(1), Some(1), Some(2), Some(2), Some(2), Some(2)]
        );

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
        assert_eq!(
            idoms,
            vec![
                None,
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(2),
                Some(6)
            ]
        );

        let graph = Graph::<_, Directed>::from([(1, 2), (1, 3), (2, 5), (3, 4), (4, 5)]);
        let idoms = immediate_dominators(&graph, 1);
        assert_eq!(
            idoms,
            vec![None, Some(1), Some(1), Some(1), Some(3), Some(1)]
        );
    }

    #[test]
    fn test_frontiers() {
        // https://pages.cs.wisc.edu/~fischer/cs701.f05/lectures/Lecture22.pdf
        let graph =
            Graph::<_, Directed>::from([(0, 1), (1, 2), (1, 3), (2, 4), (3, 4), (4, 5), (0, 5)]);
        let frontier = frontiers(&graph, 0);
        assert_eq!(
            frontier,
            vec![vec![], vec![5], vec![4], vec![4], vec![5], vec![],]
        );
    }
}
