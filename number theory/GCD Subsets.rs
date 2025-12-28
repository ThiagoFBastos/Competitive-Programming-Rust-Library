/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-28
 * Description: Given an array with numbers not greater than n, the below algorithm
 * find the count of non-empty subsets whose gcd is equal to k for each k = 1, ..., n.
 */

 /**
  * find de count of subsets that are equal to k = 1, ..., n
  * @param values the array with numbers, being that 1 <= values_i <= n
  */
fn count_gcd_subsets<const MOD: i64>(values: &[i32]) -> Vec<i64> {
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
      for j in (2*i..=n).step_by(i) {
         dp[i] -= dp[j];
         if dp[i] < 0 {
            dp[i] += MOD;
         }
      }
   }

   return dp;
}
