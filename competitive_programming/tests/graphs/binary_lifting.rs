#[cfg(test)]
mod lca_tests {
    use competitive_programming::graphs::LCA;

    #[test]
    fn simple_path_test() {
        const N: usize = 10;

        let mut lca = LCA::new(N);

        for i in 1..N {
            lca.add_edge(i - 1, i, 1);
        }

        lca.build(0);

        for u in 0..N {
            for v in 0..N {
                let distance = lca.distance(u, v);
                let vertex = lca.query(u, v);
                let expected_distance = u.abs_diff(v);
                let expected_vertex = u.min(v);

                assert_eq!(distance, expected_distance as i64);
                assert_eq!(vertex, expected_vertex);
            }
        }
    }

    #[test]
    fn star_test() {
        const N: usize = 10;

        let mut lca = LCA::new(N);

        for i in 1..N {
            lca.add_edge(0, i, 1);
        }

        lca.build(0);

        for i in 1..N {
            let distance = lca.distance(0, i);
            let expected_distance = 1;

            assert_eq!(distance, expected_distance);
            assert_eq!(lca.query(0, i), 0);
        }

        for u in 1..N {
            for v in 1..N {
                if u == v {
                    continue;
                }

                let distance = lca.distance(u, v);
                let expected_distance = 2;

                assert_eq!(distance, expected_distance);
                assert_eq!(lca.query(u, v), 0);
            }
        }
    }

    #[test]
    #[allow(unused_assignments)]
    fn double_star_test() {
        const N: usize = 10;

        let mut lca = LCA::new(N);

        lca.add_edge(0, 1, 7);

        for i in 2..N {
            lca.add_edge(i % 2, i, 7);
        }

        lca.build(0);

        for u in 0..2 {
            for v in 0..N {
                if u == v {
                    assert_eq!(lca.distance(u, v), 0);
                    assert_eq!(lca.query(u, v), u);
                } else if u == v ^ 1 {
                    assert_eq!(lca.distance(u, v), 7);
                    assert_eq!(lca.query(u, v), 0);
                } else if u % 2 == v % 2 {
                    assert_eq!(lca.distance(u, v), 7);
                    assert_eq!(lca.query(u, v), u);
                } else {
                    assert_eq!(lca.distance(u, v), 14);
                    assert_eq!(lca.query(u, v), 0);
                }
            }
        }

        for u in 3..N {
            for v in (u + 1)..N {
                let distance = lca.distance(u, v);
                let vertex = lca.query(u, v);

                let mut expected_distance = 0;
                let mut expected_vertex = 0;

                if u % 2 == v % 2 {
                    expected_distance = 14;
                    expected_vertex = u % 2;
                } else {
                    expected_distance = 21;
                }

                assert_eq!(distance, expected_distance);
                assert_eq!(vertex, expected_vertex);
            }
        }
    }
}
