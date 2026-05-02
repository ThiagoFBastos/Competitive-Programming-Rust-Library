/**
 * There is a list of n numbers and two players who move alternately.
 * On each move, a player removes either the first or last number from the list,
 * and their score increases by that number. Both players try to maximize their scores.
 * What is the maximum possible score for the first player when both players play optimally?
 * n <= 5000
 * -10^9 <= nums[i] <= 10^9
 * @param nums The list of numbers.
 * @return The maximum possible score for the first player.
 */
pub fn removal_game(nums: &[i64]) -> i64 {
    const INF: i64 = 0x1000000000000;

    let n = nums.len();
    let mut turn = !n & 1;
    let sum = nums.iter().sum::<i64>();

    let mut dp = vec![[0, 0]; n];

    for i in 0..n {
        dp[i][turn] = nums[i];
    }

    for len in (1..n).rev() {
        turn ^= 1;

        let my = turn;
        let your = turn ^ 1;

        for item in dp.iter_mut().take(len) {
            item[my] = -INF;
        }

        for i in 0..len {
            dp[i][my] = dp[i][my].max(nums[i] - dp[i + 1][your]);
        }

        for i in 0..len {
            dp[i][my] = dp[i][my].max(nums[i + n - len] - dp[i][your]);
        }
    }

    (sum + dp[0][0]) / 2
}
