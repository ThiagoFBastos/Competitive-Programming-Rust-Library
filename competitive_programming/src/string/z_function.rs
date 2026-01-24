/*
 * Author: Thiago Felipe Bastos da Silva
 * Created: 2026-01-24
 * Description: Implementation of Z Function algorithm.
 */

/**
 * Given a string calculates its z-function
 * @param s the target string
 * @return a vector with the results of z-function
 */
pub fn z_function(s: &String) -> Vec<usize> {
   let n = s.len();
   let str = s.chars().collect::<Vec<_>>();
   let mut z = vec![0; n];

   let mut l = 0;
   let mut r = 0;

   for i in 1..n {
      if i <= r {
         z[i] = z[i - l].min(r - i + 1);
      }

      while i + z[i] < n && str[z[i]] == str[i + z[i]] {
         z[i] += 1;
      }

      if i + z[i] - 1 > r {
         l = i;
         r = i + z[i] - 1;
      }
   }

   return z;
}