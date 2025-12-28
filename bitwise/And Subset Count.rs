/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2025-12-28
 * Description: You are given an array of n integers. 
 * Your task is to calculate the number of non-empty subsets whose elements' 
 * bitwise and is equal to k for each k = 0, 1,\dots, n.
 */

/**
 * count the number of subsets with a xor value that is less than n + 1
 * @param nums the array of elements to calculate the number of subsets with a xor value
 * @return a 0-indexed array with the count of subsets that xor value is equal to position i
 */
fn count_subset_and<const N: usize, const MOD: i64>(nums: &[i32]) -> Vec<i64> { 
   let n = nums.len();
 
   let mut dp = vec![0; 1 << N];
   let mut pow = vec![0; 1 << N];
 
   for value in nums.iter() {
      dp[*value as usize] += 1;
   }
 
   pow[0] = 1;
   for i in 1..=n {
      pow[i] = pow[i - 1] << 1;
      if pow[i] >= MOD {
         pow[i] -= MOD;
      }
   }
 
   for i in 0..N {
      for j in (0..=n).rev() {
         if (!j >> i) & 1 == 1 {
            dp[j] += dp[j ^ (1 << i)];
         }
      }
   }
 
   for i in 0..=n {
      dp[i] = (pow[dp[i] as usize] - 1) % MOD;
 
      if i.count_ones() % 2 == 1 {
         dp[i] = (MOD - dp[i]) % MOD;
      } 
   }
 
   for i in 0..N {
      for j in (0..=n).rev() {
         if (!j >> i) & 1 == 1 {
            dp[j] += dp[j ^ (1 << i)];
 
            if dp[j] >= MOD {
               dp[j] -= MOD;
            }
         }
      }
   }
 
   let mut answer = vec![0; n + 1];

   for i in 0..=n {
      answer[i] = dp[i];
 
      if i.count_ones() % 2 == 1 {
         answer[i] = (MOD - answer[i]) % MOD;
      } 
   }

   answer
}
