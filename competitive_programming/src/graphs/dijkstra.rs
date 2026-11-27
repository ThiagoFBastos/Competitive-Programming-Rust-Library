use std::{cmp::Reverse, collections::BinaryHeap};

type Graph = Vec<Vec<(usize, i64)>>;

pub fn dijkstra(adj: &Graph, src: usize) -> Vec<Option<i64>> {
    let n = adj.len();
    let mut dist = vec![None; n];
    let mut pq = BinaryHeap::new();

    assert_ne!(n, 0);

    pq.push(Reverse((0, src)));

    while let Some(Reverse((cost, u))) = pq.pop() {
        if dist[u].is_some() {
            continue;
        }

        dist[u] = Some(cost);

        for &(v, w) in adj[u].iter() {
            if dist[v].is_some() {
                continue;
            }

            pq.push(Reverse((w + cost, v)));
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    fn add_edge(adj: &mut Graph, u: usize, v: usize, w: i64) {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    #[test]
    fn multiple_simple_path_tree_test() {
        const N: usize = 8;
        let mut adj: Graph = vec![vec![]; N];

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
        let mut adj: Graph = vec![vec![]; N];

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
