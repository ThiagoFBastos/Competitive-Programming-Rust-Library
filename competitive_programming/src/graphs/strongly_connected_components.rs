use std::collections::VecDeque;

pub struct StronglyConnectedComponents {
    adj: Vec<Vec<usize>>,       // the adjacency list of vertices
    low: Vec<usize>,            // the lowest time of a reachable vertex
    time: Vec<Option<usize>>,   // the time of the vertex
    scc_id: Vec<Option<usize>>, // the id of the strongly connected component containing the vertex
    time_counter: usize,        // the time counter
    scc_id_counter: usize,      // the scc id counter
    stack: VecDeque<usize>,     // the stack of vertices
}

impl StronglyConnectedComponents {
    // Create a new instance of the StronglyConnectedComponents
    // param adj is the adjacency list of vertices
    pub fn new(adj: Vec<Vec<usize>>) -> Self {
        let n = adj.len();

        Self {
            adj,
            low: vec![0; n],
            time: vec![None; n],
            scc_id: vec![None; n],
            time_counter: 0,
            scc_id_counter: 0,
            stack: VecDeque::with_capacity(n),
        }
    }

    // Find the strongly connected components
    pub fn find_strongly_connected_components(&mut self) {
        let n = self.adj.len();

        for vertex in 0..n {
            if self.time[vertex].is_none() {
                self.dfs(vertex);
            }
        }
    }

    // Return the id of the strongly connected component containing the given vertex
    pub fn id(&self, vertex: usize) -> Option<usize> {
        self.scc_id[vertex]
    }

    fn dfs(&mut self, vertex: usize) {
        self.time[vertex] = Some(self.time_counter);
        self.low[vertex] = self.time_counter;
        self.time_counter += 1;
        self.stack.push_back(vertex);

        for u in self.adj[vertex].clone() {
            if let Some(t) = self.time[u] {
                if self.scc_id[u].is_none() {
                    self.low[vertex] = std::cmp::min(self.low[vertex], t);
                }
            } else {
                self.dfs(u);
                self.low[vertex] = std::cmp::min(self.low[vertex], self.low[u]);
            }
        }

        if self.time[vertex] != Some(self.low[vertex]) {
            return;
        }

        while let Some(u) = self.stack.pop_back() {
            self.scc_id[u] = Some(self.scc_id_counter);

            if u == vertex {
                break;
            }
        }

        self.scc_id_counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: returns the SCC grouping as a sorted Vec<Vec<usize>>,
    // so tests can compare groupings without caring about the exact id values.
    fn scc_groups(scc: &StronglyConnectedComponents, n: usize) -> Vec<Vec<usize>> {
        let mut groups: std::collections::HashMap<usize, Vec<usize>> =
            std::collections::HashMap::new();
        for v in 0..n {
            let id = scc
                .id(v)
                .expect("every vertex must have an scc id after running the algorithm");
            groups.entry(id).or_default().push(v);
        }
        let mut result: Vec<Vec<usize>> = groups.into_values().collect();
        for g in result.iter_mut() {
            g.sort();
        }
        result.sort();
        result
    }

    #[test]
    fn single_vertex_no_edges() {
        // one isolated vertex is its own SCC
        let adj = vec![vec![]];
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        assert_eq!(scc_groups(&scc, 1), vec![vec![0]]);
    }

    #[test]
    fn single_vertex_self_loop() {
        // a self-loop must not break the algorithm; still a single-vertex SCC
        let adj = vec![vec![0]];
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        assert_eq!(scc_groups(&scc, 1), vec![vec![0]]);
    }

    #[test]
    fn two_vertices_no_edges() {
        // no connectivity at all: each vertex is its own SCC
        let adj = vec![vec![], vec![]];
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        assert_eq!(scc_groups(&scc, 2), vec![vec![0], vec![1]]);
    }

    #[test]
    fn two_vertex_cycle() {
        // 0 -> 1 -> 0 forms a single SCC
        let adj = vec![vec![1], vec![0]];
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        assert_eq!(scc_groups(&scc, 2), vec![vec![0, 1]]);
    }

    #[test]
    fn linear_chain_no_back_edges() {
        // 0 -> 1 -> 2 is a DAG: each vertex is its own SCC
        let adj = vec![vec![1], vec![2], vec![]];
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        assert_eq!(scc_groups(&scc, 3), vec![vec![0], vec![1], vec![2]]);
    }

    #[test]
    fn triangle_cycle() {
        // 0 -> 1 -> 2 -> 0: all three vertices belong to one SCC
        let adj = vec![vec![1], vec![2], vec![0]];
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        assert_eq!(scc_groups(&scc, 3), vec![vec![0, 1, 2]]);
    }

    #[test]
    fn disconnected_graph_with_multiple_components() {
        // two separate cycles, {0,1} and {2,3}, with no edges between them
        let adj = vec![vec![1], vec![0], vec![3], vec![2]];
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        assert_eq!(scc_groups(&scc, 4), vec![vec![0, 1], vec![2, 3]]);
    }

    #[test]
    fn edge_into_already_finished_component() {
        // Regression test for the bug that was fixed: a vertex processed
        // AFTER an earlier SCC has already been closed (has an scc_id) must
        // not trigger another dfs call on that already-finished vertex.
        //
        // 0 <-> 1 form a finished SCC before vertex 2's dfs starts.
        // 2 -> 1 is an edge into that already-closed component and must be
        // ignored when computing low[2].
        let adj = vec![
            vec![1], // 0 -> 1
            vec![0], // 1 -> 0
            vec![1], // 2 -> 1 (edge into an already finished SCC)
        ];
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        let groups = scc_groups(&scc, 3);
        // {0,1} must remain a single SCC, and 2 must be its own separate SCC
        assert_eq!(groups, vec![vec![0, 1], vec![2]]);
    }

    #[test]
    fn classic_multi_scc_graph() {
        // classic textbook example with 8 vertices and 4 SCCs:
        // {0,1,2}, {3}, {4,5,6}, {7}
        let adj = vec![
            vec![1],    // 0 -> 1
            vec![2],    // 1 -> 2
            vec![0, 3], // 2 -> 0, 2 -> 3
            vec![4],    // 3 -> 4
            vec![5],    // 4 -> 5
            vec![6],    // 5 -> 6
            vec![4, 7], // 6 -> 4, 6 -> 7
            vec![],     // 7
        ];
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        assert_eq!(
            scc_groups(&scc, 8),
            vec![vec![0, 1, 2], vec![3], vec![4, 5, 6], vec![7]]
        );
    }

    #[test]
    fn complete_graph_is_one_scc() {
        // every vertex reaches every other vertex directly: a single SCC
        let n = 4;
        let mut adj = vec![vec![]; n];
        for (i, list) in adj.iter_mut().enumerate() {
            for j in 0..n {
                if i != j {
                    list.push(j);
                }
            }
        }
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        assert_eq!(scc_groups(&scc, n), vec![(0..n).collect::<Vec<_>>()]);
    }

    #[test]
    fn empty_graph() {
        // no vertices at all: should not panic and should produce nothing
        let adj: Vec<Vec<usize>> = vec![];
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        assert_eq!(scc_groups(&scc, 0), Vec::<Vec<usize>>::new());
    }

    #[test]
    fn two_cycles_connected_by_a_bridge() {
        // {0,1} -> {2,3}: a directed edge from one cycle into another.
        // Must stay as two distinct SCCs, not merge into one.
        let adj = vec![
            vec![1],    // 0 -> 1
            vec![0, 2], // 1 -> 0, 1 -> 2 (bridge into the other cycle)
            vec![3],    // 2 -> 3
            vec![2],    // 3 -> 2
        ];
        let mut scc = StronglyConnectedComponents::new(adj);
        scc.find_strongly_connected_components();
        assert_eq!(scc_groups(&scc, 4), vec![vec![0, 1], vec![2, 3]]);
    }
}
