
#[cfg(test)]
mod union_find_tests {
    use competitive_programming::data_structures::union_find::*;

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
            for j in i+1..N {
                assert!(uf.same(i, j));
            }
        }
    }

    #[test]
    fn test_unconnected_graph() {
        const N: usize = 32;

        let mut uf = DisjointSet::new(N);

        for k in 0..2 {
            for i in (k+2..N).step_by(2) {
                assert!(!uf.same(k, i));
                uf.unite(k, i);
            }
        }

        for i in 0..N {
            for j in i+1..N {
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
