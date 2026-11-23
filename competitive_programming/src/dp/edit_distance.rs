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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let s = String::from("LOVE");
        let t = String::from("MOVIE");

        let expected = 2_usize;

        let edit = edit_distance(&s, &t);

        assert_eq!(edit, expected);
    }

    #[test]
    fn without_operations_test() {
        let s = String::from("A");
        let t = String::from("A");

        let expected = 0_usize;

        let edit = edit_distance(&s, &t);

        assert_eq!(edit, expected);
    }

    #[test]
    fn only_one_operation_test() {
        let s = String::from("A");
        let t = String::from("B");

        let expected = 1_usize;

        let edit = edit_distance(&s, &t);

        assert_eq!(edit, expected);
    }

    #[test]
    fn many_operations_test() {
        let s = String::from("TWXFUABGBNLTBFNSUVQW");
        let t = String::from("GPNJILFXJUIZPLTVUIB");

        let expected = 19_usize;

        let edit = edit_distance(&s, &t);

        assert_eq!(edit, expected);
    }

    #[test]
    fn many_replacements_test() {
        const LENGTH: usize = 300;

        let s = (0..LENGTH).map(|_| 'A').collect::<String>();
        let t = (0..LENGTH).map(|_| 'B').collect::<String>();

        let expected = LENGTH;

        let edit = edit_distance(&s, &t);

        assert_eq!(edit, expected);
    }
}
