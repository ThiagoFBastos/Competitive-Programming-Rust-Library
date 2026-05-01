/**
 * @brief Calculate the edit distance between two strings
 */
pub fn edit_distance(s: &str, t: &str) -> usize {
    const INF: usize = 1_000_000_000;

    let n = s.len();
    let m = t.len();

    let mut s_chars = s.chars().collect::<Vec<_>>();
    let mut t_chars = t.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![INF; m + 2]; n + 2];

    s_chars.push('\0');
    t_chars.push('\0');

    dp[0][0] = 0;

    for (i, ch_s) in s_chars.iter().enumerate().take(n + 1) {
        for (j, ch_t) in t_chars.iter().enumerate().take(m + 1) {
            let mut value = dp[i][j];

            if *ch_s != *ch_t {
                value += 1;
            }

            dp[i + 1][j + 1] = dp[i + 1][j + 1].min(value);
            dp[i + 1][j] = dp[i + 1][j].min(1 + dp[i][j]);
            dp[i][j + 1] = dp[i][j + 1].min(1 + dp[i][j]);
        }
    }

    dp[n][m]
}
