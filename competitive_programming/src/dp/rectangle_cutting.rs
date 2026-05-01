/**
 * @brief Given an w x h rectangle, your task is to cut it into squares.
 * On each move you can select a rectangle and cut it into two rectangles
 * in such a way that all side lengths remain integers.
 *  What is the minimum possible number of moves?
 */
pub fn rectangle_cutting(w: u32, h: u32) -> u32 {
    const INF: u32 = 1_000_000_000;

    let width = w as usize;
    let height = h as usize;
    let diagonal = width.min(height);

    let mut dp = vec![vec![INF; height + 1]; width + 1];

    for (i, row) in dp.iter_mut().enumerate().take(diagonal + 1) {
        row[i] = 0
    }

    for k in 1..=width {
        for j in 1..=height {
            for i in 1..=k {
                dp[k][j] = dp[k][j].min(1 + dp[k - i][j] + dp[i][j]);
            }
            for i in 1..=j {
                dp[k][j] = dp[k][j].min(1 + dp[k][j - i] + dp[k][i]);
            }
        }
    }

    dp[width][height]
}
