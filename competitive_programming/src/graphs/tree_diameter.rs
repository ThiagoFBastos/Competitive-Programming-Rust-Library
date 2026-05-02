use std::collections::VecDeque;

type Graph = Vec<Vec<usize>>;

fn bfs(adj: &Graph) -> Vec<usize> {
    let n = adj.len();

    let mut deg = vec![0; n];
    let mut dist = vec![0; n];
    let mut queue = VecDeque::new();

    for i in 0..n {
        deg[i] = adj[i].len();

        if deg[i] == 1 {
            queue.push_back(i);
        }
    }

    while !queue.is_empty() {
        let src = queue.pop_front().unwrap();

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
