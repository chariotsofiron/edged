//! Dominance algorithms
//! <https://en.wikipedia.org/wiki/Dominator_(graph_theory)>
//! <https://www.cs.rice.edu/~keith/EMBED/dom.pdf>
//! <https://github.com/static-analysis-engineering/CodeHawk-Binary/blob/master/chb/app/Cfg.py>

use crate::graph::Graph;

/// Finds the nearest common dominator of two nodes.
/// Walks up the dominator tree from two different nodes until a common parent is reached.
fn nearest_common_dominator(
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

// /// Calculates the immediate dominators of a graph.
// ///
// /// This is an implementation of the engineered ["Simple, Fast Dominance
// /// Algorithm"][0] discovered by Cooper et al.
// ///
// /// This algorithm is **O(|V|^2)**, and therefore has slower theoretical running time
// /// than the Lengauer-Tarjan algorithm (which is **O(|E| log |V|)**. However,
// /// Cooper et al found it to be faster in practice on control flow graphs of up
// /// to ~30,000 vertices.
// ///
// /// [0]: http://www.cs.rice.edu/~keith/EMBED/dom.pdf
// #[must_use]
// pub fn dominators(graph: &Graph, start: usize) -> Vec<Option<usize>> {
//     let post_order = graph.post_order(start).collect::<Vec<_>>();
//     debug_assert!(post_order.last() == Some(&start));
//     debug_assert!(post_order.len() > 0);

//     // Maps a node to its index into `post_order`.
//     let mut ordering = vec![0; graph.len()];
//     for (i, &b) in post_order.iter().enumerate() {
//         ordering[b] = i;
//     }
//     // used for getting predecessors
//     let transpose = graph.transpose();

//     // Initialize the dominators array
//     let mut idoms: Vec<Option<usize>> = vec![None; graph.len()];

//     println!("post_order: {:?}", post_order);

//     idoms[start] = Some(start);
//     let mut changed = true;
//     while changed {
//         changed = false;
//         // Iterate over the nodes in reverse postorder (except for the start node)
//         for &b in post_order.iter().rev() {
//             if b == start {
//                 continue;
//             }
//             let new_idom_idx = {
//                 let mut predecessors = transpose
//                     .neighbors(b)
//                     .filter_map(|(pred, _)| (idoms[pred].is_some()).then(|| pred));
//                 let new_idom_idx = predecessors.next().expect(
//                     "Because the root is initialized to dominate itself, and is the \
//                      first node in every path, there must exist a predecessor to this \
//                      node that also has a dominator",
//                 );
//                 predecessors.fold(new_idom_idx, |new_idom_idx, predecessor_idx| {
//                     intersect(&idoms, new_idom_idx, predecessor_idx)
//                 })
//             };

//             if Some(new_idom_idx) != idoms[b] {
//                 idoms[b] = Some(new_idom_idx);
//                 changed = true;
//             }
//         }
//     }

//     idoms
// }

/// Calculates the immediate dominators of a graph.
#[must_use]
pub fn dominators(graph: &Graph, start: usize) -> Vec<usize> {
    let order = graph.post_order(start).collect::<Vec<_>>();
    debug_assert!(order.last() == Some(&start));
    debug_assert!(!order.is_empty());

    // Maps a node to its index into `post_order`.
    let mut postorder = vec![0; graph.len()];
    for (i, &b) in order.iter().enumerate() {
        postorder[b] = i;
    }
    let transpose = graph.transpose(); // used for getting predecessors
    let mut idoms: Vec<usize> = vec![usize::MAX; graph.len()];

    idoms[start] = start;
    let mut changed = true;
    while changed {
        changed = false;

        for &b in order.iter().rev() {
            if b == start {
                continue;
            }
            let mut new_idom = usize::MAX;

            for (predecessor, _) in transpose.neighbors(b) {
                if idoms[predecessor] == usize::MAX {
                    continue;
                }
                if new_idom == usize::MAX {
                    new_idom = predecessor;
                } else {
                    new_idom = nearest_common_dominator(&idoms, &postorder, predecessor, new_idom);
                }
            }
            debug_assert!(new_idom != usize::MAX);
            if idoms[b] != new_idom {
                idoms[b] = new_idom;
                changed = true;
            }
        }
    }

    idoms
}

#[cfg(test)]
mod tests {
    use crate::{dominance::dominators, graph::Graph};

    #[test]
    fn test_dominators() {
        // from cooper et al paper <https://www.cs.rice.edu/~keith/EMBED/dom.pdf>
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

        let idoms = dominators(&graph, 6);
        assert_eq!(idoms, vec![usize::MAX, 6, 6, 6, 6, 6, 6]);

        // from wikipedia <https://en.wikipedia.org/wiki/Dominator_(graph_theory)>
        let graph = Graph::from([(1, 2), (2, 3), (2, 4), (2, 6), (3, 5), (4, 5), (5, 2)]);
        let idoms = dominators(&graph, 1);
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
        let idoms = dominators(&graph, 1);
        assert_eq!(idoms, vec![usize::MAX, 1, 1, 2, 2, 3, 2, 6]);
    }
}
