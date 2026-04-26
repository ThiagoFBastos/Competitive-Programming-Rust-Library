use std::{cmp::Reverse, collections::BinaryHeap};

type Graph = Vec<Vec<(usize, i64)>>;

pub fn dijkstra(adj: &Graph, src: usize) -> Vec<Option<i64>> {
    let n = adj.len();
    let mut dist = vec![None; n];
    let mut pq = BinaryHeap::new();

    assert_ne!(n, 0);

    pq.push(Reverse((0, src)));

    while !pq.is_empty() {
        let (cost, u) = pq.pop().unwrap().0;

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
