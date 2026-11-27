/**
 * find de count of subsets that are equal to k = 1, ..., n
 * @param values the array with numbers, being that 1 <= values_i <= n
 */
pub fn count_gcd_subsets<const MOD: i64>(values: &[i32]) -> Vec<i64> {
    let n = values.len();

    let mut dp = vec![0; n + 1];
    let mut pow = vec![0; n + 1];
    let mut frequency = vec![0; n + 1];

    assert!(*values.iter().min().unwrap() >= 0);

    for value in values.iter() {
        frequency[*value as usize] += 1;
    }

    pow[0] = 1;
    for i in 1..=n {
        pow[i] = pow[i - 1] << 1;
        if pow[i] >= MOD {
            pow[i] -= MOD;
        }
    }

    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            dp[i] += frequency[j] as i64;
        }
        dp[i] = (pow[dp[i] as usize] - 1 + MOD) % MOD;
    }

    for i in (1..=n).rev() {
        for j in (2 * i..=n).step_by(i) {
            dp[i] -= dp[j];
            if dp[i] < 0 {
                dp[i] += MOD;
            }
        }
    }

    dp.remove(0);

    dp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        const MOD: i64 = 1_000_000_007;

        let values = vec![5, 4, 4, 2, 3];
        let expected = vec![22, 4, 1, 3, 1];

        let result = count_gcd_subsets::<MOD>(&values);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        const MOD: i64 = 1_000_000_007;

        let values = vec![8, 8, 1, 1, 1, 2, 7, 7, 8, 4];
        let expected = vec![989, 16, 0, 8, 0, 0, 3, 7, 0, 0];

        let result = count_gcd_subsets::<MOD>(&values);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_3() {
        const MOD: i64 = 1_000_000_007;

        let values = vec![1, 1, 1];
        let expected = vec![7, 0, 0];

        let result = count_gcd_subsets::<MOD>(&values);

        assert_eq!(result, expected);
    }
}
