use std::collections::VecDeque;

type Graph = Vec<Vec<usize>>;

fn bfs(adj: &Graph) -> Vec<usize> {
    let n = adj.len();

    let mut deg = (0..n).map(|v| adj[v].len()).collect::<Vec<_>>();
    let mut dist = vec![0; n];
    let mut queue = (0..n).filter(|&v| deg[v] == 1).collect::<VecDeque<_>>();

    while let Some(src) = queue.pop_front() {
        for &dest in adj[src].iter() {
            deg[dest] -= 1;

            if deg[dest] == 1 {
                dist[dest] = 1 + dist[src];
                queue.push_back(dest);
            }
        }
    }

    dist
}

/**
 * Find the diameter of a given tree
 * @param adj the adjacency list of the tree
 * @return the diameter
 */
pub fn tree_diameter(adj: &Graph) -> usize {
    assert_ne!(adj.len(), 0);

    let dist = bfs(adj);

    let e = *dist.iter().max().unwrap();
    let count = dist.iter().filter(|&&x| x == e).count();

    2 * e + count - 1
}

/**
 * Find the center of a given tree
 * @param adj the adjacency list of the tree
 * @return the center
 */
pub fn tree_center(adj: &Graph) -> Vec<usize> {
    assert_ne!(adj.len(), 0);

    let dist = bfs(adj);

    let e = *dist.iter().max().unwrap();

    dist.iter()
        .enumerate()
        .filter(|&x| *x.1 == e)
        .map(|x| x.0)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_edge(adj: &mut Graph, u: usize, v: usize) {
        adj[u].push(v);
        adj[v].push(u);
    }

    #[test]
    fn test_small_tree() {
        const N: usize = 5;

        let expected_diameter = 3;
        let expected_center = vec![0, 2];

        let mut adj: Graph = vec![vec![]; N];

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

        let mut adj: Graph = vec![vec![]; N];

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

        let adj: Graph = vec![vec![]; N];

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

        let mut adj: Graph = vec![vec![]; N];

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
