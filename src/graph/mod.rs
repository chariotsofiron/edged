//! A graph is a collection of vertices and edges.
//! <https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)>
// pub mod adjlist;
pub mod matrix;
pub mod graph_type;
pub mod util;


// pub trait Graph {
//     type Weight;

//     /// Returns the number of vertices in the graph.
//     fn len(&self) -> usize;

//     /// Returns an iterator over the children of a vertex.
//     fn children(&self, vertex: usize) -> Vec<usize>;

//     /// Returns an iterator over the parents of a vertex.
//     fn parents(&self, vertex: usize) -> Vec<usize>;

//     /// Outgoing edges of a vertex.
//     fn outgoing(&self, vertex: usize) -> Vec<(usize, Self::Weight)>;

//     /// Incoming edges of a vertex.
//     fn incoming(&self, vertex: usize) -> Vec<(usize, Self::Weight)>;

//     fn indegree(&self, vertex: usize) -> usize {
//         self.incoming(vertex).len()
//     }

//     fn outdegree(&self, vertex: usize) -> usize {
//         self.outgoing(vertex).len()
//     }

//     /// Returns the degree of a vertex.
//     fn degree(&self, vertex: usize) -> usize {
//         self.indegree(vertex) + self.outdegree(vertex)
//     }
// }
