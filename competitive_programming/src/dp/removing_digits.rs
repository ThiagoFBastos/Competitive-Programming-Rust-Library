/**
 * Given an integer n, return the minimum number of steps to reduce it to zero.
 * In one step, you can remove any digit from n and subtract it from n.
 */
pub fn removing_digits(n: u32) -> u32 {
    let m = n as usize;
    let mut dp = vec![0; m + 1];

    for k in 1..=m {
        let mut digit = 0;
        let mut target = k;

        while target > 0 {
            digit = digit.max(target % 10);
            target /= 10;
        }

        dp[k] = 1 + dp[k - digit];
    }

    dp[m]
}
