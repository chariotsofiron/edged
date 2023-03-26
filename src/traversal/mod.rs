//! Traversal algorithms.
pub mod levelorder;
pub mod postorder;
pub mod preorder;
pub mod topological;

#[cfg(test)]
mod tests {
    use crate::{
        graph::{matrix::Graph, traits::Directed},
        traversal::{levelorder::LevelOrder, postorder::PostOrder},
    };

    use super::preorder::PreOrder;

    #[test]
    fn test_traversals() {
        /* tree structure
             1
            / \
           2   3
          /   / \
         4   5   6
            / \
           7   8
        */
        let graph =
            Graph::<_, Directed>::from([(1, 3), (1, 2), (2, 4), (3, 6), (3, 5), (5, 8), (5, 7)]);

        let order = PreOrder::new(&graph, 1).collect::<Vec<_>>();
        assert_eq!(order, [1, 3, 6, 5, 8, 7, 2, 4]);
        let order = LevelOrder::new(&graph, 1).collect::<Vec<_>>();
        assert_eq!(order, [1, 2, 3, 4, 5, 6, 7, 8]);
        let order = PostOrder::new(&graph, 1).collect::<Vec<_>>();
        assert_eq!(order, [6, 8, 7, 5, 3, 4, 2, 1]);

        // <https://stackoverflow.com/q/36488968>
        let graph = Graph::<_, Directed>::from([
            (1, 4),
            (1, 2),
            (2, 5),
            (3, 6),
            (3, 5),
            (4, 2),
            (5, 6),
            (5, 4),
        ]);

        let order = PreOrder::new(&graph, 1).collect::<Vec<_>>();
        assert_eq!(order, [1, 4, 2, 5, 6]);
        let order = LevelOrder::new(&graph, 1).collect::<Vec<_>>();
        assert_eq!(order, [1, 2, 4, 5, 6]);
        let order = PostOrder::new(&graph, 1).collect::<Vec<_>>();
        assert_eq!(order, [6, 5, 2, 4, 1]);

        // graph from figure 4 of dominance paper
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
        let order = PostOrder::new(&graph, 6).collect::<Vec<_>>();
        assert_eq!(order, [3, 2, 1, 5, 4, 6]);

        let graph = Graph::<(), Directed>::from([(2, 3), (2, 4), (4, 1), (1, 2)]);
        let order = PreOrder::new(&graph, 2).collect::<Vec<_>>();
        assert_eq!(order, vec![2, 4, 1, 3]);
        let order = LevelOrder::new(&graph, 2).collect::<Vec<_>>();
        assert_eq!(order, vec![2, 3, 4, 1]);
        let order = PostOrder::new(&graph, 2).collect::<Vec<_>>();
        assert_eq!(order, vec![1, 4, 3, 2]);
    }
}
