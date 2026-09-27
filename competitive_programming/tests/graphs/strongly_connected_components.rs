// Test suite for StronglyConnectedComponents (Tarjan's algorithm)
// Append this `mod tests` block to the bottom of the file that defines
// `StronglyConnectedComponents`, or `include!` it there.

#[cfg(test)]
mod strongly_connected_components_tests {
    use competitive_programming::graphs::StronglyConnectedComponents;

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
