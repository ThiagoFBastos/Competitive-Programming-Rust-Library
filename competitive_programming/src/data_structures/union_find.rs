/*
  Simple Disjoint Set/Union Find with rank and path compression
*/

#[derive(Clone)]
pub struct DisjointSet {
    parent: Vec<usize>, // the parent of each vertex
    rank: Vec<u32>,     // the rank of the disjoint set
}

impl DisjointSet {
    /**
     * create a new instance of DisjointSet
     * @param n number of vertexes
     * @return a DisjointSet
     */
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    /**
     * find the root of disjoint set that u belongs
     * @param u a vertex of the disjoint set that you want to find the root
     * @return the root of disjoint set
     */
    pub fn find_set(&mut self, u: usize) -> usize {
        if u == self.parent[u] {
            return u;
        }

        self.parent[u] = self.find_set(self.parent[u]);

        self.parent[u]
    }

    /**
     * join the connected components that u and v belongs
     * @param u a vertex of one connected component
     * @param v a vertex of other connected component
     */
    pub fn unite(&mut self, mut u: usize, mut v: usize) {
        u = self.find_set(u);
        v = self.find_set(v);

        if u == v {
            return;
        } else if self.rank[u] > self.rank[v] {
            std::mem::swap(&mut u, &mut v);
        }

        self.parent[u] = v;
        self.rank[v] = if self.rank[u] == self.rank[v] {
            self.rank[v] + 1
        } else {
            self.rank[v]
        };
    }

    /**
     * Return if two vertexes belongs to the same connected component
     * @param u a vertex of one connected component
     * @param v a vertex of one connected component
     * @return true if u and v belongs to the same connected component and false otherwise
     */
    pub fn same(&mut self, u: usize, v: usize) -> bool {
        self.find_set(u) == self.find_set(v)
    }
}

#[cfg(test)]
mod union_find_tests {
    use super::*;

    #[test]
    fn test_tree_star() {
        const N: usize = 32;

        let mut uf = DisjointSet::new(N);

        for i in 1..N {
            assert!(!uf.same(0, i));
            uf.unite(0, i);
            assert!(uf.same(0, i));
        }
    }

    #[test]
    fn test_complete_graph() {
        const N: usize = 32;

        let mut uf = DisjointSet::new(N);

        for i in 1..N {
            assert!(!uf.same(0, i));
            uf.unite(0, i);
        }

        for i in 0..N {
            for j in i + 1..N {
                assert!(uf.same(i, j));
            }
        }
    }

    #[test]
    fn test_unconnected_graph() {
        const N: usize = 32;

        let mut uf = DisjointSet::new(N);

        for k in 0..2 {
            for i in (k + 2..N).step_by(2) {
                assert!(!uf.same(k, i));
                uf.unite(k, i);
            }
        }

        for i in 0..N {
            for j in i + 1..N {
                let same_parity = i % 2 == j % 2;
                assert_eq!(uf.same(i, j), same_parity);
            }
        }
    }

    #[test]
    fn test_independent_set() {
        const N: usize = 32;

        let mut uf = DisjointSet::new(N);

        for i in 0..N {
            for j in 0..N {
                assert_eq!(uf.same(i, j), i == j);
            }
        }
    }
}
