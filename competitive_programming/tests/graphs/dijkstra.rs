#[cfg(test)]
mod dijkstra_tests {
    use competitive_programming::graphs::dijkstra;

    fn add_edge(adj: &mut [Vec<(usize, i64)>], u: usize, v: usize, w: i64) {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    #[test]
    fn multiple_simple_path_tree_test() {
        const N: usize = 8;
        let mut adj = vec![vec![]; N];

        for i in 1..N {
            add_edge(&mut adj, i - 1, i, i as i64);
            add_edge(&mut adj, i - 1, i, 1_000_000_000);
        }

        let dist = dijkstra(&adj, 0);
        let mut total_dist = 0;

        assert_eq!(dist.len(), N);

        for (i, d) in dist.iter().enumerate() {
            total_dist += i as i64;
            assert_eq!(*d, Some(total_dist));
        }
    }

    #[test]
    fn complete_graph_test() {
        const N: usize = 8;
        const SOURCE: usize = 3;
        let mut adj = vec![vec![]; N];

        for i in 0..N {
            for j in (i + 1)..N {
                if i == SOURCE || j == SOURCE {
                    add_edge(&mut adj, i, j, 1);
                } else {
                    add_edge(&mut adj, i, j, 1_000_000_000);
                }
            }
        }

        let dist = dijkstra(&adj, SOURCE);

        assert_eq!(dist.len(), N);

        for (i, d) in dist.iter().enumerate() {
            if i == SOURCE {
                assert_eq!(*d, Some(0));
            } else {
                assert_eq!(*d, Some(1));
            }
        }
    }
}
