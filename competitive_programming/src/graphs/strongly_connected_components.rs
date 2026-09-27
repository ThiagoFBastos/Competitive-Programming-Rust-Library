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
