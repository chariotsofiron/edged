use edged::graph::{
    matrix::Graph,
    traits::{Directed, Undirected},
    util::extend_linearized_matrix,
};

fn main() {
    let graph = Graph::<(), Directed>::from([(2, 3, ()), (2, 4, ()), (4, 1, ()), (1, 2, ())]);
    println!("{:?}", graph.children(2).collect::<Vec<_>>());
}
