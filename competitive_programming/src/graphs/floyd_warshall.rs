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
