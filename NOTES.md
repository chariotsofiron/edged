
# Adjacency list

- Easy to associate with weights / other metadata efficiently
- Append only


# Adjacency matrix


- More efficient / elegant for a handful of algorithms, get to use linear algebra
- Difficult to associate nodes/edges with data
- Iterating edges can be O(n_edges) using trailing zero intrinsics
- Can be modified
- Adding new nodes can be slow... (can be amortized)


Don't add node data for now...
Can always pass in an associated array



## Links

**Dominance**
- https://en.wikipedia.org/wiki/Dominator_(graph_theory)
- https://www.cs.rice.edu/~keith/EMBED/dom.pdf
- https://github.com/static-analysis-engineering/CodeHawk-Binary/blob/master/chb/app/Cfg.py

