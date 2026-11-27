/**
 * It implements the Floyd-Warshall algorithm
 * @param matrix the adjacency matrix
 */
pub fn floyd_warshall(matrix: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = matrix.len();

    assert!(n > 0, "The number of rows must be greater than zero");

    let m = matrix[0].len();

    assert_eq!(n, m, "The matrix must be a square matrix");

    const INF: i64 = 1_000_000_000_000_000_000;
    let mut dist = matrix.to_vec();

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if dist[i][k] != INF && dist[k][j] != INF {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    const INF: i64 = 1_000_000_000_000_000_000;

    #[test]
    fn single_vertex() {
        let graph = vec![vec![0]];

        assert_eq!(floyd_warshall(&graph), vec![vec![0]]);
    }

    #[test]
    fn no_edges() {
        let graph = vec![vec![0, INF, INF], vec![INF, 0, INF], vec![INF, INF, 0]];

        assert_eq!(floyd_warshall(&graph), graph);
    }

    #[test]
    fn direct_edges() {
        let graph = vec![vec![0, 5, INF], vec![INF, 0, 3], vec![INF, INF, 0]];

        let expected = vec![vec![0, 5, 8], vec![INF, 0, 3], vec![INF, INF, 0]];

        assert_eq!(floyd_warshall(&graph), expected);
    }

    #[test]
    fn path_through_multiple_vertices() {
        let graph = vec![
            vec![0, 1, INF, INF],
            vec![INF, 0, 2, INF],
            vec![INF, INF, 0, 3],
            vec![INF, INF, INF, 0],
        ];

        let expected = vec![
            vec![0, 1, 3, 6],
            vec![INF, 0, 2, 5],
            vec![INF, INF, 0, 3],
            vec![INF, INF, INF, 0],
        ];

        assert_eq!(floyd_warshall(&graph), expected);
    }

    #[test]
    fn chooses_shortest_path() {
        let graph = vec![vec![0, 10, 5], vec![INF, 0, 2], vec![INF, 1, 0]];

        let expected = vec![vec![0, 6, 5], vec![INF, 0, 2], vec![INF, 1, 0]];

        assert_eq!(floyd_warshall(&graph), expected);
    }

    #[test]
    fn directed_graph() {
        let graph = vec![vec![0, 4, INF], vec![INF, 0, 7], vec![2, INF, 0]];

        let expected = vec![vec![0, 4, 11], vec![9, 0, 7], vec![2, 6, 0]];

        assert_eq!(floyd_warshall(&graph), expected);
    }

    #[test]
    fn zero_weight_edges() {
        let graph = vec![vec![0, 0, INF], vec![INF, 0, 5], vec![INF, INF, 0]];

        let expected = vec![vec![0, 0, 5], vec![INF, 0, 5], vec![INF, INF, 0]];

        assert_eq!(floyd_warshall(&graph), expected);
    }

    #[test]
    fn cycle() {
        let graph = vec![vec![0, 2, INF], vec![INF, 0, 3], vec![4, INF, 0]];

        let expected = vec![vec![0, 2, 5], vec![7, 0, 3], vec![4, 6, 0]];

        assert_eq!(floyd_warshall(&graph), expected);
    }

    #[test]
    fn unreachable_vertices_remain_unreachable() {
        let graph = vec![
            vec![0, 2, INF, INF],
            vec![INF, 0, INF, INF],
            vec![INF, INF, 0, 3],
            vec![INF, INF, INF, 0],
        ];

        let expected = vec![
            vec![0, 2, INF, INF],
            vec![INF, 0, INF, INF],
            vec![INF, INF, 0, 3],
            vec![INF, INF, INF, 0],
        ];

        assert_eq!(floyd_warshall(&graph), expected);
    }

    #[test]
    #[should_panic(expected = "The number of rows must be greater than zero")]
    fn empty_matrix_panics() {
        floyd_warshall(&[]);
    }

    #[test]
    #[should_panic(expected = "The matrix must be a square matrix")]
    fn non_square_matrix_panics() {
        let graph = vec![vec![0, 1], vec![1, 0], vec![1, 1]];

        floyd_warshall(&graph);
    }
}
