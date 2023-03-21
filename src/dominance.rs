//! Dominance algorithms
//! <https://en.wikipedia.org/wiki/Dominator_(graph_theory)>
//! <https://www.cs.rice.edu/~keith/EMBED/dom.pdf>
//! <https://github.com/static-analysis-engineering/CodeHawk-Binary/blob/master/chb/app/Cfg.py>

use crate::graph::adjlist::Graph;

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
pub fn immediate_dominators(graph: &Graph, start: usize) -> Vec<usize> {
    // handy references, but they seem to have bugs...
    // <https://baziotis.cs.illinois.edu/compilers/visualizing-dominators.html>
    // <https://github.com/petgraph/petgraph/blob/master/src/algo/dominators.rs>
    // <https://github.com/alexcrichton/l4c/blob/master/src/middle/ssa.rs>
    // <http://www.cs.rice.edu/~keith/EMBED/dom.pdf>
    let mut order = graph.post_order(start).collect::<Vec<_>>();
    debug_assert!(order.last() == Some(&start));

    // Maps a node to its index in a postorder traversal
    let mut postorder_idx = vec![0; graph.len()];
    for (i, &b) in order.iter().enumerate() {
        postorder_idx[b] = i;
    }
    order.pop(); // remove the start node
    order.reverse(); // reverse the postorder traversal

    let transpose = graph.transpose(); // used for getting predecessors
    let mut dominators: Vec<usize> = vec![usize::MAX; graph.len()];
    dominators[start] = start;
    let mut changed = true;
    while changed {
        changed = false;
        for &node in &order {
            let predecessors = transpose.neighbors(node).filter_map(|(predecessor, _)| {
                (dominators[predecessor] != usize::MAX).then_some(predecessor)
            });
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

// /// Returns the dominance frontiers of all nodes of a directed graph.
// pub fn dominance_frontiers(graph: &Graph, start: usize) -> Vec<Vec<usize>> {
//     let mut frontiers = vec![Vec::new(); graph.len()];
//     let idoms = immediate_dominators(graph, start);
//     let transpose = graph.transpose(); // used for getting predecessors

//     for &node in &idoms {
//         if node == usize::MAX {
//             continue;
//         }
//         let predecessors = transpose.neighbors(node).map(|(predecessor, _)| predecessor).collect::<Vec<_>>();
//         if predecessors.len() >= 2 {
//             for &predecessor in &predecessors {
//                 let mut finger = predecessor;
//                 while finger != node {
//                     frontiers[finger].push(node);
//                     finger = idoms[finger];
//                 }
//             }
//         }
//     }
//     frontiers
// }

#[cfg(test)]
mod tests {
    use crate::{dominance::immediate_dominators, graph::adjlist::Graph};

    #[test]
    fn test_dominators() {
        // from cooper et al paper
        let graph = Graph::from([
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
        let graph = Graph::from([(1, 2), (2, 3), (2, 4), (2, 6), (3, 5), (4, 5), (5, 2)]);
        let idoms = immediate_dominators(&graph, 1);
        // TODO, this is wrong
        assert_eq!(idoms, vec![usize::MAX, 1, 1, 2, 2, 2, 2]);

        let graph = Graph::from([
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
        // let frontier = dominance_frontiers(&graph, 1);
        // assert_eq!(
        //     frontier,
        //     vec![
        //         Vec::new(),
        //         Vec::new(),
        //         vec![1],
        //         vec![2],
        //         vec![2],
        //         vec![3],
        //         vec![2],
        //         vec![6]
        //     ]
        // );
    }
}
