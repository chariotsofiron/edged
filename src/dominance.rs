//! Dominance algorithms
//! <https://en.wikipedia.org/wiki/Dominator_(graph_theory)>
//! <https://www.cs.rice.edu/~keith/EMBED/dom.pdf>
//! <https://github.com/static-analysis-engineering/CodeHawk-Binary/blob/master/chb/app/Cfg.py>

use std::cmp::Ordering;

use crate::graph::Graph;

/// Finds the nearest common dominator of two nodes.
fn intersect(dominators: &[Option<usize>], mut finger1: usize, mut finger2: usize) -> usize {
    loop {
        match finger1.cmp(&finger2) {
            Ordering::Less => finger1 = dominators[finger1].unwrap(),
            Ordering::Greater => finger2 = dominators[finger2].unwrap(),
            Ordering::Equal => return finger1,
        }
    }
}

/// This is an implementation of the engineered ["Simple, Fast Dominance
/// Algorithm"][0] discovered by Cooper et al.
///
/// This algorithm is **O(|V|^2)**, and therefore has slower theoretical running time
/// than the Lengauer-Tarjan algorithm (which is **O(|E| log |V|)**. However,
/// Cooper et al found it to be faster in practice on control flow graphs of up
/// to ~30,000 vertices.
///
/// [0]: http://www.cs.rice.edu/~keith/EMBED/dom.pdf
#[must_use]
pub fn dominators(graph: &Graph, start: usize) -> Vec<Option<usize>> {
    let post_order = graph.post_order(start).collect::<Vec<_>>();
    debug_assert!(post_order.last() == Some(&start));
    debug_assert!(post_order.len() > 0);

    // Maps a node to its index into `post_order`.
    let mut ordering = vec![0; graph.len()];
    for (i, &b) in post_order.iter().enumerate() {
        ordering[b] = i;
    }
    // used for getting predecessors
    let transpose = graph.transpose();

    // Initialize the dominators array
    let mut idoms: Vec<Option<usize>> = vec![None; graph.len()];

    idoms[start] = Some(start);
    let mut changed = true;
    while changed {
        changed = false;
        // Iterate over the nodes in reverse postorder (except for the start node)
        for &b in post_order.iter().rev() {
            if b == start {
                continue;
            }
            let new_idom_idx = {
                let mut predecessors = transpose
                    .neighbors(b)
                    .filter_map(|(pred, _)| (idoms[pred].is_some()).then(|| pred));
                let new_idom_idx = predecessors.next().expect(
                    "Because the root is initialized to dominate itself, and is the \
                     first node in every path, there must exist a predecessor to this \
                     node that also has a dominator",
                );
                predecessors.fold(new_idom_idx, |new_idom_idx, predecessor_idx| {
                    intersect(&idoms, new_idom_idx, predecessor_idx)
                })
            };

            if Some(new_idom_idx) != idoms[b] {
                idoms[b] = Some(new_idom_idx);
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
        assert_eq!(
            idoms,
            vec![None, Some(6), Some(6), Some(6), Some(6), Some(6), Some(6)]
        );
    }
}
