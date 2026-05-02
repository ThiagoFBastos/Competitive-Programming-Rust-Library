#[cfg(test)]
mod tree_diameter_tests {
    use competitive_programming::graphs::{tree_center, tree_diameter};

    fn add_edge(adj: &mut [Vec<usize>], u: usize, v: usize) {
        adj[u].push(v);
        adj[v].push(u);
    }

    #[test]
    fn test_small_tree() {
        const N: usize = 5;

        let expected_diameter = 3;
        let expected_center = vec![0, 2];

        let mut adj = vec![vec![]; N];

        add_edge(&mut adj, 0, 1);
        add_edge(&mut adj, 0, 2);
        add_edge(&mut adj, 2, 3);
        add_edge(&mut adj, 2, 4);

        let diam = tree_diameter(&adj);
        let mut center = tree_center(&adj);

        center.sort();

        assert_eq!(diam, expected_diameter);
        assert_eq!(center, expected_center);
    }

    #[test]
    fn test_line_tree() {
        const N: usize = 3;

        let expected_diameter = 2;
        let expected_center = vec![0];

        let mut adj = vec![vec![]; N];

        add_edge(&mut adj, 0, 1);
        add_edge(&mut adj, 0, 2);

        let diam = tree_diameter(&adj);
        let center = tree_center(&adj);

        assert_eq!(diam, expected_diameter);
        assert_eq!(center, expected_center);
    }

    #[test]
    fn test_trivial_tree() {
        const N: usize = 1;

        let expected_diameter = 0;
        let expected_center = vec![0];

        let adj: Vec<Vec<usize>> = vec![vec![]; N];

        let diam = tree_diameter(&adj);
        let center = tree_center(&adj);

        assert_eq!(diam, expected_diameter);
        assert_eq!(center, expected_center);
    }

    #[test]
    fn test_large_tree() {
        const N: usize = 10;

        let expected_diameter = 6;
        let expected_center = vec![4];

        let mut adj = vec![vec![]; N];

        add_edge(&mut adj, 3, 5);
        add_edge(&mut adj, 0, 2);
        add_edge(&mut adj, 9, 7);
        add_edge(&mut adj, 8, 2);
        add_edge(&mut adj, 1, 6);
        add_edge(&mut adj, 4, 3);
        add_edge(&mut adj, 1, 3);
        add_edge(&mut adj, 7, 4);
        add_edge(&mut adj, 8, 4);

        let diam = tree_diameter(&adj);
        let center = tree_center(&adj);

        assert_eq!(diam, expected_diameter);
        assert_eq!(center, expected_center);
    }
}
