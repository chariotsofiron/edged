use edged::{
    graph::{
        matrix::Graph,
        traits::{Children, Directed},
    },
    traversal::preorder::PreOrder,
};

fn main() {
    let graph = Graph::<(), Directed>::from([(2, 3, ()), (2, 4, ()), (4, 1, ()), (1, 2, ())]);
    println!("{:?}", (&graph).children(2).collect::<Vec<_>>());

    let order = PreOrder::new(&graph, 2).collect::<Vec<_>>();
    assert_eq!(order, vec![2, 4, 1, 3]);
}
